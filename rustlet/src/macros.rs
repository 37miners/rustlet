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

use crate::rustlet_impls::SockletContainer;
use crate::RustletContainer;
use crate::{RustletRequest, RustletResponse};
use lazy_static::lazy_static;
use nioruntime_http::ConnData;
use nioruntime_http::WebSocketMessage;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use std::thread_local;

/// This is the enum that the [`event`] macro will return. It is
/// used to determine what type of event has occured in a socklet.
///
/// # Also see
///
/// * [`event`]
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
/// fn testsocklet() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     socklet!("mysocklet", {
///         match event!()? {
///             Socklet::Open => {
///                 // process open events for the websocket
///             },
///             Socklet::Close => {
///                 // process close events for the websocket
///             },
///             Socklet::Text => {
///                 // process text messages
///             },
///             Socklet::Binary => {
///                 // process binary messages
///             },
///             Socklet::Ping => {
///                 // process pings
///             },
///             Socklet::Pong => {
///                 // process pongs
///             },
///         }
///     });
///
///     socklet_mapping!("/mysocklet", "mysocklet");
///
///     Ok(())
/// }
/// ```
pub enum Socklet {
	/// This event occurs whenever a websocket is opened
	/// and connects to the uri associated with the socketlet mapping.
	Open,
	/// This event occurs whenver the websocket connection closes.
	/// Duplicates are filtered.
	Close,
	/// This event occurs whenever a text message is sent to the websocket.
	Text,
	/// This event occurs whenever a binary message is sent to the websocket.
	Binary,
	/// This event occurs whenever a ping is sent to the websocket.
	Ping,
	/// This event occurs whenever a pong is sent to the websocket.
	Pong,
}

thread_local!(
	pub static LOCALRUSTLET: RefCell<
		Option<
			(RustletRequest,RustletResponse)
		>
	> = RefCell::new(None)
);

thread_local!(
	pub static LOCALSOCKLET: RefCell<
		Option<
			(WebSocketMessage,ConnData)
		>
	> = RefCell::new(None)
);

lazy_static! {
	pub static ref RUSTLET_CONTAINER: Arc<RwLock<RustletContainer>> =
		Arc::new(RwLock::new(RustletContainer::new()));
	pub static ref SOCKLET_CONTAINER: Arc<RwLock<SockletContainer>> =
		Arc::new(RwLock::new(SockletContainer::new()));
}

/// Prints a smurf to the page.
/// All credit for the ascii art goes to Normand Veilleux.
///
/// # Examples
///
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
///     rustlet!("smurf", {
///         smurf!();
///     });
///
///     rustlet_mapping!("/smurf", "smurf");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! smurf {
	() => {
		response!("<pre>");
		response!("                             _,dP\"''    `\"\"\"\"Ya,_\n");
		response!("                          ,aP\"'                `\"Yb,_\n");
		response!("                        ,8\"'                       `\"8a,\n");
		response!("                      ,8\"                             `\"8,_\n");
		response!("                    ,8\"                                  \"Yb,\n");
		response!("                  ,8\"                                      `8,\n");
		response!("                 dP'                                        8I\n");
		response!("               ,8\"                           bg,_          ,P'\n");
		response!("              ,8'                              \"Y8\"Ya,,,,ad\"\n");
		response!("             ,d\"                            a,_ I8   `\"\"\"'\n");
		response!("            ,8'                              \"\"888\n");
		response!("            dP     __                           `Yb,\n");
		response!("           dP'  _,d8P::::Y8b,                     `Ya\n");
		response!("      ,adba8',d88P::;;::;;;:\"b:::Ya,_               Ya\n");
		response!("     dP\":::\"Y88P:;P\"\"\"YP\"\"\"Yb;::::::\"Ya,             \"Y,\n");
		response!("     8:::::::Yb;d\" _  \"_    dI:::::::::\"Yb,__,,gd88ba,db\n");
		response!("     Yb:::::::\"8(,8P _d8   d8:::::::::::::Y88P\"::::::Y8I\n");
		response!("     `Yb;:::::::\"\"::\"\":b,,dP::::::::::::::::::;aaa;:::8(\n");
		response!("       `Y8a;:::::::::::::::::::::;;::::::::::8P\"\"Y8)::8I\n");
		response!("         8b\"ba::::::::::::::::;adP:::::::::::\":::dP::;8'\n");
		response!("         `8b;::::::::::::;aad888P::::::::::::::;dP::;8'\n");
		response!("          `8b;::::::::\"\"\"\"88\"  d::::::::::b;:::::;;dP'\n");
		response!("            \"Yb;::::::::::Y8bad::::::::::;\"8Paaa\"\"'\n");
		response!("              `\"Y8a;;;:::::::::::::;;aadP\"\"\n");
		response!("                  ``\"\"Y88bbbdddd88P\"\"8b,\n");
		response!("                           _,d8\"::::::\"8b,\n");
		response!("                         ,dP8\"::::::;;:::\"b,\n");
		response!("                       ,dP\"8:::::::Yb;::::\"b,\n");
		response!("                     ,8P:dP:::::::::Yb;::::\"b,\n");
		response!("                  _,dP:;8\":::::::::::Yb;::::\"b\n");
		response!("        ,aaaaaa,,d8P:::8\":::::::::::;dP:::::;8\n");
		response!("     ,ad\":;;:::::\"::::8\"::::::::::;dP::::::;dI\n");
		response!("    dP\";adP\":::::;:;dP;::::aaaad88\"::::::adP:8b,___\n");
		response!("   d8:::8;;;aadP\"::8'Y8:d8P\"::::::::::;dP\";d\"'`Yb:\"b\n");
		response!("   8I:::;\"\"\":::::;dP I8P\"::::::::::;a8\"a8P\"     \"b:P\n");
		response!("   Yb::::\"8baa8\"\"\"'  8;:;d\"::::::::d8P\"'         8\"\n");
		response!("    \"YbaaP::8;P      `8;d::;a::;;;;dP           ,8\n");
		response!("       `\"Y8P\"'         Yb;;d::;aadP\"           ,d'\n");
		response!("                        \"YP:::\"P'             ,d'\n");
		response!("                          \"8bdP'    _        ,8'\n");
		response!("         Normand         ,8\"`\"\"Yba,d\"      ,d\"\n");
		response!("         Veilleux       ,P'     d\"8'     ,d\"\n");
		response!("                       ,8'     d'8'     ,P'\n");
		response!("                       (b      8 I      8,\n");
		response!("                        Y,     Y,Y,     `b,\n");
		response!("                  ____   \"8,__ `Y,Y,     `Y\"\"b,\n");
		response!("              ,adP\"\"\"\"b8P\"\"\"\"\"\"\"\"Ybdb,        Y,\n");
		response!("            ,dP\"    ,dP'            `\"\"       `8\n");
		response!("           ,8\"     ,P'                        ,P\n");
		response!("           8'      8)                        ,8'\n");
		response!("           8,      Yb                      ,aP'\n");
		response!("           `Ya      Yb                  ,ad\"'\n");
		response!("             \"Ya,___ \"Ya             ,ad\"'\n");
		response!("               ``\"\"\"\"\"\"`Yba,,,,,,,adP\"'\n");
		response!("                          `\"\"\"\"\"\"\"'\n");
		response!("</pre>");
	};
}

/// Delete the entire session. See [`session`] for more info on sessions. If a parameter is specified,
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
///         let val: u32 = query!("abc").unwrap_or("".to_string()).parse()?;
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
///         let val: u32 = query!("abc").unwrap_or("".to_string()).parse()?;
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

/// Create a signature using dalek algorithm. This is useful with the pubkey!() and verify!() macros.
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
///         let pubkey = pubkey!();
///         response!("pubkey = {:?}", pubkey);
///         let message = "abc".as_bytes();
///         let signature = sign!(message).unwrap();
///         let verification = verify!(message, pubkey, signature);
///         response!("verification = {}", verification.unwrap());
///         response!("</strong></body></html>");
///     });
///     
///     rustlet_mapping!("/set_content_type", "set_content_type");
///     
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! sign {
	($a:expr) => {{
		let mut container =
			librustlet::nioruntime_util::lockw!(librustlet::macros::RUSTLET_CONTAINER);
		match container {
			Ok(mut container) => {
				let res = container.tor_sign($a);
				match res {
					Ok(signature) => Some(signature),
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						nioruntime_log::log_multi!(
							nioruntime_log::ERROR,
							MAIN_LOG,
							"Error signing: {}",
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

/// Verifies a signature using dalek algorithm. This is useful with the pubkey!() and sign!() macros.
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
///         let pubkey = pubkey!();
///         response!("pubkey = {:?}", pubkey);
///         let message = "abc".as_bytes();
///         let signature = sign!(message).unwrap();
///         let verification = verify!(message, pubkey, signature);
///         response!("verification = {}", verification.unwrap());
///         response!("</strong></body></html>");
///     });
///
///     rustlet_mapping!("/set_content_type", "set_content_type");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! verify {
	($a:expr, $b:expr, $c:expr) => {{
		let mut container =
			librustlet::nioruntime_util::lockw!(librustlet::macros::RUSTLET_CONTAINER);
		match container {
			Ok(mut container) => {
				let res = container.verify($a, $b, $c);
				match res {
					Ok(b) => Some(b),
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						nioruntime_log::log_multi!(
							nioruntime_log::ERROR,
							MAIN_LOG,
							"Error signing: {}",
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

/// Gets the tor pubkey associated with this rustlet container. Returns a Result with the
/// byte array. If tor is not congfigured, behavior is undefined.
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
///     rustlet!("pubkey", {
///         set_content_type!("text/html");
///         response!("<html><body><strong>");
///         let pubkey = pubkey!();
///         response!("tor pubkey = {:?}", pubkey);
///         response!("</strong></body></html>");
///     });
///
///     rustlet_mapping!("/pubkey", "pubkey");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! pubkey {
	() => {{
		let mut container =
			librustlet::nioruntime_util::lockw!(librustlet::macros::RUSTLET_CONTAINER);
		match container {
			Ok(mut container) => {
				let res = container.get_onion_address_pubkey();
				match res {
					Ok(onion_pubkey) => match onion_pubkey {
						Some(onion_pubkey) => onion_pubkey,
						None => {
							const MAIN_LOG: &str = "mainlog";
							nioruntime_log::log_multi!(
								nioruntime_log::ERROR,
								MAIN_LOG,
								"Error getting onion pubkey: not found",
							);
							[0u8; 32]
						}
					},
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						nioruntime_log::log_multi!(
							nioruntime_log::ERROR,
							MAIN_LOG,
							"Error getting onion pubkey: {}",
							e.to_string()
						);
						[0u8; 32]
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
				[0u8; 32]
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
/// Rustlets are processed in the [nioruntime](https://github.com/37miners/nioruntime). So, the
/// exectuion is performant. See the other macros for detailed examples on how to use all of the
/// functionality of rustlets. To try out a minimally simple rustlet as a project, see
/// [rustlet-simple](https://github.com/37miners/rustlet-simple/).
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
		let mut container =
			librustlet::nioruntime_util::lockw!(librustlet::macros::RUSTLET_CONTAINER);
		match container {
			Ok(mut container) => {
				let res = (*container).add_rustlet(
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

/// Maps the specified uri to a socklet. This works similarly to how the
/// [`rustlet_mapping`] macro works. All websocket requests to the container
/// for this uri will be processed by the specified socklet.
///
/// # Also see
///
/// * [`rustlet_init`]
/// * [`socklet`]
/// * [`handle`]
/// * [`event`]
/// * [`text`]
/// * [`ping`]
/// * [`pong`]
/// * [`binary`]
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
///
///     socklet!("mysocklet", {
///         let handle = handle!()?;
///         let id = handle.get_connection_id();
///         match event!()? {
///             Socklet::Text => {
///                 let text = text!()?;
///                 info!("got text [cid={}]: {}", id, text);
///                 text!(handle, "echo [cid={}]: '{}'", id, text);
///             }
///             _ => {},
///         }
///     });
///
///     socklet_mapping!("/mysocklet", "mysocklet");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! socklet_mapping {
	($a:expr, $b:expr) => {{
		let mut container =
			librustlet::nioruntime_util::lockw!(librustlet::macros::SOCKLET_CONTAINER);
		match container {
			Ok(mut container) => match container.add_socklet_mapping($a, $b) {
				Ok(_) => {}
				Err(e) => {
					const MAIN_LOG: &str = "mainlog";
					nioruntime_log::log_multi!(
						nioruntime_log::ERROR,
						MAIN_LOG,
						"Couldn't add socklet mapping: {}",
						e.to_string()
					);
				}
			},
			Err(e) => {
				const MAIN_LOG: &str = "mainlog";
				nioruntime_log::log_multi!(
					nioruntime_log::ERROR,
					MAIN_LOG,
					"Couldn't get lock for socklets: {}",
					e.to_string()
				);
			}
		}
	}};
}

/// Send a websocket ping message. See <https://datatracker.ietf.org/doc/html/rfc6455> for
/// more details on the websocket protocol and the usage of pings. Note that most browsers
/// do not support this message type.
///
/// # Also see
///
/// * [`rustlet_init`]
/// * [`socklet`]
/// * [`socklet_mapping`]
/// * [`handle`]
/// * [`event`]
/// * [`text`]
/// * [`pong`]
/// * [`binary`]
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
///
///     socklet!("mysocklet", {
///         let handle = handle!()?;
///         let id = handle.get_connection_id();
///         match event!()? {
///             Socklet::Text => {
///                 let text = text!()?;
///                 info!("got text [cid={}]: {}", id, text);
///                 text!(handle, "echo [cid={}]: '{}'", id, text);
///                 ping!(handle);
///             }
///             _ => {},
///         }
///     });
///
///     socklet_mapping!("/mysocklet", "mysocklet");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ping {
	($a:expr) => {
		send_websocket_message(
			&$a,
			&WebSocketMessage {
				mtype: WebSocketMessageType::Ping,
				payload: vec![],
				mask: false,
				header_info: None,
			},
		)?;
	};
}

/// Send a websocket pong message. See <https://datatracker.ietf.org/doc/html/rfc6455> for
/// more details on the websocket protocol and the usage of pongs. Note that most browsers
/// do not support this message type.
///
/// # Also see
///
/// * [`rustlet_init`]
/// * [`socklet`]
/// * [`socklet_mapping`]
/// * [`handle`]
/// * [`event`]
/// * [`text`]
/// * [`ping`]
/// * [`binary`]
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
///
///     socklet!("mysocklet", {
///         let handle = handle!()?;
///         let id = handle.get_connection_id();
///         match event!()? {
///             Socklet::Text => {
///                 let text = text!()?;
///                 info!("got text [cid={}]: {}", id, text);
///                 text!(handle, "echo [cid={}]: '{}'", id, text);
///                 ping!(handle);
///             },
///             Socklet::Ping => {
///                 pong!(handle);
///             },
///             _ => {},
///         }
///     });
///
///     socklet_mapping!("/mysocklet", "mysocklet");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! pong {
	($a:expr) => {
		send_websocket_message(
			&$a,
			&WebSocketMessage {
				mtype: WebSocketMessageType::Pong,
				payload: vec![],
				mask: false,
				header_info: None,
			},
		)?;
	};
}

/// The binary macro is used to send and receive binary websocket messages.
/// If no parameters are specified, this macro will return the payload
/// of the message that is received from a [`Socklet::Binary`] [`event`].
/// The payload, which is a Vec<u8> is wrapped in a [`std::result::Result`] enum and an
/// error may be returned in the event of a misconfiguration. If two
/// parameters are specified, this macro will write data (second parameter)
/// to the handle (first parameter). The data is of type [`std::slice`].
///
/// # Also see
///
/// * [`rustlet_init`]
/// * [`socklet`]
/// * [`socklet_mapping`]
/// * [`handle`]
/// * [`event`]
/// * [`text`]
/// * [`ping`]
/// * [`pong`]
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
///
///     socklet!("mysocklet", {
///         let handle = handle!()?;
///         let id = handle.get_connection_id();
///         match event!()? {
///             Socklet::Binary => {
///                 let binary = binary!()?;
///                 info!("got binary on [cid={}]: {:?}", id, binary);
///                 let bin_resp = &[1,2,3,4,5];
///                 binary!(handle, bin_resp);
///             }
///             _ => {},
///         }
///     });
///
///     socklet_mapping!("/mysocklet", "mysocklet");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! binary {
	() => {{
		librustlet::macros::LOCALSOCKLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((message, _conn_data)) => {
				let payload = message.payload.clone();
				Ok(payload)
			}
			None => {
				let error: Error = ErrorKind::ApplicationError(
					"no thread local message variable. Are you not in a socklet?".to_string(),
				)
				.into();
				Err(error)
			}
		})
	}};
	($a:expr,$b:expr) => {{
		send_websocket_message(
			&$a,
			&WebSocketMessage {
				mtype: WebSocketMessageType::Binary,
				payload: $b.to_vec(),
				mask: false,
				header_info: None,
			},
		)?;
	}};
}

/// The text macro is used to send and receive text websocket messages.
/// If no parameters are specified, this macro will return the payload
/// of the message that is received from a [`Socklet::Text`] [`event`].
/// The payload, which is a [`std::string::String`] is wrapped in
/// a [`std::result::Result`] enum and an error may be returned in the event
/// of a misconfiguration. If two parameters are specified, this macro will
/// write data (second parameter) to the handle (first parameter).
/// The data is of type [`std::str`].
///
/// # Also see
///
/// * [`rustlet_init`]
/// * [`socklet`]
/// * [`socklet_mapping`]
/// * [`handle`]
/// * [`event`]
/// * [`binary`]
/// * [`ping`]
/// * [`pong`]
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
///
///     socklet!("mysocklet", {
///         let handle = handle!()?;
///         let id = handle.get_connection_id();
///         match event!()? {
///             Socklet::Text => {
///                 let text = text!()?;
///                 info!("got text on [cid={}]: {:?}", id, text);
///                 let text_resp = format!("echo: {}", text);
///                 text!(handle, text_resp);
///             }
///             _ => {},
///         }
///     });
///
///     socklet_mapping!("/mysocklet", "mysocklet");
///
///     Ok(())
/// }
#[macro_export]
macro_rules! text {
	() => {
		{
			librustlet::macros::LOCALSOCKLET.with(|f| {
				match &mut (*f.borrow_mut()) {
					Some((message,_conn_data)) => {
						Ok(std::str::from_utf8(&message.payload[..])?.to_string())
					},
					None => {
						let error: Error = ErrorKind::ApplicationError(
                                        		"no thread local message variable. Are you not in a socklet?"
                                        		.to_string()
                                		).into();
						Err(error)
					},
				}
			})
		}
	};
        ($a:expr,$b:expr)=>{
                {
			send_websocket_message(
				& $a,
				&WebSocketMessage {
					mtype: WebSocketMessageType::Text,
					payload: $b.as_bytes().to_vec(),
					mask: false,
					header_info: None,
				}
			)?;
                }
        };
        ($a:expr,$b:expr,$($c:tt)*)=>{
                {
			send_websocket_message(
				& $a,
				&WebSocketMessage {
					mtype: WebSocketMessageType::Text,
					payload: format!($b, $($c)*).as_bytes().to_vec(),
					mask: false,
					header_info: None,
				}
			)?;
		}
	};
}

/// This macro is called to determine which type of WebSocket event
/// has occured within a ['socklet'] closure. The macro returns a [`Socklet`] enum.
/// which can be used to determine what action should be taken by the socklet.
///
/// # Also see
///
/// * [`Socklet`]
///
/// # Examples
/// ```
/// use nioruntime_err::Error;
/// use librustlet::*;
/// use nioruntime_log::*;
///
/// debug!();
/// fn testsocklet() -> Result<(), Error> {
///
///     // init the rustlet container, in this case with default values
///     rustlet_init!(RustletConfig::default());
///
///     socklet!("mysocklet", {
///         match event!()? {
///             Socklet::Open => {
///                 // process open events for the websocket
///             },
///             Socklet::Close => {
///                 // process close events for the websocket
///             },
///             Socklet::Text => {
///                 // process text messages
///             },
///             Socklet::Binary => {
///                 // process binary messages
///             },
///             Socklet::Ping => {
///                 // process pings
///             },
///             Socklet::Pong => {
///                 // process pongs
///             },
///         }
///     });
///
///     socklet_mapping!("/mysocklet", "mysocklet");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! event {
	() => {{
		librustlet::macros::LOCALSOCKLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((message, conn_data)) => Ok(match message.mtype {
				WebSocketMessageType::Open => Socklet::Open,
				WebSocketMessageType::Close => Socklet::Close,
				WebSocketMessageType::Ping => Socklet::Ping,
				WebSocketMessageType::Pong => Socklet::Pong,
				WebSocketMessageType::Text => Socklet::Text,
				WebSocketMessageType::Binary => Socklet::Binary,
			}),
			None => {
				let error: Error = ErrorKind::ApplicationError(
					"no thread local message variable. Are you not in a socklet?".to_string(),
				)
				.into();
				return Err(error);
			}
		})
	}};
}

/// The handle macro reutrns a handle that can be used to reply to any websocket.
/// It is important to note that it need not be used just for the websocket that
/// is currently being processed, but the handle can be stored and used by any thread
/// to respond to websocket events. The handle is a mutable reference to a
/// [`nioruntime_http::ConnData`] struct. To get a hashable element from this
/// struct, the [`nioruntime_http::ConnData::get_connection_id`] function may be
/// called. That id, which is a u128 can be used to store any data about this
/// connection, including the handle itself in a collection.
///
/// # Also see
///
/// * [`rustlet_init`]
/// * [`socklet`]
/// * [`socklet_mapping`]
/// * [`event`]
/// * [`text`]
/// * [`binary`]
/// * [`ping`]
/// * [`pong`]
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
///     
///     socklet!("mysocklet", {
///         let handle = handle!()?;
///         let id = handle.get_connection_id();
///         match event!()? {
///             Socklet::Text => {
///                 let text = text!()?;
///                 info!("got text on [cid={}]: {:?}", id, text);
///                 let text_resp = format!("echo: {}", text);
///                 text!(handle, text_resp);
///             }
///             _ => {},
///         }
///     });
///     
///     socklet_mapping!("/mysocklet", "mysocklet");
///     
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! handle {
	() => {{
		librustlet::macros::LOCALSOCKLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((message, conn_data)) => Ok(conn_data.clone()),
			None => {
				let error: Error = ErrorKind::ApplicationError(
					"no thread local conn_data variable. Are you not in a socklet?".to_string(),
				)
				.into();
				return Err(error);
			}
		})
	}};
}

/// Specifies a socklet. Socklets are closures that are the equivalent to [`rustlet`]'s, except
/// they handle websockets instead of HTTP requests. Since, like rustlets, socklets are closures
/// variables can be moved into them and shared with other socklets and rustlets or any other
/// closure. Socklets are processed in the [nioruntime](https://github.com/37miners/nioruntime). So, the
/// exectuion is performant. See the other macros for detailed examples on how to use all of the
/// functionality of socklets.
///
/// # Also see
///
/// * [`rustlet_init`]
/// * [`handle`]
/// * [`socklet_mapping`]
/// * [`event`]
/// * [`text`]
/// * [`binary`]
/// * [`ping`]
/// * [`pong`]
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
///
///     socklet!("perfsocklet", {
///         let handle = handle!()?;
///         match event!()? {
///             Socklet::Binary => {
///                 let bin = binary!()?;
///                 binary!(handle, bin);
///             }
///             _ => {}
///         }
///     });
///
///     socklet_mapping!("/perfsocklet", "perfsocklet");
///
///     socklet!("mysocklet", {
///         let handle = handle!()?;
///         let id = handle.get_connection_id();
///         match event!()? {
///             Socklet::Open => {
///                 info!("socklet [cid={}] open!", id);
///             }
///             Socklet::Close => {
///                 info!("socklet [cid={}] close!", id);
///             }
///             Socklet::Text => {
///                 let text = text!()?;
///                 info!("got text [cid={}]: {}", id, text);
///                 text!(handle, "echo [cid={}]: '{}'", id, text,);
///             }
///             Socklet::Binary => {
///                 let bin = binary!()?;
///                 info!("got binary [cid={}]: {:?}", id, bin);
///                 binary!(handle, [0u8, 1u8, 2u8, 3u8]);
///                 if bin.len() > 0 && bin[0] == 100 {
///                     ping!(handle);
///                 }
///             }
///             Socklet::Ping => {
///                 pong!(handle);
///             }
///             Socklet::Pong => {}
///             }
///     });
///
///     socklet_mapping!("/mysocklet", "mysocklet");
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! socklet {
	($a:expr,$b:expr) => {
		let mut container =
			librustlet::nioruntime_util::lockw!(librustlet::macros::SOCKLET_CONTAINER);
		match container {
			Ok(mut container) => {
				let res = container.add_socklet(
					$a,
					Box::pin(
						move |message: &WebSocketMessage, conn_data: &mut ConnData| {
							librustlet::macros::LOCALSOCKLET.with(|f| {
								*f.borrow_mut() = Some(((*message).clone(), (*conn_data).clone()));
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
							"Error adding socklet to container: {}",
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
					"Couldn't start socklet: couldn't get lock: {}",
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
		let mut container =
			librustlet::nioruntime_util::lockw!(librustlet::macros::RUSTLET_CONTAINER)?;
		container.set_config($config)?;
		container.start()?;
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
		let mut container =
			librustlet::nioruntime_util::lockw!(librustlet::macros::RUSTLET_CONTAINER);
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
///         let name = query!("name"); // get the name parameter from the query
///         response!("name='{:?}'\n", name); // print name. query returns an option.
///                                           // If not specified, None is returned.
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
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => {
				let qp = request.get_query_parameter($a);
				match qp {
					Ok(qp) => qp,
					Err(e) => {
						mainlogerror!("query error: {}", e);
						None
					}
				}
			}
			None => {
				mainlogerror!("unexpected error no request/response found");
				None
			}
		})
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
