use super::*;
use std::convert::TryInto;

pub enum JoyConDevice {
    Connected(HidDevice),
    Disconnected,
}

impl JoyConDevice {
    pub const VENDOR_ID: u16 = 1406;
    pub const PRODUCT_ID_JOYCON_L: u16 = 8198;
    pub const PRODUCT_ID_JOYCON_R: u16 = 8199;

    pub fn new(device_info: &DeviceInfo, hidapi: &HidApi) -> JoyConResult<Self> {
        if device_info.vendor_id() != Self::VENDOR_ID {
            Err(JoyConDeviceError::InvalidVendorID(device_info.vendor_id()))?;
        }

        let device = device_info.open_device(&hidapi)?;

        match device_info.product_id() {
            Self::PRODUCT_ID_JOYCON_L => Ok(JoyConDevice::Connected(device)),
            Self::PRODUCT_ID_JOYCON_R => Ok(JoyConDevice::Connected(device)),
            other => Err(JoyConDeviceError::InvalidProductID(other))?,
        }
    }

    pub fn write(&self, data: &[u8]) -> JoyConResult<usize> {
        match &self {
            JoyConDevice::Connected(hid_device) => Ok(hid_device.write(data)?),
            JoyConDevice::Disconnected => Err(JoyConError::Disconnected),
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> JoyConResult<usize> {
        match &self {
            JoyConDevice::Connected(hid_device) => Ok(hid_device.read(buf)?),
            JoyConDevice::Disconnected => Err(JoyConError::Disconnected),
        }
    }

    pub fn is_connected(&self) -> bool {
        match &self {
            JoyConDevice::Connected(_) => true,
            JoyConDevice::Disconnected => false,
        }
    }
}

impl Debug for JoyConDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match &self {
            JoyConDevice::Connected(_) => "Connected",
            JoyConDevice::Disconnected => "Disconnected",
        })
    }
}

impl<'a> TryInto<&'a HidDevice> for &'a JoyConDevice {
    type Error = JoyConError;

    fn try_into(self) -> Result<&'a HidDevice, Self::Error> {
        match self {
            JoyConDevice::Connected(d) => Ok(d),
            JoyConDevice::Disconnected => Err(JoyConError::Disconnected),
        }
    }
}