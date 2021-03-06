use std::collections::HashMap;
use std::io::Cursor;

use serde::Serialize;
use serde_json;

use conduit::Response;

pub use self::errors::{bad_request, human, internal, internal_error, CargoError, CargoResult};
pub use self::errors::{std_error, ChainError};
pub use self::io_util::{read_fill, read_le_u32, LimitErrorReader};
pub use self::request_helpers::*;
pub use self::request_proxy::RequestProxy;

pub mod errors;
mod io_util;
mod request_helpers;
mod request_proxy;
pub mod rfc3339;

pub fn json_response<T: Serialize>(t: &T) -> Response {
    let json = serde_json::to_string(t).unwrap();
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".to_string(),
        vec!["application/json; charset=utf-8".to_string()],
    );
    headers.insert("Content-Length".to_string(), vec![json.len().to_string()]);
    Response {
        status: (200, "OK"),
        headers,
        body: Box::new(Cursor::new(json.into_bytes())),
    }
}
