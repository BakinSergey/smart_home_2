use serde_json::Value;
use std::net::ToSocketAddrs;
use stp::client::StpClient;
use stp::error::{ConnectError, RequestError};

/// Клиент дома.
pub struct RpcOverStpClient {
    stp: StpClient,
}

impl RpcOverStpClient {
    /// Подключаемся к серверу.
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> Result<Self, ConnectError> {
        let stp = StpClient::connect(addr)?;
        Ok(Self { stp })
    }

    /// json rpc request-reply.
    pub fn rr(&mut self, json_req: Value) -> Result<String, RequestError> {
        let req = json_req.to_string();
        self.stp.send_request(req)
    }
}
