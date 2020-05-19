use super::*;
use std::convert::TryInto;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum JoyConDeviceType {
    JoyConL = 0,
    JoyConR = 1,
    ProCon = 2,
}

pub struct JoyConDevice {
    hid_device: Option<HidDevice>,
    serial_number: String,
    device_type: JoyConDeviceType,
}

impl JoyConDevice {
    pub const VENDOR_ID: u16 = 1406;
    pub const PRODUCT_ID_JOYCON_L: u16 = 8198;
    pub const PRODUCT_ID_JOYCON_R: u16 = 8199;
    pub const PRODUCT_ID_PROCON: u16 = 8201;

    pub fn check_type_of_device(device_info: &DeviceInfo) -> JoyConResult<JoyConDeviceType> {
        if device_info.vendor_id() != JoyConDevice::VENDOR_ID {
            return Err(JoyConDeviceError::InvalidVendorID(device_info.vendor_id()).into());
        }

        match device_info.product_id() {
            JoyConDevice::PRODUCT_ID_JOYCON_L => Ok(JoyConDeviceType::JoyConL),
            JoyConDevice::PRODUCT_ID_JOYCON_R => Ok(JoyConDeviceType::JoyConR),
            JoyConDevice::PRODUCT_ID_PROCON => Ok(JoyConDeviceType::ProCon),
            other => Err(JoyConDeviceError::InvalidProductID(other).into()),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.hid_device.is_some()
    }

    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    /// Set blocking mode.
    ///
    /// # Notice
    /// If you are using non-blocking mode,
    /// it is more likely to fail to validate the sub command reply.
    pub fn set_blocking_mode(&self, blocking: bool) -> JoyConResult<()> {
        if let Some(hid_device) = &self.hid_device {
            Ok(hid_device.set_blocking_mode(blocking)?)
        } else {
            Err(JoyConError::Disconnected)
        }
    }

    pub fn device_type(&self) -> JoyConDeviceType {
        self.device_type.clone()
    }

    pub fn reset_device(&mut self, hid_device: HidDevice) {
        self.hid_device = Some(hid_device);
    }

    pub fn forget_device(&mut self) {
        self.hid_device = None;
    }

    pub fn new(device_info: &DeviceInfo, hidapi: &HidApi) -> JoyConResult<Self> {
        let device_type = Self::check_type_of_device(device_info)?;

        let serial = device_info.serial_number().unwrap_or("");
        let hid_device = hidapi.open_serial(device_info.vendor_id(),
                                            device_info.product_id(),
                                            serial)?;

        Ok(
            JoyConDevice {
                hid_device: Some(hid_device),
                serial_number: serial.to_string(),
                device_type,
            }
        )
    }

    pub fn write(&self, data: &[u8]) -> JoyConResult<usize> {
        if let Some(hid_device) = &self.hid_device {
            Ok(hid_device.write(data)?)
        } else {
            Err(JoyConError::Disconnected)
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> JoyConResult<usize> {
        if let Some(hid_device) = &self.hid_device {
            let res = hid_device.read(buf)?;

            if buf.iter().all(|e| e == &0) {
                Err(JoyConReportError::EmptyReport.into())
            } else {
                Ok(res)
            }
        } else {
            Err(JoyConError::Disconnected)
        }
    }

    /// * timeout - milli seconds
    pub fn read_timeout(&self, buf: &mut [u8], timeout: i32) -> JoyConResult<usize> {
        if let Some(hid_device) = &self.hid_device {
            let res = hid_device.read_timeout(buf, timeout)?;

            if buf.iter().all(|e| e == &0) {
                Err(JoyConReportError::EmptyReport.into())
            } else {
                Ok(res)
            }
        } else {
            Err(JoyConError::Disconnected)
        }
    }
}

impl Debug for JoyConDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "JoyConDevice {{ hid_device: {}, serial_number: {}, device_type: {:?} }}",
                 if self.is_connected() {
                     "Connected"
                 } else { "Disconnected" },
                 &self.serial_number,
                 &self.device_type
        )
    }
}

impl<'a> TryInto<&'a HidDevice> for &'a JoyConDevice {
    type Error = JoyConError;

    fn try_into(self) -> Result<&'a HidDevice, Self::Error> {
        self.hid_device.as_ref()
            .ok_or(JoyConError::Disconnected)
    }
}
