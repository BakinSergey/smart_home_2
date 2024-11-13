extern crate erased_serde;

use crate::json_rpc::error::JsonRpcError;
use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonRpcErrorReply {
    pub jsonrpc: String,
    pub id: String,
    pub error: JsonRpcError,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Result {
    pub data: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct JsonRpcReply {
    pub jsonrpc: String,
    pub id: String,
    pub result: Result,
}

pub fn reply(id: String, result: String) -> JsonRpcReply {
    JsonRpcReply {
        id,
        jsonrpc: String::from("2.0"),
        result: Result { data: result },
    }
}

pub fn reply_error(id: String, error: JsonRpcError) -> JsonRpcErrorReply {
    JsonRpcErrorReply {
        id,
        jsonrpc: String::from("2.0"),
        error,
    }
}

pub trait JsonRpcReplyMsg: erased_serde::Serialize {}
serialize_trait_object!(JsonRpcReplyMsg);

impl JsonRpcReplyMsg for JsonRpcReply {}
impl JsonRpcReplyMsg for JsonRpcErrorReply {}
