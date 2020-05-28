use super::*;
use std::convert::TryInto;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum JoyConDeviceType {
    JoyConL = 0,
    JoyConR = 1,
    ProCon = 2,
}

pub mod calibration {
    use super::*;

    pub mod stick {
        use super::*;

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct AxisCalibration {
            max: i16,
            center: i16,
            min: i16,
        }

        impl AxisCalibration {
            /// Max above center
            pub fn max(&self) -> i16 {
                self.max
            }

            /// Center
            pub fn center(&self) -> i16 {
                self.center
            }

            /// Min above center
            pub fn min(&self) -> i16 {
                self.min
            }
        }

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct StickCalibration {
            x: AxisCalibration,
            y: AxisCalibration,
        }

        impl StickCalibration {
            pub fn x(&self) -> &AxisCalibration {
                &self.x
            }

            pub fn y(&self) -> &AxisCalibration {
                &self.y
            }
        }

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct JoyConSticksCalibration {
            left: Option<StickCalibration>,
            right: Option<StickCalibration>,
        }

        impl JoyConSticksCalibration {
            pub fn left(&self) -> &Option<StickCalibration> {
                &self.left
            }

            pub fn right(&self) -> &Option<StickCalibration> {
                &self.right
            }
        }

        impl From<[u8; 18]> for JoyConSticksCalibration {
            fn from(stick_cal: [u8; 18]) -> Self {
                fn stick_cal_to_data(stick_cal: &[u8]) -> [u16; 6] {
                    let mut data = [0u16; 6];

                    data[0] = ((stick_cal[1] as u16) << 8) & 0xF00 | stick_cal[0] as u16;
                    data[1] = ((stick_cal[2] as u16) << 4) | ((stick_cal[1] as u16) >> 4);
                    data[2] = ((stick_cal[4] as u16) << 8) & 0xF00 | stick_cal[3] as u16;
                    data[3] = ((stick_cal[5] as u16) << 4) | ((stick_cal[4] as u16) >> 4);
                    data[4] = ((stick_cal[7] as u16) << 8) & 0xF00 | stick_cal[6] as u16;
                    data[5] = ((stick_cal[8] as u16) << 4) | ((stick_cal[7] as u16) >> 4);

                    data
                }

                let left_stick_calibration = {
                    let left_stick_cal = &stick_cal[0..9];
                    if left_stick_cal.iter()
                        .all(|u| u == &0xFF) {
                        None
                    } else {
                        let left_stick_data = stick_cal_to_data(&stick_cal[0..9]);

                        let left_stick_calibration =
                            StickCalibration {
                                x: AxisCalibration {
                                    center: left_stick_data[2] as i16,
                                    max: left_stick_data[2] as i16 + left_stick_data[0] as i16,
                                    min: left_stick_data[2] as i16 - left_stick_data[4] as i16,
                                },
                                y: AxisCalibration {
                                    center: left_stick_data[3] as i16,
                                    max: left_stick_data[3] as i16 + left_stick_data[1] as i16,
                                    min: left_stick_data[3] as i16 - left_stick_data[5] as i16,
                                },
                            };

                        Some(left_stick_calibration)
                    }
                };

                let right_stick_calibration = {
                    let right_stick_cal = &stick_cal[9..18];
                    if right_stick_cal.iter()
                        .all(|u| u == &0xFF) {
                        None
                    } else {
                        let right_stick_data = stick_cal_to_data(right_stick_cal);

                        let right_stick_calibration =
                            StickCalibration {
                                x: AxisCalibration {
                                    center: right_stick_data[0] as i16,
                                    max: right_stick_data[0] as i16 + right_stick_data[4] as i16,
                                    min: right_stick_data[0] as i16 - right_stick_data[2] as i16,
                                },
                                y: AxisCalibration {
                                    center: right_stick_data[1] as i16,
                                    max: right_stick_data[1] as i16 + right_stick_data[5] as i16,
                                    min: right_stick_data[1] as i16 - right_stick_data[3] as i16,
                                },
                            };

                        Some(right_stick_calibration)
                    }
                };

                JoyConSticksCalibration {
                    left: left_stick_calibration,
                    right: right_stick_calibration,
                }
            }
        }

        pub fn get_factory_calibration(device: &HidDevice) -> Option<JoyConSticksCalibration> {
            device.write(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x3D, 0x60, 0, 0, 18])
                .ok()?;

            let mut buf = [0u8; 64];
            device.read(&mut buf)
                .ok()?;

            let mut report = [0u8; 18];
            report.copy_from_slice(&buf[20..38]);

            Some(JoyConSticksCalibration::from(report))
        }

        pub fn get_user_calibration(device: &HidDevice) -> Option<JoyConSticksCalibration> {
            device.write(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x12, 0x80, 0, 0, 20])
                .ok()?;

            let mut buf = [0u8; 64];
            device.read(&mut buf)
                .ok()?;

            let mut report = [0u8; 18];
            {
                let (left, right) = report.split_at_mut(9);
                left.copy_from_slice(&buf[20..29]);
                right.copy_from_slice(&buf[31..40]);
            }

            Some(JoyConSticksCalibration::from(report))
        }
    }
}


pub struct JoyConDevice {
    hid_device: Option<HidDevice>,
    serial_number: String,
    device_type: JoyConDeviceType,
    stick_factory_calibration: calibration::stick::JoyConSticksCalibration,
    stick_user_calibration: calibration::stick::JoyConSticksCalibration,
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

    pub fn stick_factory_calibration(&self) -> &calibration::stick::JoyConSticksCalibration {
        &self.stick_factory_calibration
    }

    pub fn stick_user_calibration(&self) -> &calibration::stick::JoyConSticksCalibration {
        &self.stick_user_calibration
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
        let stick_factory_calibration = calibration::stick::get_factory_calibration(&hid_device)
            .ok_or(JoyConDeviceError::FailedStickCalibrationLoading)?;
        let stick_user_calibration = calibration::stick::get_user_calibration(&hid_device)
            .ok_or(JoyConDeviceError::FailedStickCalibrationLoading)?;

        Ok(
            JoyConDevice {
                hid_device: Some(hid_device),
                serial_number: serial.to_string(),
                device_type,
                stick_factory_calibration,
                stick_user_calibration,
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
        writeln!(f, "JoyConDevice {{ hid_device: {}, serial_number: {}, device_type: {:?}, stick_factory_calibration: {:?}, stick_user_calibration: {:?} }}",
                 if self.is_connected() {
                     "Connected"
                 } else { "Disconnected" },
                 &self.serial_number,
                 &self.device_type,
                 &self.stick_factory_calibration,
                 &self.stick_user_calibration,
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
