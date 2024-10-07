use crate::devices::device::SmartDevice;

pub struct DeviceInfo<T> {
    pub device_stub: T,
}

impl<T: SmartDevice + Default> DeviceInfo<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { device_stub: T::default() }
    }
}

// pub struct DeviceInfo<T: SmartDevice + ?Default> {}
