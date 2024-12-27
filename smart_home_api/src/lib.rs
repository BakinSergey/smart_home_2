pub mod command;
pub mod home_client;
pub mod info_provider;
pub mod json_rpc;
pub mod my_smart_home;
pub mod smart_device;

pub const DEFAULT_TCP_SOCKET: &str = "127.0.0.1:54321";
pub const DEFAULT_UDP_SOCKET: &str = "127.0.0.1:55555";
