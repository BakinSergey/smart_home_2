use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: String,
}

pub fn parse_error(data: String) -> JsonRpcError {
    JsonRpcError {
        code: -32700,
        message: "Parse error".to_string(),
        data,
    }
}

pub fn invalid_request(data: String) -> JsonRpcError {
    JsonRpcError {
        code: -32600,
        message: "Invalid Request".to_string(),
        data,
    }
}

pub fn invalid_method(data: String) -> JsonRpcError {
    JsonRpcError {
        code: -32601,
        message: "Invalid Method".to_string(),
        data,
    }
}

pub fn invalid_param(data: String) -> JsonRpcError {
    JsonRpcError {
        code: -32602,
        message: "Invalid Parameters of request".to_string(),
        data,
    }
}

pub fn internal_error(data: String) -> JsonRpcError {
    JsonRpcError {
        code: -32603,
        message: "Internal error".to_string(),
        data,
    }
}

pub fn api_error(data: String) -> JsonRpcError {
    JsonRpcError {
        code: 1,
        message: "Api logic error".to_string(),
        data,
    }
}

pub fn unhandled_error(data: String) -> JsonRpcError {
    JsonRpcError {
        code: 804,
        message: "unhandled error".to_string(),
        data,
    }
}
