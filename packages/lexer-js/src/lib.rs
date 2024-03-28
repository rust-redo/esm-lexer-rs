#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use serde_json::{to_string};
use napi::bindgen_prelude::Buffer;
use esm_lexer::{ParsedData};


#[napi]
pub fn parse(code: Buffer) -> Buffer {
    let data = to_string(&ParsedData::new()).unwrap_or_default(); 

    Buffer::from(data)
}
