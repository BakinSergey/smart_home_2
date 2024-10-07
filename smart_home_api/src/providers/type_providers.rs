use crate::devices::kettle::Kettle;
use crate::devices::socket::Socket;
use crate::devices::thermometer::Thermometer;
use crate::providers::provider::DeviceTypeInfoProvider;

pub struct SocketDeviceInfoProvider;
pub struct ThermometerDeviceInfoProvider;
pub struct KettleDeviceInfoProvider;

impl DeviceTypeInfoProvider for SocketDeviceInfoProvider {
    type DeviceType = Socket;
    fn create_stub(&self) -> Self::DeviceType {
        Socket::new("100500")
    }
}

impl DeviceTypeInfoProvider for ThermometerDeviceInfoProvider {
    type DeviceType = Thermometer;
    fn create_stub(&self) -> Self::DeviceType {
        Thermometer::new("100500")
    }
}

impl DeviceTypeInfoProvider for KettleDeviceInfoProvider {
    type DeviceType = Kettle;
    fn create_stub(&self) -> Self::DeviceType {
        Kettle::new("100500")
    }
}