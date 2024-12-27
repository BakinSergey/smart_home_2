use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum SmartHomeError {
    #[error("room: {0} not exist")]
    RoomNonExist(String),

    #[error("room with same name exist in home: {0}")]
    RoomSameNameExistInHome(String),

    #[error("device: {name:?} not exist in room: {room:?}")]
    NoDeviceInRoom { name: String, room: String },

    #[error("device with same name exist in room: {0}")]
    DeviceSameNameExistInRoom(String),
}

pub type SmartHomeResult<T> = Result<T, SmartHomeError>;
