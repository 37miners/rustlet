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
use std::convert::TryInto;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

const CLIENT_WS_HANDSHAKE: &[u8] =
	"GET /perfsocklet HTTP/1.1\r\nUpgrade: websocket\r\nSec-WebSocket-Key: x\r\n\r\n".as_bytes();
const SIMPLE_WS_MESSAGE: &[u8] = &[130, 1, 1]; // binary data with a single byte of data, unmasked
const SEPARATOR: &str =
	"------------------------------------------------------------------------------------------";
//123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
//         1         2         3         4         5         6         7         8         9

debug!();

fn client_thread(count: usize, id: usize, port: u16) -> Result<u128, Error> {
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

	let mut total_nanos = 0;

	for i in 0..count {
		let start_time = std::time::SystemTime::now();
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
		let elapsed = std::time::SystemTime::now().duration_since(start_time)?;
		total_nanos += elapsed.as_nanos();
	}

	Ok(total_nanos)
}

fn main() -> Result<(), Error> {
	let time_start = std::time::SystemTime::now();
	info!("Starting Perf test!");
	info_no_ts!("{}", SEPARATOR);

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

	let nano_sum: Arc<RwLock<u128>> = Arc::new(RwLock::new(0));
	for x in 0..itt {
		let mut jhs = vec![];
		for i in 0..threads {
			let id = i.clone();
			let nano_sum = nano_sum.clone();
			jhs.push(std::thread::spawn(move || {
				let res = client_thread(count, id, port);
				let total_nanos = match res {
					Ok(total_nanos) => total_nanos,
					Err(e) => {
						error!("Error in client thread: {}", e.to_string());
						assert!(false);
						0
					}
				};

				let mut nano_sum = nano_sum.write().unwrap();
				*nano_sum += total_nanos;
			}));
		}

		for jh in jhs {
			jh.join().expect("panic in thread");
		}
		info!("Iteration {} complete. ", x + 1);
	}

	let nano_sum = nano_sum.read().unwrap();
	let duration = std::time::SystemTime::now().duration_since(time_start)?;
	let mut total_requests: u64 = (threads * count * itt).try_into().unwrap_or(0);
	let avg_lat = (*nano_sum as f64 / 1_000_000 as f64) / total_requests as f64;
	total_requests *= 2; // we multiple by 2 because it's two messages.

	let messages_per_second = (total_requests * 1000) as f64 / duration.as_millis() as f64;

	info_no_ts!("{}", SEPARATOR);
	info!(
		"Total elapsed time = {}ms. Total requests = {}.",
		duration.as_millis(),
		total_requests,
	);
	info!("Messages per second = {}.", messages_per_second);
	info!("Average round trip latency = {}ms.", avg_lat);

	Ok(())
}
