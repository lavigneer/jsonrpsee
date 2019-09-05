//! Servers that receive JSON payloads.
//!
//! A "raw server" is a server that produces JSON payloads and can send back JSON payloads in
//! return. This module isn't concerned with concepts such as a "batch", "successes" and "failures".
//! All it does is accept and send back JSON data.
//!
//! ## Example usage
//!
//! ```
//! use jsonrpsee::raw_server::{RawServerRef, RawServerRq as _};
//! use jsonrpsee::types::{Error, Request, Response, Version};
//! 
//! async fn run_server<R>(mut server: R)
//!     where for<'r> &'r mut R: RawServerRef<'r>
//! {
//!     // Note that this implementation is a bit naive, as no request will be accepted by the
//!     // server while `request_to_response` is running. This is fine as long as building the
//!     // response is instantaneous (which is the case in this exampe), but probably isn't for
//!     // actual real-world usages.
//!     while let Ok(rq) = server.next_request().await {
//!         let response = request_to_response(rq.request()).await;
//!         let _ = rq.respond(&response).await;
//!     }
//! }
//!
//! async fn request_to_response(rq: &Request) -> Response {
//!     // ... to be implemented ...
//!     Response::from(Error::method_not_found(), Version::V2)
//! }
//! ```
//!

pub use self::http::HttpServer;
pub use self::traits::{RawServerRef, RawServerRq};

mod http;
mod traits;
