// Copyright 2021 The BMW Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(target_os = "windows")]
use std::os::windows::io::AsRawSocket;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

use clap::load_yaml;
use clap::App;
use librustlet::*;
use native_tls::TlsConnector;
use nioruntime_log::*;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

const MAX_BUF: usize = 100_000;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

nioruntime_log::info!();

fn fun() -> Result<(), Error> {
	rustlet!("error", {
		response!("<html><body>test of error");
		if true {
			return Err(ErrorKind::InternalError("test error".to_string()).into());
		}
	});
	Ok(())
}

fn fun2() -> Result<(), Error> {
	rustlet!("panic", {
		response!("<html><body>test of panic");
		flush!();
		let x: Option<bool> = None;
		x.unwrap();
	});
	Ok(())
}

// include build information
pub mod built_info {
	include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn client_thread(
	count: usize,
	id: usize,
	tlat_sum: Arc<Mutex<f64>>,
	tlat_max: Arc<Mutex<u128>>,
	nginx: bool,
	mio: bool,
	tls: bool,
	connector: &TlsConnector,
) -> Result<(), Error> {
	let mut lat_sum = 0.0;
	let mut lat_max = 0;
	let addr = if nginx {
		"127.0.0.1:80"
	} else {
		"127.0.0.1:8080"
	};
	let (mut stream, mut tls_stream, fd) = if tls {
		let _lock = tlat_sum.lock();
		let tls_stream = connector
			.connect("example.com", TcpStream::connect(addr)?)
			.unwrap();
		#[cfg(unix)]
		let fd = tls_stream.get_ref().as_raw_fd();
		#[cfg(target_os = "windows")]
		let fd = tls_stream.get_ref().as_raw_socket();
		(None, Some(tls_stream), fd)
	} else {
		let _lock = tlat_sum.lock();
		let stream = TcpStream::connect(addr)?;
		#[cfg(unix)]
		let fd = stream.as_raw_fd();
		#[cfg(target_os = "windows")]
		let fd = stream.as_raw_socket();
		(Some(stream), None, fd)
	};
	let buf2 = &mut [0u8; MAX_BUF];
	let start_itt = std::time::SystemTime::now();
	let uri = if nginx { "/" } else { "/empty" };
	let request_string = if nginx {
		format!(
			"GET {} HTTP/1.1\r\nHost: localhost:80\r\nConnection: keep-alive\r\n\r\n",
			uri
		)
	} else {
		format!(
			"GET {} HTTP/1.1\r\nCookie: rustletsessionid=179034848365240461385689167936203491398;\r\nConnection: keep-alive\r\n\r\n",
			uri
		)
	};
	let request_bytes = request_string.as_bytes();
	for i in 0..count {
		if i != 0 && i % 10000 == 0 {
			let elapsed = start_itt.elapsed().unwrap().as_millis();
			let qps = (i as f64 / elapsed as f64) * 1000 as f64;
			info!("Request {} on thread {}, qps={}", i, id, qps);
		}
		let start_query = std::time::SystemTime::now();
		let res = match stream {
			Some(ref mut stream) => stream.write(&request_bytes),
			None => {
				let stream = tls_stream.as_mut().unwrap();
				stream.write(&request_bytes)
			}
		};

		match res {
			Ok(_x) => {}
			Err(e) => {
				info!("Write Error: {}", e.to_string());
				std::thread::sleep(std::time::Duration::from_millis(1));
			}
		}

		let mut len_sum = 0;
		loop {
			let res = match stream {
				Some(ref mut stream) => stream.read(&mut buf2[len_sum..]),
				None => {
					let stream = tls_stream.as_mut().unwrap();
					stream.read(&mut buf2[len_sum..])
				}
			};

			match res {
				Ok(_) => {}
				Err(ref e) => {
					info!("Read Error: {}, fd = {}", e.to_string(), fd);
					assert!(false);
				}
			}

			let len = res.unwrap();

			len_sum += len;
			if nginx {
				if len_sum >= 5
					&& buf2[len_sum - 1] == 10
					&& buf2[len_sum - 2] == 62
					&& buf2[len_sum - 3] == 108
					&& buf2[len_sum - 4] == 109
					&& buf2[len_sum - 5] == 116
				{
					break;
				}
			} else if mio {
				if len_sum >= 5
					&& buf2[len_sum - 1] == 10
					&& buf2[len_sum - 2] == 13
					&& buf2[len_sum - 3] == 114
					&& buf2[len_sum - 4] == 101
				{
					break;
				}
			} else {
				if len_sum >= 5
					&& buf2[len_sum - 1] == '\n' as u8
					&& buf2[len_sum - 2] == '\r' as u8
					&& buf2[len_sum - 3] == '\n' as u8
					&& buf2[len_sum - 4] == '\r' as u8
					&& buf2[len_sum - 5] == '0' as u8
				{
					break;
				}
			}
		}

		let elapsed = start_query.elapsed().unwrap().as_nanos();
		lat_sum += elapsed as f64;
		if elapsed > lat_max {
			lat_max = elapsed;
		}

		// clear buf2
		for i in 0..len_sum {
			buf2[i] = 0;
		}
	}

	{
		let mut tlat_sum = tlat_sum.lock().unwrap();
		(*tlat_sum) += lat_sum;
	}
	{
		let mut tlat_max = tlat_max.lock().unwrap();
		if lat_max > *tlat_max {
			(*tlat_max) = lat_max;
		}
	}

	Ok(())
}

#[derive(Debug)]
struct Example {
	num: u32,
}

impl Example {
	pub fn new(num: u32) -> Self {
		Example { num }
	}
}

impl Writeable for Example {
	fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
		writer.write_u32(self.num)?;

		Ok(())
	}
}

impl Readable for Example {
	fn read<R: Reader>(reader: &mut R) -> Result<Self, Error> {
		let num = reader.read_u32()?;
		Ok(Example { num })
	}
}

#[derive(Debug)]
struct Example2 {
	num: u64,
}

impl Example2 {
	pub fn _new(num: u64) -> Self {
		Example2 { num }
	}
}

impl Writeable for Example2 {
	fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
		writer.write_u64(self.num)?;

		Ok(())
	}
}

impl Readable for Example2 {
	fn read<R: Reader>(reader: &mut R) -> Result<Self, Error> {
		let num = reader.read_u64()?;
		Ok(Example2 { num })
	}
}

fn main() {
	match real_main() {
		Ok(_) => {}
		Err(e) => {
			fatal!("Real main generated error: {}", e);
		}
	}
}

fn real_main() -> Result<(), Error> {
	let yml = load_yaml!("rustlet.yml");
	let args = App::from_yaml(yml)
		.version(built_info::PKG_VERSION)
		.get_matches();

	let client = args.is_present("client");
	let mio = args.is_present("mio");
	let nginx = args.is_present("nginx");
	let debug = args.is_present("debug");
	let delete_request_rotation = args.is_present("delete_request_rotation");

	let certs = args.is_present("certs");
	let private_key = args.is_present("private_key");

	if certs && !private_key || !certs && private_key {
		error!("Either both cert and private_key or neither must be specified");
		return Ok(());
	}

	let tls_config = match args.value_of("certs") {
		Some(certificates_file) => {
			let private_key_file = args.value_of("private_key").unwrap_or("").to_string();
			let certificates_file = certificates_file.to_string();
			Some(TlsConfig {
				certificates_file,
				private_key_file,
			})
		}
		None => None,
	};

	if client {
		let threads = args.is_present("threads");
		let count = args.is_present("count");
		let itt = args.is_present("itt");
		let tls = args.is_present("tls");

		let threads = match threads {
			true => args.value_of("threads").unwrap().parse().unwrap(),
			false => 1,
		};

		let count = match count {
			true => args.value_of("count").unwrap().parse().unwrap(),
			false => 1,
		};

		let itt = match itt {
			true => args.value_of("itt").unwrap().parse().unwrap(),
			false => 1,
		};

		info!(
			"Running client {}",
			if nginx { "against nginx" } else { "" }
		);
		info!("Threads={}", threads);
		info!("Iterations={}", itt);
		info!("Requests per thread per iteration={}", count);
		info_no_ts!(
			"--------------------------------------------------------------------------------"
		);

		let time = std::time::SystemTime::now();
		let tlat_sum = Arc::new(Mutex::new(0.0));
		let tlat_max = Arc::new(Mutex::new(0));
		let connector = TlsConnector::builder()
			.danger_accept_invalid_hostnames(true)
			.build()
			.unwrap();

		for x in 0..itt {
			let mut jhs = vec![];
			for i in 0..threads {
				let id = i.clone();
				let tlat_sum = tlat_sum.clone();
				let tlat_max = tlat_max.clone();
				let connector = connector.clone();
				jhs.push(std::thread::spawn(move || {
					let res = client_thread(
						count,
						id,
						tlat_sum.clone(),
						tlat_max.clone(),
						nginx,
						mio,
						tls,
						&connector,
					);
					match res {
						Ok(_) => {}
						Err(e) => {
							info!("Error in client thread: {}", e.to_string());
							assert!(false);
						}
					}
				}));
			}

			for jh in jhs {
				jh.join().expect("panic in thread");
			}
			info!("Iteration {} complete. ", x + 1);
		}

		let elapsed_millis = time.elapsed().unwrap().as_millis();
		let lat_max = tlat_max.lock().unwrap();
		info_no_ts!(
			"--------------------------------------------------------------------------------"
		);
		info!("Test complete in {} ms", elapsed_millis);
		let tlat = tlat_sum.lock().unwrap();
		let avg_lat = (*tlat) / (1_000_000 * count * threads * itt) as f64;
		//let qps_simple = (1000.0 / avg_lat) * threads as f64;
		let qps = (threads * count * itt * 1000) as f64 / elapsed_millis as f64;
		info!("QPS={}", qps);
		info!("Average latency={}ms", avg_lat,);
		info!("Max latency={}ms", (*lat_max) as f64 / (1_000_000 as f64));
	} else {
		rustlet_init!(RustletConfig {
			session_timeout: 60,
			http_config: HttpConfig {
				evh_config: EventHandlerConfig {
					thread_count: 8,
					tls_config,
				},
				max_log_queue: 100_000,
				stats_frequency: 10_000,
				delete_request_rotation,
				debug,
				server_name: format!("Rustlet Httpd {}", VERSION),
				..Default::default()
			},
		});

		rustlet!("empty", {});

		rustlet!("get_session", {
			let value: Option<Example> = session!("abc");
			match value {
				Some(value) => {
					response!("abc={:?}", value);
				}
				None => {
					response!("none");
				}
			}
		});

		rustlet!("set_session", {
			match query!("abc") {
				Some(abc) => session!("abc", Example::new(abc.parse().unwrap_or(0))),
				None => {}
			}
		});

		rustlet!("delete_session", {
			session_delete!();
		});

		rustlet!("delete_abc", {
			session_delete!("abc");
		});

		rustlet!("cookies", {
			let cookie = cookie!("abc");
			set_cookie!("abc", "def");
			response!("cookie={:?}\n", cookie);
		});

		rustlet!("async", {
			let ac = async_context!();
			response!("<html><body>Content: <span id='abc'>first message</span>\n");
			flush!();

			std::thread::spawn(move || {
				async_context!(ac);
				std::thread::sleep(std::time::Duration::from_millis(1000));
				response!("<script>document.getElementById('abc').innerHTML = 'second message';</script>\n");
				flush!();
				std::thread::sleep(std::time::Duration::from_millis(1000));
				response!("<script>document.getElementById('abc').innerHTML = 'third message';</script>\n");
				flush!();
				std::thread::sleep(std::time::Duration::from_millis(1000));
				response!("<script>document.getElementById('abc').innerHTML = 'fourth message';</script>\n");
				flush!();
				std::thread::sleep(std::time::Duration::from_millis(1000));
				response!("<script>document.getElementById('abc').innerHTML = 'fifth message';</script>\n");
				flush!();
				response!("</body></html>");
				async_complete!();
			});
		});

		rustlet!("redir", {
			set_redirect!("http://www.disney.com");
		});

		let x = Arc::new(Mutex::new(0));
		let x_clone = x.clone();

		rustlet!("myrustlet", {
			let name = query!("name");
			let mut x = x.lock().unwrap();
			*x += 1;
			add_header!("my_header", "ok");
			set_content_type!("text/plain");
			response!("name: {:?}, x={}", name, x);
		});

		rustlet!("myrustlet2", {
			let query = request!("query");
			let ua = request!("header", "User-Agent");
			let mut x = x_clone.lock().unwrap();
			*x += 1;
			response!("ok\n");
			response!("q2: {} x={}, ua={}", query, x, ua);
		});

		rustlet!("printheaders", {
			for i in 0..header_len!() {
				let header_name = header_name!(i);
				let header_value = header_value!(i);
				response!("header[{}] [{}] -> [{}]\n", i, header_name, header_value);
				flush!();
			}
			let method = request!("method");
			response!("method='{}'\n", method);
			let version = request!("version");
			response!("http version='{}'\n", version);
			let uri = request!("uri");
			response!("uri='{}'\n", uri);
			let unknown = request!("blah");
			response!("blah (should be empty)='{}'\n", unknown);
			let query = request!("query");
			response!("query='{}'\n", query);
			let content = request_content!();
			response!("content={:?}\n", content);
		});

		rustlet!("content", {
			let content = request_content!();
			let content_as_ut8 = std::str::from_utf8(&content)?;
			response!("content='{}'\n", content_as_ut8);
		});

		rustlet!("bin_write", {
			bin_write!("test of bin write".as_bytes());
		});

		rustlet!("formupload", {
			for i in 0..header_len!() {
				let header_name = header_name!(i);
				let header_value = header_value!(i);
				response!("header[{}] [{}] -> [{}]\n", i, header_name, header_value);
				flush!();
			}
			let content = request_content!();
			let content = &mut &content[..];
			let mut headers = hyper::header::Headers::new();
			for i in 0..header_len!() {
				headers.append_raw(header_name!(i), header_value!(i).as_bytes().to_vec());
			}
			let res =
				mime_multipart::read_multipart_body(content, &headers, false).unwrap_or(vec![]);
			response!("res={:?}", res);
		});

		let _ = fun();
		let _ = fun2();

		rustlet_mapping!("/bin_write", "bin_write");
		rustlet_mapping!("/myrustlet", "myrustlet");
		rustlet_mapping!("/myrustlet2", "myrustlet2");
		rustlet_mapping!("/printheaders", "printheaders");
		rustlet_mapping!("/redir", "redir");
		rustlet_mapping!("/error", "error");
		rustlet_mapping!("/panic", "panic");
		rustlet_mapping!("/async", "async");
		rustlet_mapping!("/cookies", "cookies");
		rustlet_mapping!("/empty", "empty");
		rustlet_mapping!("/set_session", "set_session");
		rustlet_mapping!("/get_session", "get_session");
		rustlet_mapping!("/delete_session", "delete_session");
		rustlet_mapping!("/delete_abc", "delete_abc");
		rustlet_mapping!("/content", "content");
		rustlet_mapping!("/formupload", "formupload");

		std::thread::park();
	}
	Ok(())
}
