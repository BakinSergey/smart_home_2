use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct JsonRpcRequest {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: Value,
}
