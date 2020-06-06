use super::*;
use std::convert::TryInto;

#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
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
        pub enum StickCalibration {
            Available {
                x: AxisCalibration,
                y: AxisCalibration,
            },
            Unavailable,
        }

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct JoyConSticksCalibration {
            left: StickCalibration,
            right: StickCalibration,
        }

        impl JoyConSticksCalibration {
            pub fn left(&self) -> &StickCalibration {
                &self.left
            }

            pub fn right(&self) -> &StickCalibration {
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
                    if left_stick_cal.iter().all(|u| u == &0xFF)
                    {
                        StickCalibration::Unavailable
                    } else {
                        let left_stick_data = stick_cal_to_data(&stick_cal[0..9]);

                        StickCalibration::Available {
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
                        }
                    }
                };

                let right_stick_calibration = {
                    let right_stick_cal = &stick_cal[9..18];
                    if right_stick_cal.iter().all(|u| u == &0xFF)
                    {
                        StickCalibration::Unavailable
                    } else {
                        let right_stick_data = stick_cal_to_data(right_stick_cal);

                        StickCalibration::Available {
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
                        }
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

            for _ in 0..5 {
                let mut buf = [0u8; 64];
                device.read_timeout(&mut buf, 20)
                    .ok()?;

                match buf[14..20] {
                    [0x10, 0x3D, 0x60, 0, 0, 18] => {}
                    _ => continue,
                }

                let mut report = [0u8; 18];
                report.copy_from_slice(&buf[20..38]);

                return Some(JoyConSticksCalibration::from(report));
            }

            None
        }

        pub fn get_user_calibration(device: &HidDevice) -> Option<JoyConSticksCalibration> {
            device.write(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x12, 0x80, 0, 0, 20])
                .ok()?;

            for _ in 0..5 {
                let mut buf = [0u8; 64];
                device.read_timeout(&mut buf, 20)
                    .ok()?;

                match buf[14..20] {
                    [0x10, 0x12, 0x80, 0, 0, 20] => {}
                    _ => continue,
                }

                let mut report = [0u8; 18];
                {
                    let (left, right) = report.split_at_mut(9);
                    left.copy_from_slice(&buf[20..29]);
                    right.copy_from_slice(&buf[31..40]);
                }

                return Some(JoyConSticksCalibration::from(report));
            }

            None
        }

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct StickParameters {
            dead_zone: u16,
            range_ratio: u16,
        }

        impl StickParameters {
            pub fn dead_zone(&self) -> u16 {
                self.dead_zone
            }

            pub fn range_ratio(&self) -> u16 {
                self.range_ratio
            }
        }

        impl From<[u8; 18]> for StickParameters {
            fn from(array: [u8; 18]) -> Self {
                fn decode(stick_cal: &[u8]) -> [u16; 12] {
                    let mut data = [0u16; 12];

                    data[0] = ((stick_cal[1] as u16) << 8) & 0xF00 | stick_cal[0] as u16;
                    data[1] = ((stick_cal[2] as u16) << 4) | ((stick_cal[1] as u16) >> 4);
                    data[2] = ((stick_cal[4] as u16) << 8) & 0xF00 | stick_cal[3] as u16;
                    data[3] = ((stick_cal[5] as u16) << 4) | ((stick_cal[4] as u16) >> 4);
                    data[4] = ((stick_cal[7] as u16) << 8) & 0xF00 | stick_cal[6] as u16;
                    data[5] = ((stick_cal[8] as u16) << 4) | ((stick_cal[7] as u16) >> 4);
                    data[6] = ((stick_cal[10] as u16) << 8) & 0xF00 | stick_cal[9] as u16;
                    data[7] = ((stick_cal[11] as u16) << 4) | ((stick_cal[10] as u16) >> 4);
                    data[8] = ((stick_cal[13] as u16) << 8) & 0xF00 | stick_cal[12] as u16;
                    data[9] = ((stick_cal[14] as u16) << 4) | ((stick_cal[13] as u16) >> 4);
                    data[10] = ((stick_cal[16] as u16) << 8) & 0xF00 | stick_cal[15] as u16;
                    data[11] = ((stick_cal[17] as u16) << 4) | ((stick_cal[16] as u16) >> 4);

                    data
                }

                let decoded = decode(&array);

                StickParameters {
                    dead_zone: decoded[2],
                    range_ratio: decoded[3],
                }
            }
        }

        pub fn get_parameters(device: &HidDevice) -> Option<StickParameters> {
            device.write(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x86, 0x60, 0, 0, 18])
                .ok()?;

            for _ in 0..5 {
                let mut buf = [0u8; 64];
                device.read_timeout(&mut buf, 20)
                    .ok()?;

                match buf[14..20] {
                    [0x10, 0x86, 0x60, 0, 0, 18] => {}
                    _ => continue,
                }

                let mut report = [0u8; 18];
                report.copy_from_slice(&buf[20..38]);

                return Some(StickParameters::from(report));
            }

            None
        }
    }

    pub mod imu {
        use super::*;
        use std::fmt::Debug;
        use std::hash::Hash;

        #[derive(Debug)]
        pub struct XYZ<T: Debug> {
            pub x: T,
            pub y: T,
            pub z: T,
        }

        impl<T> Clone for XYZ<T> where T: Debug + Clone {
            fn clone(&self) -> Self {
                Self {
                    x: self.x.clone(),
                    y: self.y.clone(),
                    z: self.z.clone(),
                }
            }
        }

        impl<T> Hash for XYZ<T> where T: Debug + Hash {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.x.hash(state);
                self.y.hash(state);
                self.z.hash(state);
            }
        }

        impl<T> PartialEq for XYZ<T> where T: Debug + PartialEq {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x
                    && self.y == other.y
                    && self.z == other.z
            }
        }

        impl<T> Eq for XYZ<T> where T: Debug + Eq {}

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub enum IMUCalibration {
            Available {
                /// Acc XYZ origin position when completely horizontal and stick is upside
                acc_origin_position: XYZ<i16>,
                /// Acc XYZ sensitivity special coeff, for default sensitivity: ±8G.
                acc_sensitivity_special_coeff: XYZ<i16>,
                /// Gyro XYZ origin position when still
                gyro_origin_position: XYZ<i16>,
                /// Gyro XYZ sensitivity special coeff, for default sensitivity: ±2000dps.
                gyro_sensitivity_special_coeff: XYZ<i16>,
            },
            Unavailable,
        }

        impl From<[u8; 24]> for IMUCalibration {
            fn from(value: [u8; 24]) -> Self {
                use std::slice::Iter;
                use std::iter::Cloned;

                if value.iter().all(|v| v == &0xFF) {
                    return IMUCalibration::Unavailable;
                }

                fn convert(little: u8, big: u8) -> i16 {
                    i16::from_be_bytes([big, little])
                }

                fn iter_to_xyz_i16(iter: &mut Cloned<Iter<u8>>) -> XYZ<i16> {
                    let x = convert(iter.next().unwrap(), iter.next().unwrap());
                    let y = convert(iter.next().unwrap(), iter.next().unwrap());
                    let z = convert(iter.next().unwrap(), iter.next().unwrap());

                    XYZ { x, y, z }
                }

                let mut iter = value.iter().cloned();

                let acc_origin_position = iter_to_xyz_i16(&mut iter);
                let acc_sensitivity_special_coeff = iter_to_xyz_i16(&mut iter);
                let gyro_origin_position = iter_to_xyz_i16(&mut iter);
                let gyro_sensitivity_special_coeff = iter_to_xyz_i16(&mut iter);

                IMUCalibration::Available {
                    acc_origin_position,
                    acc_sensitivity_special_coeff,
                    gyro_origin_position,
                    gyro_sensitivity_special_coeff,
                }
            }
        }

        pub fn get_factory_calibration(device: &HidDevice) -> Option<IMUCalibration> {
            device.write(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x20, 0x60, 0, 0, 24])
                .ok()?;

            for _ in 0..5 {
                let mut buf = [0u8; 64];
                device.read_timeout(&mut buf, 20)
                    .ok()?;

                match buf[14..20] {
                    [0x10, 0x20, 0x60, 0, 0, 24] => {}
                    _ => continue,
                }

                let mut report = [0u8; 24];
                report.copy_from_slice(&buf[20..44]);

                return Some(IMUCalibration::from(report));
            }

            None
        }

        pub fn get_user_calibration(device: &HidDevice) -> Option<IMUCalibration> {
            device.write(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x28, 0x80, 0, 0, 24])
                .ok()?;

            for _ in 0..5 {
                let mut buf = [0u8; 64];
                device.read_timeout(&mut buf, 20)
                    .ok()?;

                match buf[14..20] {
                    [0x10, 0x28, 0x80, 0, 0, 24] => {}
                    _ => continue,
                }

                let mut report = [0u8; 24];
                report.copy_from_slice(&buf[20..44]);

                return Some(IMUCalibration::from(report));
            }

            None
        }

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct IMUOffsets {
            pub x: i16,
            pub y: i16,
            pub z: i16,
        }

        impl From<[u8; 6]> for IMUOffsets {
            fn from(array: [u8; 6]) -> Self {
                let x = i16::from_le_bytes([array[0], array[1]]);
                let y = i16::from_le_bytes([array[2], array[3]]);
                let z = i16::from_le_bytes([array[4], array[5]]);

                IMUOffsets {
                    x,
                    y,
                    z,
                }
            }
        }

        pub fn get_offsets(device: &HidDevice) -> Option<IMUOffsets> {
            device.write(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x80, 0x60, 0, 0, 6])
                .ok()?;

            for _ in 0..5 {
                let mut buf = [0u8; 64];
                device.read_timeout(&mut buf, 20)
                    .ok()?;

                match buf[14..20] {
                    [0x10, 0x80, 0x60, 0, 0, 6] => {}
                    _ => continue,
                }

                let mut report = [0u8; 6];
                report.copy_from_slice(&buf[20..26]);

                return Some(IMUOffsets::from(report));
            }

            None
        }
    }
}

pub mod color {
    use super::*;

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct Color {
        /// Controller body color. ex. [30, 220, 0] (Splatoon2 green)
        pub body: [u8; 3],
        pub buttons: [u8; 3],
        pub left_grip: Option<[u8; 3]>,
        pub right_grip: Option<[u8; 3]>,
    }

    impl From<[u8; 12]> for Color {
        fn from(array: [u8; 12]) -> Self {
            let body = [array[0], array[1], array[2]];
            let buttons = [array[3], array[4], array[5]];
            let left_grip = if array[6..9].iter().all(|a| a == &0xFF) {
                None
            } else {
                Some([array[6], array[7], array[8]])
            };
            let right_grip = if array[9..12].iter().all(|a| a == &0xFF) {
                None
            } else {
                Some([array[9], array[10], array[11]])
            };

            Color {
                body,
                buttons,
                left_grip,
                right_grip,
            }
        }
    }

    pub fn get_color(device: &HidDevice) -> Option<Color> {
        device.write(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x50, 0x60, 0, 0, 12])
            .ok()?;

        for _ in 0..5 {
            let mut buf = [0u8; 64];
            device.read_timeout(&mut buf, 20)
                .ok()?;

            match buf[14..20] {
                [0x10, 0x50, 0x60, 0, 0, 12] => {}
                _ => continue,
            }

            let mut report = [0u8; 12];
            report.copy_from_slice(&buf[20..32]);

            return Some(Color::from(report));
        }

        None
    }
}


pub struct JoyConDevice {
    hid_device: Option<HidDevice>,
    serial_number: String,
    device_type: JoyConDeviceType,
    stick_parameters: calibration::stick::StickParameters,
    stick_factory_calibration: calibration::stick::JoyConSticksCalibration,
    stick_user_calibration: calibration::stick::JoyConSticksCalibration,
    imu_offsets: calibration::imu::IMUOffsets,
    imu_factory_calibration: calibration::imu::IMUCalibration,
    imu_user_calibration: calibration::imu::IMUCalibration,
    color: color::Color,
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

    pub fn stick_parameters(&self) -> &calibration::stick::StickParameters {
        &self.stick_parameters
    }

    pub fn stick_factory_calibration(&self) -> &calibration::stick::JoyConSticksCalibration {
        &self.stick_factory_calibration
    }

    pub fn stick_user_calibration(&self) -> &calibration::stick::JoyConSticksCalibration {
        &self.stick_user_calibration
    }

    pub fn imu_offsets(&self) -> &calibration::imu::IMUOffsets {
        &self.imu_offsets
    }

    pub fn imu_factory_calibration(&self) -> &calibration::imu::IMUCalibration {
        &self.imu_factory_calibration
    }

    pub fn imu_user_calibration(&self) -> &calibration::imu::IMUCalibration {
        &self.imu_user_calibration
    }

    pub fn color(&self) -> &color::Color { &self.color }

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
        let stick_parameters = calibration::stick::get_parameters(&hid_device)
            .ok_or(JoyConDeviceError::FailedStickParameterLoading)?;
        let stick_factory_calibration = calibration::stick::get_factory_calibration(&hid_device)
            .ok_or(JoyConDeviceError::FailedStickCalibrationLoading)?;
        let stick_user_calibration = calibration::stick::get_user_calibration(&hid_device)
            .ok_or(JoyConDeviceError::FailedStickCalibrationLoading)?;
        let imu_offsets = calibration::imu::get_offsets(&hid_device)
            .ok_or(JoyConDeviceError::FailedIMUOffsetsLoading)?;
        let imu_factory_calibration = calibration::imu::get_factory_calibration(&hid_device)
            .ok_or(JoyConDeviceError::FailedIMUCalibrationLoading)?;
        let imu_user_calibration = calibration::imu::get_user_calibration(&hid_device)
            .ok_or(JoyConDeviceError::FailedIMUCalibrationLoading)?;
        let color = color::get_color(&hid_device)
            .ok_or(JoyConDeviceError::FailedColorLoading)?;

        Ok(
            JoyConDevice {
                hid_device: Some(hid_device),
                serial_number: serial.to_string(),
                device_type,
                stick_parameters,
                stick_factory_calibration,
                stick_user_calibration,
                imu_offsets,
                imu_factory_calibration,
                imu_user_calibration,
                color,
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
        writeln!(f, "JoyConDevice {{ hid_device: {}, serial_number: {}, device_type: {:?}, stick_parameters: {:?}, , stick_factory_calibration: {:?}, stick_user_calibration: {:?}, imu_offsets: {:?}, imu_factory_calibration: {:?}, imu_user_calibration: {:?}, color: {:?} }}",
                 if self.is_connected() {
                     "Connected"
                 } else { "Disconnected" },
                 &self.serial_number,
                 &self.device_type,
                 &self.stick_parameters,
                 &self.stick_factory_calibration,
                 &self.stick_user_calibration,
                 &self.imu_offsets,
                 &self.imu_factory_calibration,
                 &self.imu_user_calibration,
                 &self.color,
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
