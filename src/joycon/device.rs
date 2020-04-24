use super::*;
use std::convert::TryInto;


pub fn is_joycon(device_info: &DeviceInfo) -> JoyConResult<()> {
    if device_info.vendor_id() != JoyConDevice::VENDOR_ID {
        Err(JoyConDeviceError::InvalidVendorID(device_info.vendor_id()))?;
    }

    match device_info.product_id() {
        JoyConDevice::PRODUCT_ID_JOYCON_L | JoyConDevice::PRODUCT_ID_JOYCON_R => Ok(()),
        other => Err(JoyConDeviceError::InvalidProductID(other))?,
    }
}

pub enum JoyConDevice {
    Connected(HidDevice),
    Disconnected,
}

impl JoyConDevice {
    pub const VENDOR_ID: u16 = 1406;
    pub const PRODUCT_ID_JOYCON_L: u16 = 8198;
    pub const PRODUCT_ID_JOYCON_R: u16 = 8199;

    pub fn new(device_info: &DeviceInfo, hidapi: &HidApi) -> JoyConResult<Self> {
        is_joycon(device_info)?;

        let serial = device_info.serial_number().unwrap_or("");
        let device = hidapi.open_serial(device_info.vendor_id(),
                                        device_info.product_id(),
                                        serial)?;
        // let device = device_info.open_device(&hidapi)?;
        Ok(JoyConDevice::Connected(device))
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
