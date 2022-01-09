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

use crate::RustletContainer;
use crate::{RustletRequest, RustletResponse};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use std::thread_local;

thread_local!(
	pub static LOCALRUSTLET: RefCell<
		Option<
			(RustletRequest,RustletResponse)
		>
	> = RefCell::new(None)
);

lazy_static! {
	pub static ref RUSTLET_CONTAINER: Arc<RwLock<RustletContainer>> =
		Arc::new(RwLock::new(RustletContainer::new()));
}

/// Delete the session. See [`session`] for more info on sessions. If a parameter is specified,
/// only that parameter is deleted from the session. With no parameter specified, the entire session
/// is invalidated.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// // implement a simple `Writeable` and `Readable` struct to demonstrate how the session works.
/// #[derive(Debug)]
/// struct Example {
///    num: u32,
/// }
///
/// impl Example {
///     pub fn new(num: u32) -> Self {
///         Example { num }
///     }
/// }
///
/// impl Writeable for Example {
///     fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
///         writer.write_u32(self.num)?;
///         Ok(())
///     }
/// }
///
/// impl Readable for Example {
///     fn read<R: Reader>(reader: &mut R) -> Result<Self, Error> {
///         let num = reader.read_u32()?;
///         Ok(Example { num })
///     }
/// }
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     // this rustlet gets the value stored in session variable "abc". If it has not been set
///     // (i.e. by the set_session rustlet, 'none' is displayed.
///     rustlet!("get_session", {
///         let value: Option<Example> = session!("abc"); // the type 'Example' is coerced here
///         match value {
///             Some(value) => {
///                 response!("abc={:?}", value); // print out it's value
///             }
///             None => {
///                 response!("none"); // print out none
///             }
///         }
///     });
///
///     rustlet!("set_session", {
///         // get the value of 'abc' in the query string and try to parse as u32
///         let val: u32 = query!("abc").parse()?;
///         // create an Example with this value and insert it into the session under var 'abc'
///         session!("abc", Example::new(val));
///     });
///
///     // delete the entire session
///     rustlet!("delete_session", {
///         session_delete!();
///     });
///
///     // delete only the 'abc' value from the session
///     rustlet!("delete_abc", {
///         session_delete!("abc");
///     });
///
///     rustlet_mapping!("/get_session", "get_session");
///     rustlet_mapping!("/set_session", "set_session");
///     rustlet_mapping!("/delete_session", "delete_session");
///     rustlet_mapping!("/delete_abc", "delete_abc");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! session_delete {
	($a:expr) => {
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut *(f.borrow_mut()) {
			Some((request, _response)) => match request.remove_session_entry($a) {
				Ok(_) => {}
				Err(e) => {
					mainlogerror!("session_delete generated error: {}", e.to_string());
				}
			},
			None => {
				mainlogerror!("Error: not in a rustlet context");
			}
		});
	};
	() => {
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut *(f.borrow_mut()) {
			Some((request, _response)) => match request.invalidate_session() {
				Ok(_) => {}
				Err(e) => {
					mainlogerror!("invalide_session generated error: {}", e.to_string());
				}
			},
			None => {
				mainlogerror!("Error: not in a rustlet context");
			}
		});
	};
}

/// Sets or gets a value in the session. The session is an in-memory key/value store that can be used
/// by rustlets to store data that can be accessed by other rustlets. A session cookie is set called
/// rustletsessionid that lets the rustlet container know which user is which. The session is automatically
/// invalidated after a certain period of time where no calls to session! or session_delete! are made. By
/// default, this amount of time is 30 minutes, but it is configurable in
/// [`crate::RustletConfig::session_timeout`]. If only one parameter is specified, the value is retrieved
/// from the session data store, if two parameters are specified, the value is set, see the examples below
/// for more details.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// // implement a simple `Writeable` and `Readable` struct to demonstrate how the session works.
/// #[derive(Debug)]
/// struct Example {
///    num: u32,
/// }
///
/// impl Example {
///     pub fn new(num: u32) -> Self {
///         Example { num }
///     }
/// }
///
/// impl Writeable for Example {
///     fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
///         writer.write_u32(self.num)?;
///         Ok(())
///     }
/// }
///
/// impl Readable for Example {
///     fn read<R: Reader>(reader: &mut R) -> Result<Self, Error> {
///         let num = reader.read_u32()?;
///         Ok(Example { num })
///     }  
/// }
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     // this rustlet gets the value stored in session variable "abc". If it has not been set
///     // (i.e. by the set_session rustlet, 'none' is displayed.
///     rustlet!("get_session", {
///         let value: Option<Example> = session!("abc"); // the type 'Example' is coerced here
///         match value {
///             Some(value) => {
///                 response!("abc={:?}", value); // print out it's value
///             }
///             None => {
///                 response!("none"); // print out none
///             }
///         }
///     });
///     
///     rustlet!("set_session", {
///         // get the value of 'abc' in the query string and try to parse as u32
///         let val: u32 = query!("abc").parse()?;
///         // create an Example with this value and insert it into the session under var 'abc'
///         session!("abc", Example::new(val));
///     });
///     
///     // delete the entire session
///     rustlet!("delete_session", {
///         session_delete!();
///     });
///     
///     // delete only the 'abc' value from the session
///     rustlet!("delete_abc", {
///         session_delete!("abc");
///     });
///
///     rustlet_mapping!("/get_session", "get_session");
///     rustlet_mapping!("/set_session", "set_session");
///     rustlet_mapping!("/delete_session", "delete_session");
///     rustlet_mapping!("/delete_abc", "delete_abc");
///     
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! session {
	($a:expr) => {
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut *(f.borrow_mut()) {
			Some((request, _response)) => match request.get_session($a) {
				Ok(value) => value,
				Err(e) => {
					mainlogerror!("get_session generated error: {}", e.to_string());
					None
				}
			},
			None => {
				mainlogerror!("Error: not in a rustlet context");
				None
			}
		})
	};
	($a:expr,$b:expr) => {
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut *(f.borrow_mut()) {
			Some((request, _response)) => match request.set_session($a, $b) {
				Ok(_) => {}
				Err(e) => {
					mainlogerror!("get_session generated error: {}", e.to_string());
				}
			},
			None => {
				mainlogerror!("Error: not in a rustlet context");
			}
		})
	};
}

/// Gets the tor pubkey associated with this rustlet container.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("set_content_type", {
///         set_content_type!("text/html");
///         response!("<html><body><strong>");
///         let tor_pubkey = tor_pubkey!();
///         response!("tor pubkey = {:?}", tor_pubkey);
///         response!("</strong></body></html>");
///         // flush called automatically by the container after control is returned.
///     });
///
///     rustlet_mapping!("/set_content_type", "set_content_type");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! tor_pubkey {
	() => {{
		let mut container = librustlet::macros::RUSTLET_CONTAINER.write();
		match container {
			Ok(mut container) => {
				let res = container.get_onion_address_pubkey();
				match res {
					Ok(onion_pubkey) => onion_pubkey,
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						nioruntime_log::log_multi!(
							nioruntime_log::ERROR,
							MAIN_LOG,
							"Error getting onion pubkey: {}",
							e.to_string()
						);
						None
					}
				}
			}
			Err(e) => {
				const MAIN_LOG: &str = "mainlog";
				nioruntime_log::log_multi!(
					nioruntime_log::ERROR,
					MAIN_LOG,
					"Couldn't get tor pubkey: couldn't get lock: {}",
					e.to_string()
				);

				None
			}
		}
	}};
}

/// Flushes any buffered data previously sent via the [`response`] macro.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("set_content_type", {
///         set_content_type!("text/html");
///         response!("<html><body><strong>Some Content Here");
///         flush!(); // the first part of the response will be written immidiately.
///         response!("</strong></body></html>");
///         // flush called automatically by the container after control is returned.
///     });
///
///     rustlet_mapping!("/set_content_type", "set_content_type");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! flush {
	() => {
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut *(f.borrow_mut()) {
			Some((_request, response)) => match response.flush() {
				Ok(_) => {}
				Err(e) => {
					mainlogerror!("async_complete generated error: {}", e.to_string());
				}
			},
			None => {
				mainlogerror!("Error: not in a rustlet context");
			}
		});
	};
}

/// Completes a rustlet execution that is executing in an async_context.
/// This macro must be called so that the rustlet container knows that
/// a particular async rustlet is complete. Also see [`async_context`] and the
/// example below.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("async", {
///         response!("first message\n");
///         let ac = async_context!();
///
///         std::thread::spawn(move || {
///             async_context!(ac);
///             // simulate long running task:
///             std::thread::sleep(std::time::Duration::from_millis(1000));
///             response!("second message\n");
///             async_complete!();
///         });            
///     });
///
///     rustlet_mapping!("/async", "async");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! async_complete {
	() => {
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut *(f.borrow_mut()) {
			Some((_request, response)) => match response.async_complete() {
				Ok(_) => {}
				Err(e) => {
					mainlogerror!("async_complete generated error: {}", e.to_string());
				}
			},
			None => {
				mainlogerror!("Error: not in a rustlet context");
			}
		});
	};
}

/// Creates an async context which may be used to pass execution of the rustlet into
/// another thread thus allowing the rustlet thread pool to continue processing other
/// rustlets. The first call to [`async_context`] should not specify any parameters
/// and the second call should specify the returned parameter of the first call and be
/// in another thread. See the example below for details.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("async", {
///         response!("first message\n");
///         let ac = async_context!();
///
///         std::thread::spawn(move || {
///             async_context!(ac);
///             // simulate long running task:
///             std::thread::sleep(std::time::Duration::from_millis(1000));
///             response!("second message\n");
///             async_complete!();
///         });
///     });
///
///     rustlet_mapping!("/", "async");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! async_context {
	() => {{
		let mut ret: librustlet::RustletAsyncContext = librustlet::RustletAsyncContext {
			request: None,
			response: None,
		};

		librustlet::macros::LOCALRUSTLET.with(|f| match &mut *(f.borrow_mut()) {
			Some((request, response)) => {
				match response.set_is_async(true) {
					Ok(_) => {}
					Err(e) => {
						mainlogerror!("async_context generated error: {}", e.to_string());
					}
				}

				ret = librustlet::RustletAsyncContext {
					request: Some(request.clone()),
					response: Some(response.clone()),
				};
			}
			None => {
				mainlogerror!("Error: not in a rustlet context");
			}
		});
		ret
	}};
	($a:expr) => {
		librustlet::macros::LOCALRUSTLET.with(|f| match $a.request {
			Some(request) => match $a.response {
				Some(response) => {
					*f.borrow_mut() = Some((request.clone(), response.clone()));
				}
				None => {
					mainlogerror!("no response in local rustlet");
				}
			},
			None => {
				mainlogerror!("no request in local rustlet");
			}
		});
	};
}

/// Specifies a rustlet. Rustlets are closures that process HTTP requests and generate a response,
/// so variables can be moved into them and shared among other rustlets or any other closure.
/// Rustlets are processed in the [nioruntime](https://github.com/bitcoinmw/nioruntime). So, the
/// exectuion is performant. See the other macros for detailed examples on how to use all of the
/// functionality of rustlets. To try out a minimally simple rustlet as a project, see
/// [rustlet-simple](https://github.com/bitcoinmw/rustlet-simple/).
/// # Also see
///
/// * [`add_header`]
/// * [`async_complete`]
/// * [`async_context`]
/// * [`bin_write`]
/// * [`cookie`]
/// * [`flush`]
/// * [`header_len`]
/// * [`header_name`]
/// * [`header_value`]
/// * [`query`]
/// * [`request`]
/// * [`request_content`]
/// * [`response`]
/// * [`rustlet_init`]
/// * [`rustlet_mapping`]
/// * [`session`]
/// * [`session_delete`]
/// * [`set_content_type`]
/// * [`set_cookie`]
/// * [`set_redirect`]
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use std::sync::{Mutex, Arc};
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///     let count = Arc::new(Mutex::new(0));
///     let count_clone = count.clone();
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     // define our first rustlet
///     rustlet!("myrustlet", {
///         let mut count = count.lock().unwrap();
///         *count += 1;
///         response!("count in myrustlet={}", count);
///     });
///
///     // define our second rustlet
///     rustlet!("myrustlet2", {
///         let mut count = count_clone.lock().unwrap();
///         *count += 1;
///         response!("count in myrustlet2={}", count);
///     });
///
///     // define mappings to our rustlets
///     rustlet_mapping!("/myrustlet", "myrustlet");
///     rustlet_mapping!("/myrustlet2", "myrustlet2");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! rustlet {
	($a:expr,$b:expr) => {
		let mut container = librustlet::macros::RUSTLET_CONTAINER.write();
		match container {
			Ok(mut container) => {
				let res = container.add_rustlet(
					$a,
					Box::pin(
						move |request: &mut RustletRequest, response: &mut RustletResponse| {
							librustlet::macros::LOCALRUSTLET.with(|f| {
								*f.borrow_mut() = Some(((*request).clone(), (*response).clone()));
							});
							{
								$b
							}
							Ok(())
						},
					),
				);
				match res {
					Ok(_) => {}
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						nioruntime_log::log_multi!(
							nioruntime_log::ERROR,
							MAIN_LOG,
							"Error adding rustlet to container: {}",
							e.to_string()
						);
					}
				}
			}
			Err(e) => {
				const MAIN_LOG: &str = "mainlog";
				nioruntime_log::log_multi!(
					nioruntime_log::ERROR,
					MAIN_LOG,
					"Couldn't start rustlet: couldn't get lock: {}",
					e.to_string()
				);
			}
		}
	};
}

/// Initialize the rustlet container based on the specified configuration. The default
/// configuration may be used by calling `RustletConfig::default()`. See [`crate::RustletConfig`]
/// for details on configuring the Rustlet and Http containers.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init rustlet container with defaults
///     rustlet_init!(RustletConfig::default()); // use default config
///
///     rustlet!("hello_world", {
///         response!("Hello World\n");
///     });
///
///     rustlet_mapping!("/", "hello_world");
///
///     Ok(())
/// }
/// ```
///
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///     rustlet_init!(
///         RustletConfig {
///             http_config: HttpConfig {
///                 port: 80,
///                 ..HttpConfig::default()
///             },
///             ..RustletConfig::default()
///         }
///     );
///     rustlet!("hello_world", {
///         response!("Hello World\n");
///     });
///
///     rustlet_mapping!("/", "hello_world");
///
///     Ok(())
/// }           
/// ```
#[macro_export]
macro_rules! rustlet_init {
	($config:expr) => {{
		let mut container = librustlet::macros::RUSTLET_CONTAINER.write();

		match container {
			Ok(mut container) => {
				let res = container.set_config($config);
				match res {
					Ok(_) => {
						let res = container.start();
						match res {
							Ok(_) => {}
							Err(e) => {
								const MAIN_LOG: &str = "mainlog";
								nioruntime_log::log_multi!(
									nioruntime_log::ERROR,
									MAIN_LOG,
									"Couldn't start rustlet: start: {}",
									e.to_string()
								);
							}
						}
					}
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						nioruntime_log::log_multi!(
							nioruntime_log::ERROR,
							MAIN_LOG,
							"Couldn't start rustlet: set_config: {}",
							e.to_string()
						);
					}
				}
			}
			Err(e) => {
				const MAIN_LOG: &str = "mainlog";
				nioruntime_log::log_multi!(
					nioruntime_log::ERROR,
					MAIN_LOG,
					"Couldn't start rustlet: couldn't get lock: {}",
					e.to_string()
				);
			}
		}
	}};
}

/// Maps the specified uri to a rustlet. All requests to the container for this uri
/// will be processed by the specified rustlet.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///     rustlet_init!(
///         RustletConfig {
///             http_config: HttpConfig {
///                 port: 80,
///                 ..HttpConfig::default()
///             },
///             ..RustletConfig::default()
///         }
///     );
///     rustlet!("hello_world", {
///         response!("Hello World\n");
///     });
///     // maps the uri /hello to the rustlet "hello_world"
///     rustlet_mapping!("/hello", "hello_world");
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! rustlet_mapping {
	($a:expr, $b:expr) => {{
		let mut container = librustlet::macros::RUSTLET_CONTAINER.write();

		match container {
			Ok(mut container) => match container.add_rustlet_mapping($a, $b) {
				Ok(_) => {}
				Err(e) => {
					const MAIN_LOG: &str = "mainlog";
					nioruntime_log::log_multi!(
						nioruntime_log::ERROR,
						MAIN_LOG,
						"Couldn't start rustlet: add_mapping: {}",
						e.to_string()
					);
				}
			},
			Err(e) => {
				const MAIN_LOG: &str = "mainlog";
				nioruntime_log::log_multi!(
					nioruntime_log::ERROR,
					MAIN_LOG,
					"Couldn't start rustlet: couldn't get lock: {}",
					e.to_string()
				);
			}
		}
	}};
}

/// Sets the content-type header of this request.
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("set_content_type", {
///         set_content_type!("text/html");
///         response!("<html><body><strong>Some Content Here</strong></body></html>");
///     });
///
///     rustlet_mapping!("/", "set_content_type");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! set_content_type {
	($a:expr) => {
		add_header!("Content-Type", $a);
	};
}

/// Adds a header to the response for this rustlet. The first parameter is the name of the header
/// to set and the second parameter is the value of the header. See examples below.
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("set_content_type", {
///         add_header!("Cache-Control", "no-cache");
///         response!("<html><body><strong>Some Content Here</strong></body></html>");
///     });
///
///     rustlet_mapping!("/", "set_content_type");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! add_header {
	($a:expr, $b:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => {
				let res = response.add_header($a, $b);
				match res {
					Ok(_) => {}
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						nioruntime_log::log_multi!(
							nioruntime_log::ERROR,
							MAIN_LOG,
							"Couldn't call response.write: {}",
							e.to_string()
						);
					}
				}
			}
			None => {
				const MAIN_LOG: &str = "mainlog";
				nioruntime_log::log_multi!(
					nioruntime_log::ERROR,
					MAIN_LOG,
					"Couldn't find response struct",
				);
			}
		});
	}};
}

/// Sets a redirect to another URL. The 301 HTTP Response is used for the redirect.
/// See example below.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("set_redirect", {
///         set_redirect!("http://www.example.com");
///     });
///
///     rustlet_mapping!("/", "set_redirect");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! set_redirect {
	($a:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => {
				let res = response.set_redirect($a);
				match res {
					Ok(_) => {}
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						nioruntime_log::log_multi!(
							nioruntime_log::ERROR,
							MAIN_LOG,
							"Couldn't call response.write: {}",
							e.to_string()
						);
					}
				}
			}
			None => {
				const MAIN_LOG: &str = "mainlog";
				nioruntime_log::log_multi!(
					nioruntime_log::ERROR,
					MAIN_LOG,
					"Couldn't find response struct",
				);
			}
		});
	}};
}

/// Writes a binary response to the client. The parameter must be a byte array.
/// Note that: data written via bin_write is buffered and is not necessarily sent immidiately.
/// To ensure all data is written, the user must call the [`flush`] macro.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("bin_write", {
///         bin_write!("test of bin write".as_bytes());
///     });
///
///     rustlet_mapping!("/", "bin_write");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! bin_write {
	($a:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => {
				let res = response.write($a);
				match res {
					Ok(_) => {}
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						nioruntime_log::log_multi!(
							nioruntime_log::ERROR,
							MAIN_LOG,
							"Couldn't call response.write: {}",
							e.to_string()
						);
					}
				}
			}
			None => {
				const MAIN_LOG: &str = "mainlog";
				nioruntime_log::log_multi!(
					nioruntime_log::ERROR,
					MAIN_LOG,
					"Couldn't find response struct",
				);
			}
		});
	}};
}

/// Writes a formated response to the client. The formatting is the same formatting as
/// the format! macro, as that is used internally to format the response. Note that
/// data written via response is buffered and is not necessarily sent immidiately.
/// To ensure all data is written, the user must call the [`flush`] macro.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("hello_world", {
///         response!("hello world {}", "any formatted value can go here");
///     });
///
///     rustlet_mapping!("/", "hello_world");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! response {
	($a:expr)=>{
                {
			librustlet::macros::LOCALRUSTLET.with(|f| {
				match &mut (*f.borrow_mut()) {
					Some((request,response)) => {
						let res = response.write(format!($a).as_bytes());
						match res {
							Ok(_) => {},
							Err(e) => {
								const MAIN_LOG: &str = "mainlog";
								nioruntime_log::log_multi!(
									nioruntime_log::ERROR,
									MAIN_LOG,
									"Couldn't call response.write: {}",
									e.to_string()
								);
							},
						}
					},
					None => {
						const MAIN_LOG: &str = "mainlog";
                                        	nioruntime_log::log_multi!(
                                                	nioruntime_log::ERROR,
                                                	MAIN_LOG,
                                                	"Couldn't find response struct",
                                        	);
					},
				}
			});
		}
	};
	($a:expr,$($b:tt)*)=>{
		{
                        librustlet::macros::LOCALRUSTLET.with(|f| {
                                match &mut (*f.borrow_mut()) {
                                        Some((request,response)) => {
                                                let res = response.write(format!($a, $($b)*).as_bytes());
                                                match res {
                                                        Ok(_) => {},
                                                        Err(e) => {
								mainlogerror!(
									"Couldn't call response.write: {}",
									e.to_string(),
								);
                                                        },
                                                }
                                        },
                                        None => {
						mainlogerror!("Couldn't find response struct");
                                        },
                                }
                        });
		}

	};
}

/// Returns the content of the message body of the HTTP request.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("show_content_as_utf8", {
///         let content = request_content!();
///         let content_as_ut8 = std::str::from_utf8(&content)?;
///         response!("content='{}'\n", content_as_ut8);
///     });
///
///     rustlet_mapping!("/", "show_content_as_utf8");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! request_content {
	() => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &(*f.borrow()) {
			Some((request, response)) => match request.get_content() {
				Ok(content) => content,
				Err(e) => {
					mainlogerror!("unexpected error get_content generated: {}", e.to_string());
					vec![]
				}
			},
			None => {
				mainlogerror!("unexpected error no request/response found");
				vec![]
			}
		})
	}};
}

/// Get the value of the specified cookie. To set cookies, see [`set_cookie`].
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("cookies", {
///         let cookie = cookie!("abc");
///         set_cookie!("abc", "def");
///         response!("cookie={:?}\n", cookie);
///     });
///
///     rustlet_mapping!("/", "cookies");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! cookie {
	($a:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => match request.get_cookie($a) {
				Ok(cookie) => cookie,
				Err(e) => {
					mainlogerror!("unexpected error getting cookie: {}", e.to_string());
					None
				}
			},
			None => {
				mainlogerror!("unexpected error no request/response found");
				None
			}
		})
	}};
}

/// Set the value of the specified cookie. To get cookies, see [`cookie`].
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///             
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("cookies", {
///         let cookie = cookie!("abc");
///         set_cookie!("abc", "def");
///         response!("cookie={:?}\n", cookie);
///     });
///
///     rustlet_mapping!("/", "cookies");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! set_cookie {
	($a:expr,$b:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => match response.set_cookie($a, $b, "") {
				Ok(_) => {}
				Err(e) => {
					mainlogerror!("error setting cookie: {}", e.to_string());
				}
			},
			None => {
				mainlogerror!("unexpected error no request/response found");
			}
		})
	}};
	($a:expr,$b:expr,$c:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => match response.set_cookie($a, $b, $c) {
				Ok(_) => {}
				Err(e) => {
					mainlogerror!("error setting cookie: {}", e.to_string());
				}
			},
			None => {
				mainlogerror!("unexpected error no request/response found");
			}
		})
	}};
}

/// Returns the number of headers sent in this HTTP request.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("header_len", {
///         for i in 0..header_len!() {
///             let header_name = header_name!(i);
///             let header_value = header_value!(i);
///             response!("header[{}] [{}] -> [{}]\n", i, header_name, header_value);
///         }
///     });
///
///     rustlet_mapping!("/", "header_len");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! header_len {
	() => {{
		let res: usize = request!("header_len").parse().unwrap_or(0);
		res
	}};
}

/// Returns the header name for the specified index.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("header_name", {
///         for i in 0..header_len!() {
///             let header_name = header_name!(i);
///             let header_value = header_value!(i);
///             response!("header[{}] [{}] -> [{}]\n", i, header_name, header_value);
///         }
///     });
///
///     rustlet_mapping!("/", "header_name");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! header_name {
	($a:expr) => {{
		request!("header_i_name", &format!("{}", $a))
	}};
}

/// Returns the header value for the specified index.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("header_value", {
///         for i in 0..header_len!() {
///             let header_name = header_name!(i);
///             let header_value = header_value!(i);
///             response!("header[{}] [{}] -> [{}]\n", i, header_name, header_value);
///         }
///     });
///
///     rustlet_mapping!("/", "header_value");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! header_value {
	($a:expr) => {{
		request!("header_i_value", &format!("{}", $a))
	}};
}

/// Get the value of the specified query parameter. Parsing is done with
/// the [`querystring`](https://docs.rs/querystring/1.1.0/querystring/) library.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("query", {
///         let query = request!("query"); // the full query for the request
///         response!("query='{}'\n", query);
///         let email = request!("query", "email"); // get a specific value associated with the key. In this case "email"
///         response!("email='{}'\n", email);
///     });
///
///     rustlet_mapping!("/", "query");
///
///     Ok(())
/// }           
/// ```
#[macro_export]
macro_rules! query {
	($a:expr) => {{
		request!("query", $a)
	}};
}

/// Get data from the request for this rustlet.
/// See the example below for possible values of the request parameter.
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
///
/// fn test() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     rustlet!("request", {
///         let method = request!("method"); // the HTTP request method (GET or POST).
///         response!("method='{}'\n", method);
///         let version = request!("version"); // the HTTP version 0.9, 1.0, 1.1, or 2.0
///         response!("http version='{}'\n", version);
///         let uri = request!("uri"); // the request URI.
///         response!("uri='{}'\n", uri);
///         let unknown = request!("blah"); // this shows that calling an invalid value returns ''
///         response!("blah (should be empty)='{}'\n", unknown);
///         let query = request!("query"); // the full query for the request
///         response!("query='{}'\n", query);
///     });
///
///     rustlet_mapping!("/", "request");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! request {
	($a:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => {
				let value = $a.to_lowercase();
				if value == "query" {
					request.get_query().unwrap_or("".to_string())
				} else if value == "method" {
					match request
						.get_http_method()
						.unwrap_or(nioruntime_http::HttpMethod::Get)
					{
						nioruntime_http::HttpMethod::Get => "GET".to_string(),
						nioruntime_http::HttpMethod::Post => "POST".to_string(),
					}
				} else if value == "version" {
					match request
						.get_http_version()
						.unwrap_or(nioruntime_http::HttpVersion::V10)
					{
						nioruntime_http::HttpVersion::V09 => "V09".to_string(),
						nioruntime_http::HttpVersion::V10 => "V10".to_string(),
						nioruntime_http::HttpVersion::V11 => "V11".to_string(),
						nioruntime_http::HttpVersion::V20 => "V20".to_string(),
					}
				} else if value == "header_len" {
					format!("{}", request.get_header_len().unwrap_or(0))
				} else if value == "uri" {
					request.get_uri().unwrap_or("".to_string())
				} else {
					mainlogerror!("unknown parameter: '{}'", $a);
					"".to_string()
				}
			}
			None => {
				mainlogerror!("unexpected error no request/response found");
				"".to_string()
			}
		})
	}};
	($a:expr,$b:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => {
				let value = $a.to_lowercase();
				if value == "query" {
					let qp = request.get_query_parameter($b);
					match qp {
						Ok(qp) => match qp {
							Some(qp) => qp,
							None => "".to_string(),
						},
						Err(e) => {
							mainlogerror!("query error: {}", e);
							"".to_string()
						}
					}
				} else if value == "header_i_name" {
					let usize_value: usize = $b.parse().unwrap_or(usize::MAX);
					match request.get_header_i_name(usize_value) {
						Ok(name) => name,
						Err(e) => {
							mainlogerror!("header_i_name error: {}", e);
							"".to_string()
						}
					}
				} else if value == "header_i_value" {
					let usize_value: usize = $b.parse().unwrap_or(usize::MAX);
					match request.get_header_i_value(usize_value) {
						Ok(name) => name,
						Err(e) => {
							mainlogerror!("header_i_name error: {}", e);
							"".to_string()
						}
					}
				} else if value == "header" {
					let header = request.get_header($b);
					match header {
						Ok(header) => match header {
							Some(header) => header,
							None => "".to_string(),
						},
						Err(e) => {
							mainlogerror!("header error: {}", e);
							"".to_string()
						}
					}
				} else {
					"".to_string()
				}
			}
			None => {
				mainlogerror!("unexpected error no request/response found");
				"".to_string()
			}
		})
	}};
}

/// Internal macro used to log to the main log. Applications should use the default logger (or another
/// user specified logger). See [`nioruntime_log`] for details on logging.
#[macro_export]
macro_rules! mainlogerror {
	($a:expr) => {{
		const MAIN_LOG: &str = "mainlog";
		nioruntime_log::log_multi!(
			nioruntime_log::ERROR,
			MAIN_LOG,
			$a,
		);
	}};
	($a:expr,$($b:tt)*)=>{{
                const MAIN_LOG: &str = "mainlog";
                nioruntime_log::log_multi!(
                        nioruntime_log::ERROR,
                        MAIN_LOG,
                        $a,
			$($b)*
                );
	}};

}
