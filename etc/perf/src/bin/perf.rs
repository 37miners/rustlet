// Copyright 2021 The Grin Developers
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

//! Main for perf test

use clap::load_yaml;
use clap::App;
use librustlet::nioruntime_log;
use librustlet::*;
use nioruntime_log::*;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

const CLIENT_WS_HANDSHAKE: &[u8] =
	"GET /perfsocklet HTTP/1.1\r\nUpgrade: websocket\r\nSec-WebSocket-Key: x\r\n\r\n".as_bytes();
const SIMPLE_WS_MESSAGE: &[u8] = &[130, 1, 1]; // binary data with a single byte of data, unmasked

debug!();

fn client_thread(count: usize, id: usize, port: u16) -> Result<(), Error> {
	let mut buf1 = [0u8; 150].to_vec();
	let mut buf2 = [0u8; 150].to_vec();

	let addr = format!("127.0.0.1:{}", port);
	let mut stream = TcpStream::connect(addr)?;

	// do websocket handshake
	stream.write(CLIENT_WS_HANDSHAKE)?;
	let mut offset = 0;
	loop {
		let mut found = false;
		let len = stream.read(&mut buf2)?;
		let _ = &buf1[offset..(offset + len)].clone_from_slice(&buf2[0..len]);
		for i in 3..len + offset {
			if buf1[i] == '\n' as u8
				&& buf1[i - 1] == '\r' as u8
				&& buf1[i - 2] == '\n' as u8
				&& buf1[i - 3] == '\r' as u8
			{
				// handshake completed.
				found = true;
				break;
			}
		}

		if found {
			break;
		}
		offset += len;
	}
	// handshake complete

	for i in 0..count {
		stream.write(SIMPLE_WS_MESSAGE)?;
		let mut total_len = 0;
		loop {
			let len = stream.read(&mut buf1)?;
			if total_len + len > 3 {
				error!("too much data read back!");
				assert!(false);
			} else if total_len == 0 {
				if buf1[0] != 130 || buf1[1] != 1 || buf1[2] != 1 {
					error!("invalid byte");
					assert!(false);
				}
				break;
			} else if total_len == 1 {
				if buf1[0] != 1 || buf1[1] != 1 {
					error!("invalid byte");
					assert!(false);
				}
			} else if total_len == 2 {
				if buf1[0] != 1 {
					error!("invalid byte");
					assert!(false);
				}
			} else {
				error!("should have exited loop");
				assert!(false);
			}
			total_len += len;
			if total_len >= 3 {
				break;
			}
		} // we need to read 3 bytes back as the reply.
	}

	Ok(())
}

fn main() -> Result<(), Error> {
	info!("Starting Perf test!");

	let yml = load_yaml!("perf.yml");
	let args = App::from_yaml(yml).version("1").get_matches();

	let threads = args.is_present("threads");
	let count = args.is_present("count");
	let itt = args.is_present("itt");
	let port = args.is_present("port");

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

	let port = match port {
		true => args.value_of("port").unwrap().parse().unwrap(),
		false => 8080,
	};

	for x in 0..itt {
		let mut jhs = vec![];
		for i in 0..threads {
			let id = i.clone();
			jhs.push(std::thread::spawn(move || {
				let res = client_thread(count, id, port);
				match res {
					Ok(_) => {}
					Err(e) => {
						error!("Error in client thread: {}", e.to_string());
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

	Ok(())
}
