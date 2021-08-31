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

use crate::{Readable, Writeable};
use lazy_static::lazy_static;
pub use nioruntime_http::{ConnData, HttpConfig, HttpServer};
use nioruntime_http::{HttpMethod, HttpVersion, State, WriteHandle};
use nioruntime_log::*;
use nioruntime_util::ser::BinReader;
use nioruntime_util::ser::BinWriter;
use nioruntime_util::{Error, ErrorKind};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::metadata;
use std::fs::File;
use std::io::Read;
use std::pin::Pin;
use std::sync::RwLockWriteGuard;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

info!();
const MAIN_LOG: &str = "mainlog";
const MAX_CHUNK_SIZE: usize = 1024 * 1024 * 10;
const MAX_ESCAPE_SEQUENCE: usize = 100;
const SEPARATOR_LINE: &str =
	"------------------------------------------------------------------------------------------------------------------------------------";

// note: AsyncContext currently does not support HTTP pipelining. No major browsers appear to support HTTP pipelining.
// but this should technically be supported as part of the HTTP standard.

#[derive(Clone)]
pub struct RustletAsyncContext {
	pub request: Option<RustletRequest>,
	pub response: Option<RustletResponse>,
}

impl RustletAsyncContext {
	pub fn complete(&mut self) -> Result<(), Error> {
		match &mut self.response {
			Some(response) => {
				response.async_complete()?;
			}
			None => {
				log_multi!(ERROR, MAIN_LOG, "response not found in async context");
			}
		}
		Ok(())
	}
}

pub struct SessionData {
	mod_time: u128,
	data: HashMap<String, Vec<u8>>,
}

impl SessionData {
	fn new() -> Self {
		let now = SystemTime::now().duration_since(UNIX_EPOCH);
		let now = now
			.unwrap_or(std::time::Duration::from_millis(0))
			.as_millis();

		SessionData {
			mod_time: now,
			data: HashMap::new(),
		}
	}
}

#[derive(Clone)]
pub struct RustletRequest {
	content: Vec<u8>,
	http_method: HttpMethod,
	http_version: HttpVersion,
	http_config: HttpConfig,
	uri: String,
	query: String,
	headers: Vec<(Vec<u8>, Vec<u8>)>,
	keep_alive: bool,
	query_map: Option<HashMap<String, String>>,
	header_map: Option<HashMap<String, String>>,
	session_map: Arc<RwLock<HashMap<u128, SessionData>>>,
	session_id: u128,
}

impl RustletRequest {
	pub fn new(
		uri: String,
		query: String,
		content: Vec<u8>,
		http_method: HttpMethod,
		http_version: HttpVersion,
		http_config: HttpConfig,
		headers: Vec<(Vec<u8>, Vec<u8>)>,
		keep_alive: bool,
		session_map: Arc<RwLock<HashMap<u128, SessionData>>>,
	) -> Self {
		RustletRequest {
			uri,
			query,
			content,
			http_method,
			http_version,
			http_config,
			headers,
			keep_alive,
			query_map: None,
			header_map: None,
			session_map,
			session_id: 0,
		}
	}

	pub fn set_session_id(&mut self, session_id: u128) -> Result<(), Error> {
		self.session_id = session_id;

		Ok(())
	}

	pub fn get_session<T: Readable>(&mut self, name: &str) -> Result<Option<T>, Error> {
		let mut create_session = false;
		{
			let mut session_map = nioruntime_util::lockw!(self.session_map);
			match session_map.get_mut(&self.session_id) {
				Some(mut data) => {
					let value = data.data.get(&name.to_string());
					let now = SystemTime::now()
						.duration_since(UNIX_EPOCH)
						.map_err(|e| {
							let error: Error =
								ErrorKind::InternalError(format!("time went backwards, {}", e))
									.into();
							error
						})?
						.as_millis();
					data.mod_time = now;
					match value {
						Some(value) => {
							return Ok(Some(Readable::read(&mut BinReader::new(
								&mut value.as_slice(),
							))?))
						}
						None => {}
					}
				}
				None => {
					create_session = true;
				}
			}
			if create_session {
				session_map.insert(self.session_id, SessionData::new());
			}
		}

		Ok(None)
	}

	pub fn set_session<T: Writeable>(&mut self, name: &str, value: T) -> Result<(), Error> {
		let mut session_map = nioruntime_util::lockw!(self.session_map);
		match session_map.get_mut(&self.session_id) {
			Some(session_data) => {
				let mut sink: Vec<u8> = vec![];
				let mut writer = BinWriter::new(&mut sink);
				value.write(&mut writer)?;
				session_data.data.insert(name.to_string(), sink);
				let now = SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.map_err(|e| {
						let error: Error =
							ErrorKind::InternalError(format!("time went backwards, {}", e)).into();
						error
					})?
					.as_millis();
				session_data.mod_time = now;
			}
			None => {
				let mut session_data = SessionData::new();
				let mut sink: Vec<u8> = vec![];
				let mut writer = BinWriter::new(&mut sink);
				value.write(&mut writer)?;
				session_data.data.insert(name.to_string(), sink);
				let now = SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.map_err(|e| {
						let error: Error =
							ErrorKind::InternalError(format!("time went backwards, {}", e)).into();
						error
					})?
					.as_millis();
				session_data.mod_time = now;
				session_map.insert(self.session_id, session_data);
			}
		};

		Ok(())
	}

	pub fn remove_session_entry(&mut self, name: &str) -> Result<(), Error> {
		let mut session_map = nioruntime_util::lockw!(self.session_map);

		match session_map.get_mut(&self.session_id) {
			Some(session_data) => {
				session_data.data.remove(&name.to_string());
				let now = SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.map_err(|e| {
						let error: Error =
							ErrorKind::InternalError(format!("time went backwards, {}", e)).into();
						error
					})?
					.as_millis();
				session_data.mod_time = now;
			}
			None => {}
		}

		Ok(())
	}

	pub fn invalidate_session(&mut self) -> Result<(), Error> {
		let mut session_map = nioruntime_util::lockw!(self.session_map);
		session_map.remove(&self.session_id);

		Ok(())
	}

	pub fn get_cookie(&mut self, name: &str) -> Result<Option<String>, Error> {
		if self.header_map.is_none() {
			self.build_header_map()?;
		}

		let cookie_str = self.header_map.as_ref().unwrap().get("Cookie");
		match cookie_str {
			Some(cookie_str) => {
				let cookie_spl = cookie_str.split(";");
				for cookie in cookie_spl {
					let cookie = cookie.trim();
					let cookie_spl: Vec<&str> = cookie.split("=").collect();
					if cookie_spl.len() >= 2 && cookie_spl[0] == name {
						return Ok(Some(cookie_spl[1].to_string()));
					}
				}
			}
			None => {}
		}

		Ok(None)
	}

	pub fn get_header_len(&mut self) -> Result<usize, Error> {
		if self.header_map.is_none() {
			self.build_header_map()?;
		}

		Ok(match self.header_map.as_ref() {
			Some(map) => map.len(),
			None => 0,
		})
	}

	pub fn get_header_i_value(&self, i: usize) -> Result<String, Error> {
		let vec_len = self.headers.len();
		if i >= vec_len {
			Ok("".to_string())
		} else {
			Ok(std::str::from_utf8(&self.headers[i].1)
				.unwrap_or(&"".to_string())
				.to_string())
		}
	}

	pub fn get_header_i_name(&self, i: usize) -> Result<String, Error> {
		let vec_len = self.headers.len();
		if i >= vec_len {
			Ok("".to_string())
		} else {
			Ok(std::str::from_utf8(&self.headers[i].0)
				.unwrap_or(&"".to_string())
				.to_string())
		}
	}

	pub fn get_header(&mut self, name: &str) -> Result<Option<String>, Error> {
		let name = name.to_string();
		if self.header_map.is_none() {
			self.build_header_map()?;
		}

		match self.header_map.as_ref() {
			Some(map) => {
				let value = map.get(&name);
				match value {
					Some(value) => Ok(Some((*value).clone())),
					None => Ok(None),
				}
			}
			None => Ok(None),
		}
	}

	pub fn get_headers(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
		Ok(self.headers.clone())
	}

	pub fn get_http_method(&self) -> Result<HttpMethod, Error> {
		Ok(self.http_method.clone())
	}

	pub fn get_http_version(&self) -> Result<HttpVersion, Error> {
		Ok(self.http_version.clone())
	}

	pub fn get_content(&self) -> Result<Vec<u8>, Error> {
		Ok(self.content.clone())
	}

	pub fn get_uri(&self) -> Result<String, Error> {
		Ok(self.uri.clone())
	}

	pub fn get_query(&self) -> Result<String, Error> {
		Ok(self.query.clone())
	}

	pub fn get_query_parameter(&mut self, name: &str) -> Result<Option<String>, Error> {
		let name = name.to_string();
		if self.query_map.is_none() {
			self.build_query_map()?;
		}

		match self.query_map.as_ref() {
			Some(map) => {
				let value = map.get(&name);
				match value {
					Some(value) => Ok(Some((*value).clone())),
					None => Ok(None),
				}
			}
			None => Ok(None),
		}
	}

	fn build_header_map(&mut self) -> Result<(), Error> {
		let mut map = HashMap::new();
		let vec_len = self.headers.len();
		for i in 0..vec_len {
			let key = std::str::from_utf8(&self.headers[i].0);
			let value = std::str::from_utf8(&self.headers[i].1);

			// we don't accept non utf-8 headers
			if key.is_err() || value.is_err() {
				continue;
			}

			map.insert(key.unwrap().to_string(), value.unwrap().to_string());
		}
		self.header_map = Some(map);
		Ok(())
	}

	fn build_query_map(&mut self) -> Result<(), Error> {
		let vec = querystring::querify(&self.query);
		let vec_len = vec.len();
		let mut map = HashMap::new();
		for i in 0..vec_len {
			map.insert(vec[i].0.to_string(), vec[i].1.to_string());
		}
		self.query_map = Some(map);
		Ok(())
	}
}

#[derive(Clone)]
pub struct RustletResponse {
	wh: WriteHandle,
	config: HttpConfig,
	headers_written: Arc<Mutex<bool>>,
	additional_headers: Vec<(String, String)>,
	redirect: Arc<Mutex<Option<String>>>,
	keep_alive: bool,
	chained: bool,
	is_async: Arc<RwLock<bool>>,
	buffer: Arc<RwLock<Vec<u8>>>,
	is_complete: bool,
}

impl RustletResponse {
	pub fn new(wh: WriteHandle, config: HttpConfig, keep_alive: bool, chained: bool) -> Self {
		RustletResponse {
			wh,
			config,
			headers_written: Arc::new(Mutex::new(false)),
			keep_alive,
			additional_headers: vec![],
			redirect: Arc::new(Mutex::new(None)),
			chained,
			is_async: Arc::new(RwLock::new(false)),
			buffer: Arc::new(RwLock::new(vec![])),
			is_complete: false,
		}
	}

	pub fn set_cookie(&mut self, name: &str, value: &str, other: &str) -> Result<(), Error> {
		match self.get_headers_written() {
			true => Err(ErrorKind::OrderingError(
				"Headers already written. Cannot set a cookie".to_string(),
			)
			.into()),
			false => {
				self.additional_headers.push((
					"Set-Cookie".to_string(),
					format!("{}={}; {}", name, value, other),
				));
				Ok(())
			}
		}
	}

	fn get_headers_written(&self) -> bool {
		match self.headers_written.lock() {
			Ok(v) => *v,
			Err(e) => *e.into_inner(),
		}
	}

	fn set_headers_written(&self, value: bool) {
		match self.headers_written.lock() {
			Ok(mut v) => *v = value,
			Err(e) => *e.into_inner() = value,
		}
	}

	fn get_redirect(&self) -> Option<String> {
		match self.redirect.lock() {
			Ok(r) => (*r).clone(),
			Err(e) => (*e.into_inner()).clone(),
		}
	}

	pub fn set_redirect(&self, value: &str) -> Result<(), Error> {
		if self.get_headers_written() {
			return Err(ErrorKind::OrderingError(
				"headers already written. Cannot set redirect".to_string(),
			)
			.into());
		}
		match self.redirect.lock() {
			Ok(mut r) => *r = Some(value.to_string()),
			Err(e) => *e.into_inner() = Some(value.to_string()),
		}

		Ok(())
	}

	pub fn add_header(&mut self, name: &str, value: &str) -> Result<(), Error> {
		if self.get_headers_written() {
			Err(ErrorKind::OrderingError(
				"headers already written. Cannot add a header".to_string(),
			)
			.into())
		} else {
			self.additional_headers
				.push((name.to_string(), value.to_string()));
			Ok(())
		}
	}

	pub fn set_content_type(&mut self, ctype: &str) -> Result<(), Error> {
		if self.get_headers_written() {
			Err(ErrorKind::OrderingError(
				"headers already written. Cannot set content-type".to_string(),
			)
			.into())
		} else {
			self.additional_headers
				.push(("Content-Type".to_string(), ctype.to_string()));
			Ok(())
		}
	}

	pub fn flush(&mut self) -> Result<(), Error> {
		let mut buffer = nioruntime_util::lockw!(self.buffer);
		let headers = if !self.get_headers_written() && !self.chained {
			self.set_headers_written(true);
			HttpServer::build_headers(
				&self.config,
				true,
				self.keep_alive,
				self.additional_headers.clone(),
				self.get_redirect(),
			)?
		} else {
			"".to_string()
		};

		if self.keep_alive {
			let buffer_len = buffer.len();
			if buffer_len > 0 {
				let msg_len_bytes = format!("{:X}\r\n", buffer_len);
				let msg_len_bytes = msg_len_bytes.as_bytes();
				let len = msg_len_bytes.len();
				for i in 0..len {
					buffer.insert(i, msg_len_bytes[i]);
				}
				buffer.push('\r' as u8);
				buffer.push('\n' as u8);
			}
			if self.is_complete {
				buffer.push('0' as u8);
				buffer.push('\r' as u8);
				buffer.push('\n' as u8);
				buffer.push('\r' as u8);
				buffer.push('\n' as u8);
			}
		}

		let headers = headers.as_bytes();
		// TODO: optimize this?
		for i in 0..headers.len() {
			buffer.insert(i, headers[i]);
		}

		let data = &buffer[..];
		self.wh.write(&data[0..data.len()])?;
		self.set_headers_written(true);
		let mut callback_state = nioruntime_util::lockw!(self.wh.callback_state);
		match self.keep_alive {
			true => *callback_state = State::HeadersChunked,
			false => *callback_state = State::HeadersClose,
		}
		info!("setting callback state to {:?}", *callback_state);
		buffer.clear();

		Ok(())
	}

	pub fn write(&mut self, data: &[u8]) -> Result<(), Error> {
		let mut buffer = nioruntime_util::lockw!(self.buffer);
		buffer.append(&mut data.to_vec());
		Ok(())
	}

	pub fn set_is_async(&mut self, value: bool) -> Result<(), Error> {
		(*nioruntime_util::lockw!(self.is_async)) = value;
		Ok(())
	}

	pub fn async_complete(&mut self) -> Result<(), Error> {
		self.set_is_async(false)?;
		self.complete()?;
		Ok(())
	}

	pub fn complete(&mut self) -> Result<(), Error> {
		if self.chained || (*nioruntime_util::lockr!(self.is_async)) {
			if self.chained {
				self.flush()?;
			}
			// don't close the connection or send 0 len if we're in a chained request
			return Ok(());
		}

		self.is_complete = true;
		self.flush()?;
		if !self.keep_alive {
			self.wh.close()?;
		}

		Ok(())
	}
}

pub type Rustlet =
	Pin<Box<dyn Fn(&mut RustletRequest, &mut RustletResponse) -> Result<(), Error> + Send + Sync>>;

pub(crate) struct RustletContainerHolder {
	rustlets: HashMap<String, Pin<Box<Rustlet>>>,
	mappings: HashMap<String, String>,
}

impl RustletContainerHolder {
	pub fn new() -> Self {
		RustletContainerHolder {
			rustlets: HashMap::new(),
			mappings: HashMap::new(),
		}
	}
}

lazy_static! {
	pub(crate) static ref RUSTLETS: Arc<RwLock<RustletContainerHolder>> =
		Arc::new(RwLock::new(RustletContainerHolder::new()));
	pub(crate) static ref SESSION_MAP: Arc<RwLock<HashMap<u128, SessionData>>> =
		Arc::new(RwLock::new(HashMap::new()));
	pub(crate) static ref RUSTLET_CONFIG: Arc<RwLock<Option<RustletConfig>>> =
		Arc::new(RwLock::new(None));
}

/// The configuration of the rustlet container.
#[derive(Clone)]
pub struct RustletConfig {
	/// The timeout (in seconds) for sessions in this container. The default value is 1,800 seconds (30 minutes).
	pub session_timeout: u64,
	/// The [`nioruntime_http::HttpConfig`] configuration for this container.
	pub http_config: HttpConfig,
}

impl Default for RustletConfig {
	fn default() -> RustletConfig {
		RustletConfig {
			session_timeout: 60 * 30, // 30 mins
			http_config: HttpConfig::default(),
		}
	}
}

fn housekeeper() -> Result<(), Error> {
	let session_timeout = {
		let config = nioruntime_util::lockr!(RUSTLET_CONFIG);
		match &(*config) {
			Some(config) => config.session_timeout,
			None => 0,
		}
	};

	if session_timeout > 0 {
		let mut session_map = nioruntime_util::lockw!(SESSION_MAP);

		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(|e| {
				let error: Error =
					ErrorKind::InternalError(format!("time went backwards, {}", e)).into();
				error
			})?
			.as_millis();

		let mut rem_list = vec![];
		for (k, v) in &*session_map {
			let diff = (now - v.mod_time) / 1000;
			if diff > session_timeout.into() {
				rem_list.push(k.clone());
			}
		}

		for id in rem_list {
			session_map.remove(&id);
		}
	}

	Ok(())
}

fn on_panic() -> Result<(), Error> {
	let container = nioruntime_util::lockw!(crate::macros::RUSTLET_CONTAINER);
	match &container.http {
		Some(http) => {
			if http.http_context.is_some() {
				HttpServer::do_house_keeping(http.http_context.as_ref().unwrap(), &http.config)?;
			}
		}
		None => {}
	}
	Ok(())
}

fn api_callback(
	conn_data: &mut RwLockWriteGuard<ConnData>, // connection_data
	has_content: bool,                          // does this request have content?
	start_content: usize,                       // start content in ConnData buffer
	end_content: usize,                         // end content in ConnData buffer
	method: HttpMethod,                         // GET or POST
	config: HttpConfig,                         // HttpServer's configuration
	wh: WriteHandle,                            // WriteHandle to write back data
	version: HttpVersion,                       // HttpVersion
	uri: &str,                                  // uri
	query: &str,                                // query
	headers: Vec<(Vec<u8>, Vec<u8>)>,           // headers
	keep_alive: bool,                           // keep-alive
) -> Result<(), Error> {
	let res = do_api_callback(
		conn_data,
		has_content,
		start_content,
		end_content,
		method,
		config.clone(),
		wh.clone(),
		version,
		uri,
		query,
		headers,
		keep_alive,
		SESSION_MAP.clone(),
	);

	match res {
		Ok(_) => {}
		Err(e) => {
			log_multi!(
				ERROR,
				MAIN_LOG,
				"error calling [{}?{}]: '{}'",
				uri,
				query,
				e.to_string()
			);

			let (headers_written, _redir) =
				crate::macros::LOCALRUSTLET.with(|f| match &(*f.borrow()) {
					Some((_request, response)) => {
						(response.get_headers_written(), response.get_redirect())
					}
					None => (false, None),
				});
			if headers_written {
				if !keep_alive {
					let mut response = RustletResponse::new(wh.clone(), config, false, true);
					response.write(
						format!(
							"{}{}{}",
							"\n</br>",
							SEPARATOR_LINE,
							"\n</br>Internal Server error. See logs for details.</body></html>"
						)
						.as_bytes(),
					)?;
					wh.close()?;
				} else {
					let msg_str = format!(
						"{}{}{}",
						"\n</br>",
						SEPARATOR_LINE,
						"\n</br>Internal Server error. See logs for details.</body></html>"
					);
					let msg = msg_str.as_bytes();
					let msg_len_bytes = format!("{:X}\r\n", msg.len());
					wh.write(&msg_len_bytes.as_bytes()[0..msg_len_bytes.len()])?;
					wh.write(&msg[0..msg.len()])?;
					wh.write(&("\r\n0\r\n\r\n".as_bytes())[0..7])?;
					wh.close()?;
				}
			} else {
				let mut response = RustletResponse::new(wh.clone(), config, false, false);
				response.write("Internal Server error. See logs for details.".as_bytes())?;
				wh.close()?;
			}
		}
	}

	Ok(())
}

fn execute_rustlet(
	rustlet_name: &str,
	conn_data: &mut RwLockWriteGuard<ConnData>, // connection_data
	has_content: bool,                          // whether this request has content
	start_content: usize,                       // start content
	end_content: usize,                         // end content
	method: HttpMethod,                         // GET or POST
	config: HttpConfig,                         // HttpServer's configuration
	wh: WriteHandle,                            // WriteHandle to write back data
	version: HttpVersion,                       // HttpVersion
	uri: &str,                                  // uri
	query: &str,                                // query
	headers: Vec<(Vec<u8>, Vec<u8>)>,           // headers
	keep_alive: bool,                           // keep-alive
	chained: bool,                              // is this a chained rustlet call?
	session_map: Arc<RwLock<HashMap<u128, SessionData>>>,
) -> Result<(), Error> {
	let rustlets = nioruntime_util::lockr!(RUSTLETS);

	let rustlet = rustlets.rustlets.get(rustlet_name);
	match rustlet {
		Some(rustlet) => {
			let mut response = RustletResponse::new(wh, config.clone(), keep_alive, chained);
			let content = match has_content {
				true => (*conn_data).get_buffer()[start_content..end_content].to_vec(),
				false => vec![],
			};
			let mut request = RustletRequest::new(
				uri.to_string(),
				query.to_string(),
				content,
				method,
				version,
				config,
				headers,
				keep_alive,
				session_map,
			);
			let id: u128 = rand::random();
			let rsessionid = request.get_cookie("rustletsessionid");

			let rsessionid = match rsessionid {
				Ok(rsessionid) => match rsessionid {
					Some(rsessionid) => match rsessionid.parse() {
						Ok(rsessionid) => rsessionid,
						Err(_) => id,
					},
					None => id,
				},
				Err(e) => {
					log_multi!(
						ERROR,
						MAIN_LOG,
						"error getting rsessionid: {}",
						e.to_string()
					);
					id
				}
			};

			if rsessionid == id {
				// we have to set this as it's a new id
				response.set_cookie("rustletsessionid", &format!("{}", id), "path=/")?;
			}

			request.set_session_id(rsessionid)?;
			(rustlet)(&mut request, &mut response).map_err(|e| {
				match response.flush() {
					Ok(_) => {}
					Err(e) => {
						log_multi!(ERROR, MAIN_LOG, "error flushing: {}", e.to_string());
					}
				}

				return e;
			})?;
			response.complete()?;
		}
		None => {
			let mut response = RustletResponse::new(wh.clone(), config, keep_alive, chained);
			response.write(format!("Rustlet '{}' does not exist.", rustlet_name).as_bytes())?;
			if keep_alive {
				wh.write(&("0\r\n\r\n".as_bytes())[0..5])?;
			} else {
				wh.close()?;
			}
		}
	}
	Ok(())
}

fn do_api_callback(
	conn_data: &mut RwLockWriteGuard<ConnData>, // connection_data
	has_content: bool,
	start_content: usize,
	end_content: usize,
	method: HttpMethod,               // GET or POST
	config: HttpConfig,               // HttpServer's configuration
	wh: WriteHandle,                  // WriteHandle to write back data
	version: HttpVersion,             // HttpVersion
	uri: &str,                        // uri
	query: &str,                      // query
	headers: Vec<(Vec<u8>, Vec<u8>)>, // headers
	keep_alive: bool,                 // keep-alive
	session_map: Arc<RwLock<HashMap<u128, SessionData>>>,
) -> Result<(), Error> {
	let rustlets = nioruntime_util::lockr!(RUSTLETS);

	let rustlet = rustlets.mappings.get(uri);
	match rustlet {
		Some(rustlet_name) => {
			execute_rustlet(
				rustlet_name,
				conn_data,
				has_content,
				start_content,
				end_content,
				method,
				config,
				wh,
				version,
				uri,
				query,
				headers,
				keep_alive,
				false,
				session_map,
			)?;
		}
		None => {
			// see if it's an RSP.
			if uri.to_lowercase().ends_with(".rsp") {
				process_rsp(
					conn_data,
					//content,
					has_content,
					start_content,
					end_content,
					method,
					config,
					wh,
					version,
					uri,
					query,
					headers,
					keep_alive,
					session_map,
				)?;
			} else {
				log_multi!(ERROR, MAIN_LOG, "error, no mapping for '{}'", uri);
				let mut response = RustletResponse::new(wh.clone(), config, false, false);
				response.write("Internal Server error. See logs for details.".as_bytes())?;
				wh.close()?;
			}
		}
	}

	Ok(())
}

fn process_rsp(
	conn_data: &mut RwLockWriteGuard<ConnData>, // connection_data
	has_content: bool,
	start_content: usize,
	end_content: usize,
	method: HttpMethod,               // GET or POST
	config: HttpConfig,               // HttpServer's configuration
	wh: WriteHandle,                  // WriteHandle to write back data
	version: HttpVersion,             // HttpVersion
	uri: &str,                        // uri
	query: &str,                      // query
	headers: Vec<(Vec<u8>, Vec<u8>)>, // headers
	keep_alive: bool,                 // keep-alive
	session_map: Arc<RwLock<HashMap<u128, SessionData>>>,
) -> Result<(), Error> {
	let rsp_path = HttpServer::get_path(&config, uri)?;
	let mut flen = metadata(rsp_path.clone())?.len();
	let mut file = File::open(rsp_path.clone())?;
	let buflen: usize = if flen.try_into().unwrap_or(MAX_CHUNK_SIZE) > MAX_CHUNK_SIZE {
		return Err(ErrorKind::InvalidRSPError(format!(
			"RSPs are limited to {} bytes.",
			MAX_CHUNK_SIZE
		))
		.into());
	} else {
		flen.try_into().unwrap_or(MAX_CHUNK_SIZE)
	};

	let mut buf = vec![0; (buflen + MAX_ESCAPE_SEQUENCE) as usize];
	let mut first_loop = true;

	loop {
		let amt = file.read(&mut buf[0..buflen])?;
		if first_loop {
			HttpServer::write_headers(&wh, &config, true, keep_alive, vec![], None)?;
			let mut callback_state = nioruntime_util::lockw!(wh.callback_state);
			match keep_alive {
				true => *callback_state = State::HeadersChunked,
				false => *callback_state = State::HeadersClose,
			}
		}

		let mut start = 0;
		let mut end;
		loop {
			end = amt;
			for i in (start + 2)..amt {
				if buf[i] == '=' as u8 && buf[i - 1] == '@' as u8 && buf[i - 2] == '<' as u8 {
					// we have begun an escape sequence
					end = i - 2;
					break;
				}
			}
			let wlen = end - start;
			if keep_alive {
				let msg_len_bytes = format!("{:X}\r\n", wlen);
				let msg_len_bytes = msg_len_bytes.as_bytes();
				wh.write(&msg_len_bytes[0..msg_len_bytes.len()])?;
				wh.write(&buf[start..end])?;
				if flen <= end.try_into().unwrap_or(0) {
					wh.write(&("\r\n0\r\n\r\n".as_bytes())[0..7])?;
					if !keep_alive {
						wh.close()?;
					}
				} else {
					wh.write(&("\r\n".as_bytes())[0..2])?;
				}
			} else {
				if flen <= end.try_into().unwrap_or(0) {
					wh.write(&buf[start..end])?;
					if !keep_alive {
						wh.close()?;
					}
				} else {
					wh.write(&buf[start..end])?;
				}
			}

			if end == amt {
				break;
			} else {
				// find the end of the escape sequence
				for i in end + 3..(amt - 1) {
					if buf[i] == '>' as u8 {
						let rustlet_name = std::str::from_utf8(&buf[(end + 3)..i])?;
						execute_rustlet(
							rustlet_name,
							conn_data,
							//content,
							has_content,
							start_content,
							end_content,
							method.clone(),
							config.clone(),
							wh.clone(),
							version.clone(),
							uri,
							query,
							headers.clone(),
							keep_alive,
							true,
							session_map.clone(),
						)?;
						start = i + 1;
						break;
					}
				}
				if start < end {
					// error we didn't find the end of it
					// TODO: handle chunk overlapping escape sequences
					// TODO: handle invalid RSP better, show linenum, etc
					return Err(ErrorKind::InvalidRSPError(
						"non-terminated escape sequence in RSP".to_string(),
					)
					.into());
				}
			}
		}
		flen -= amt.try_into().unwrap_or(0);

		if flen <= 0 {
			break;
		}
		first_loop = false;
	}

	Ok(())
}

pub struct RustletContainer {
	config: Option<RustletConfig>,
	http: Option<HttpServer>,
}

impl RustletContainer {
	pub fn new() -> Self {
		RustletContainer {
			config: None,
			http: None,
		}
	}

	pub fn set_config(&mut self, config: RustletConfig) -> Result<(), Error> {
		let http = HttpServer::new(config.http_config.clone());
		self.config = Some(config.clone());
		self.http = Some(http);
		let mut static_config = nioruntime_util::lockw!(RUSTLET_CONFIG);

		*static_config = Some(config);

		Ok(())
	}

	pub fn start(&mut self) -> Result<(), Error> {
		let http = self.http.as_mut();
		match http {
			Some(mut http) => {
				http.config.callback = api_callback;
				http.config.on_panic = on_panic;
				http.config.on_housekeeper = housekeeper;
				http.start()?;
				http.add_api_extension("rsp".to_string())?;
			}
			None => {
				log_multi!(
					ERROR,
					MAIN_LOG,
					"Couldn't start rustlet: Configuration not found"
				);
			}
		}

		Ok(())
	}

	pub fn add_rustlet(&mut self, name: &str, rustlet: Rustlet) -> Result<(), Error> {
		let mut rustlets = nioruntime_util::lockw!(RUSTLETS);
		rustlets
			.rustlets
			.insert(name.to_string(), Box::pin(rustlet));

		Ok(())
	}

	pub fn add_rustlet_mapping(&mut self, path: &str, name: &str) -> Result<(), Error> {
		let mut rustlets = nioruntime_util::lockw!(RUSTLETS);

		match self.http.as_ref() {
			Some(http) => {
				http.add_api_mapping(path.to_string())?;
				rustlets.mappings.insert(path.to_string(), name.to_string());
			}
			None => {
				log_multi!(
					ERROR,
					MAIN_LOG,
					"Couldn't add rustlet mapping: Configuration not found"
				);
			}
		}

		Ok(())
	}
}
