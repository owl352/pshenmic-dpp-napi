use napi::bindgen_prelude::{BigInt, Null};
use napi_derive::napi;

#[derive(Debug)]
#[napi(js_name = "DynamicValue")]
pub enum DynamicValue {
    Text(String),
    Bytes(Vec<u8>),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(BigInt),
    Bool(bool),
    Null(Null)
}