use std::io;
use thiserror;

/// Ошибка соединения.
#[derive(Debug, thiserror::Error)]
pub enum ConnectError {
    #[error("bad handshake")]
    BadHandshake,

    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Ошибка отправки сообщения.
#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Ошибка приема сообщения.
#[derive(Debug, thiserror::Error)]
pub enum RecvError {
    #[error("bad encoding")]
    BadEncoding,

    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Ошибка при обмене данными с сервером.
#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),

    #[error(transparent)]
    Recv(#[from] RecvError),
}
