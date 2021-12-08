//! JoyCon's input report mode is divided into five main categories.
//!
//! | Struct | Remarks | Frequency |
//! | :-- | :-- | :-- |
//! | [`SimpleHIDMode<D>`] | Simple HID mode. Reports pushed buttons, and stick directions (one of [8 directions]). | (Every time some button pressed) |
//! | [`StandardFullMode<D>`] | IMU(6-Axis sensor) data with standard input report | 60Hz |
//! | [`SubCommandMode<D, RD>`] | SubCommand's reply with standard input report | ? |
//! | (Unimplemented) | NFC/IR MCU FW update with standard input report  | |
//! | (Unimplemented) | NFC/IR data with standard input report | 60Hz |
//!
//! Standard input report consists of input report ID, Timer, [battery level],
//! [connection info], [button status], [analog stick data], and vibrator input report.
//!
//! If you want to implement input report mode, you can use [`InputReportMode<D>`].
//!
//! [`SimpleHIDMode<D>`]: simple_hid_mode/struct.SimpleHIDMode.html
//! [8 directions]: simple_hid_mode/enum.StickDirection.html
//! [`StandardFullMode<D>`]: standard_full_mode/struct.StandardFullMode.html
//! [`SubCommandMode<D, RD>`]: sub_command_mode/struct.SubCommandMode.html
//! [battery level]: struct.Battery.html
//! [connection info]: struct.ConnectionInfo.html
//! [button status]: struct.PushedButtons.html
//! [analog stick data]: struct.AnalogStickData.html
//! [`InputReportMode<D>`]: trait.InputReportMode.html

pub use self::{simple_hid_mode::SimpleHIDMode, standard_full_mode::StandardFullMode};
use super::*;
pub use common::*;
use std::convert::TryFrom;
use std::hash::Hash;

mod common {
    use super::*;
    use std::convert::TryFrom;

    /// Battery level
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BatteryLevel {
        Empty,
        Critical,
        Low,
        Medium,
        Full,
    }

    /// Battery info
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
    pub struct Battery {
        pub level: BatteryLevel,
        pub is_charging: bool,
    }

    impl TryFrom<u8> for Battery {
        type Error = JoyConError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            let is_charging = value % 2 == 1;
            let value = if is_charging { value - 1 } else { value };
            let level = match value {
                0 => BatteryLevel::Empty,
                2 => BatteryLevel::Critical,
                4 => BatteryLevel::Low,
                6 => BatteryLevel::Medium,
                8 => BatteryLevel::Full,
                _ => {
                    return Err(JoyConReportError::InvalidStandardInputReport(
                        InvalidStandardInputReport::Battery(value),
                    )
                    .into())
                }
            };

            Ok(Battery { level, is_charging })
        }
    }

    /// Device info
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
    pub enum Device {
        JoyCon,
        ProConOrChargingGrip,
    }

    /// Connection info
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
    pub struct ConnectionInfo {
        pub device: Device,
        pub is_powered: bool,
    }

    impl TryFrom<u8> for ConnectionInfo {
        type Error = JoyConError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            let device = match (value >> 1) & 3 {
                3 => Device::JoyCon,
                0 => Device::ProConOrChargingGrip,
                _ => return Err(InvalidStandardInputReport::ConnectionInfo(value).into()),
            };
            let is_powered = (value & 1) == 1;

            Ok(ConnectionInfo { device, is_powered })
        }
    }

    /// Button status
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct PushedButtons {
        pub right: Vec<Buttons>,
        pub shared: Vec<Buttons>,
        pub left: Vec<Buttons>,
    }

    impl PushedButtons {
        const RIGHT_BUTTONS: [Buttons; 8] = [
            Buttons::Y,
            Buttons::X,
            Buttons::B,
            Buttons::A,
            Buttons::SR,
            Buttons::SL,
            Buttons::R,
            Buttons::ZR,
        ];
        const SHARED_BUTTONS: [Buttons; 8] = [
            Buttons::Minus,
            Buttons::Plus,
            Buttons::RStick,
            Buttons::LStick,
            Buttons::Home,
            Buttons::Capture,
            // originally none
            Buttons::Capture,
            Buttons::ChargingGrip,
        ];
        const LEFT_BUTTONS: [Buttons; 8] = [
            Buttons::Down,
            Buttons::Up,
            Buttons::Right,
            Buttons::Left,
            Buttons::SR,
            Buttons::SL,
            Buttons::L,
            Buttons::ZL,
        ];

        pub fn contains(&self, button: Buttons) -> bool {
            self.right.contains(&button)
                || self.shared.contains(&button)
                || self.left.contains(&button)
        }
    }

    impl From<[u8; 3]> for PushedButtons {
        fn from(value: [u8; 3]) -> Self {
            let right_val = value[0];
            let shared_val = value[1];
            let left_val = value[2];

            let right = PushedButtons::RIGHT_BUTTONS
                .iter()
                .enumerate()
                .filter(|(idx, _)| {
                    let idx = 2u8.pow(*idx as u32) as u8;
                    right_val & idx == idx
                })
                .map(|(_, b)| *b)
                .collect();
            let shared = PushedButtons::SHARED_BUTTONS
                .iter()
                .enumerate()
                .filter(|(idx, _)| {
                    let idx = 2u8.pow(*idx as u32) as u8;
                    shared_val & idx == idx
                })
                .map(|(_, b)| *b)
                .collect();
            let left = PushedButtons::LEFT_BUTTONS
                .iter()
                .enumerate()
                .filter(|(idx, _)| {
                    let idx = 2u8.pow(*idx as u32) as u8;
                    left_val & idx == idx
                })
                .map(|(_, b)| *b)
                .collect();

            PushedButtons {
                right,
                shared,
                left,
            }
        }
    }

    /// Analog stick data
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct AnalogStickData {
        pub horizontal: u16,
        pub vertical: u16,
    }

    impl From<[u8; 3]> for AnalogStickData {
        fn from(value: [u8; 3]) -> Self {
            let horizontal = value[0] as u16 | ((value[1] as u16 & 0xF) << 8);
            let vertical = (value[1] as u16 >> 4) | ((value[2] as u16) << 4);

            Self {
                horizontal,
                vertical,
            }
        }
    }

    /// Common parts of the standard input report
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct CommonReport {
        pub input_report_id: u8,
        pub timer: u8,
        pub battery: Battery,
        pub connection_info: ConnectionInfo,
        pub pushed_buttons: PushedButtons,
        pub left_analog_stick_data: AnalogStickData,
        pub right_analog_stick_data: AnalogStickData,
        pub vibrator_input_report: u8,
    }

    impl TryFrom<[u8; 13]> for CommonReport {
        type Error = JoyConError;

        fn try_from(report: [u8; 13]) -> JoyConResult<CommonReport> {
            let input_report_id = report[0];
            let timer = report[1];

            let (battery, connection_info) = {
                let value = report[2];
                let high_nibble = value / 16;
                let low_nibble = value % 16;

                (
                    Battery::try_from(high_nibble)?,
                    ConnectionInfo::try_from(low_nibble)?,
                )
            };

            let pushed_buttons = {
                let array = [report[3], report[4], report[5]];
                PushedButtons::from(array)
            };

            let left_analog_stick_data = {
                let array = [report[6], report[7], report[8]];
                AnalogStickData::from(array)
            };
            let right_analog_stick_data = {
                let array = [report[9], report[10], report[11]];
                AnalogStickData::from(array)
            };

            let vibrator_input_report = report[12];

            Ok(CommonReport {
                input_report_id,
                timer,
                battery,
                connection_info,
                pushed_buttons,
                left_analog_stick_data,
                right_analog_stick_data,
                vibrator_input_report,
            })
        }
    }
}

pub trait InputReportMode<D: JoyConDriver>: Sized {
    type Report: 'static + Send + TryFrom<[u8; 362], Error = JoyConError>;
    type ArgsType: 'static + Send + Copy + AsRef<[u8]>;

    const SUB_COMMAND: SubCommand;
    const ARGS: Self::ArgsType;

    /// Set Joy-Con's input report mode and return instance
    fn new(driver: D) -> JoyConResult<Self>;

    /// read Joy-Con's input report
    fn read_input_report(&self) -> JoyConResult<Self::Report> {
        let mut buf = [0u8; 362];
        self.driver().read(&mut buf)?;

        Self::Report::try_from(buf)
    }

    /// * timeout - milli seconds
    fn read_input_report_timeout(&self, timeout: i32) -> JoyConResult<Self::Report> {
        let mut buf = [0u8; 362];
        self.driver().read_timeout(&mut buf, timeout)?;

        Self::Report::try_from(buf)
    }

    /// Refference of driver.
    fn driver(&self) -> &D;

    /// Mutable refference of driver.
    fn driver_mut(&mut self) -> &mut D;

    /// Unwrap.
    fn into_driver(self) -> D;
}

#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
/// Standard input report with extra report.
pub struct StandardInputReport<EX: TryFrom<[u8; 349], Error = JoyConError>> {
    pub common: CommonReport,
    pub extra: EX,
}

impl<EX> TryFrom<[u8; 362]> for StandardInputReport<EX>
where
    EX: TryFrom<[u8; 349], Error = JoyConError>,
{
    type Error = JoyConError;

    fn try_from(value: [u8; 362]) -> Result<Self, Self::Error> {
        // common report
        let common = {
            let mut data = [0x00; 13];
            data.copy_from_slice(&value[0..13]);
            CommonReport::try_from(data)?
        };

        // extra report
        let extra = {
            let mut data = [0x00; 349];
            data.copy_from_slice(&value[13..362]);
            EX::try_from(data)?
        };

        Ok(StandardInputReport { common, extra })
    }
}

impl<EX> Debug for StandardInputReport<EX>
where
    EX: TryFrom<[u8; 349], Error = JoyConError> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StandardInputReport {{ common: {:?}, extra: {:?} }}",
            &self.common, &self.extra,
        )
    }
}

impl<EX> Clone for StandardInputReport<EX>
where
    EX: TryFrom<[u8; 349], Error = JoyConError> + Clone,
{
    fn clone(&self) -> Self {
        let common = self.common.clone();
        let extra = self.extra.clone();

        StandardInputReport { common, extra }
    }
}

impl<EX> Hash for StandardInputReport<EX>
where
    EX: TryFrom<[u8; 349], Error = JoyConError> + Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.common.hash(state);
        self.extra.hash(state);
    }
}

impl<EX> PartialEq for StandardInputReport<EX>
where
    EX: TryFrom<[u8; 349], Error = JoyConError> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.common.eq(&other.common) && self.extra.eq(&other.extra)
    }
}

impl<EX> Eq for StandardInputReport<EX> where EX: TryFrom<[u8; 349], Error = JoyConError> + Eq {}

/// Receive standard input report with sub-command's reply.
///
/// If you want to send sub-command and get reply, imply [`SubCommandReplyData`] for your struct.
///
/// [`SubCommandReplyData`]: trait.SubCommandReplyData.html
pub mod sub_command_mode {
    use super::*;

    /// Ack byte. If it is ACK, it contains data type.
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
    pub enum AckByte {
        Ack { data_type: u8 },
        Nack,
    }

    impl From<u8> for AckByte {
        fn from(u: u8) -> Self {
            if u >> 7 == 1 {
                AckByte::Ack {
                    data_type: u & 0x7F,
                }
            } else {
                AckByte::Nack
            }
        }
    }

    /// An interface for dealing with sub-command's reply.
    ///
    /// # Example - implement `SubCommandReplyData`
    /// ```ignore
    /// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    /// pub struct LightsStatus {
    ///     light_up: Vec<LightUp>,
    ///     flash: Vec<Flash>,
    /// }
    ///
    /// const LIGHT_UP: [LightUp; 4] =
    ///     [LightUp::LED0, LightUp::LED1, LightUp::LED2, LightUp::LED3];
    /// const FLASH: [Flash; 4] =
    ///     [Flash::LED0, Flash::LED1, Flash::LED2, Flash::LED3];
    ///
    /// impl TryFrom<[u8; 35]> for LightsStatus {
    ///     type Error = JoyConError;
    ///
    ///     fn try_from(value: [u8; 35]) -> Result<Self, Self::Error> {
    ///         let value = value[0];
    ///
    ///         // parse reply
    ///         let light_up = LIGHT_UP.iter()
    ///             .filter(|&&l| {
    ///                 let light = l as u8;
    ///                 value & light == light
    ///             })
    ///             .cloned()
    ///             .collect();
    ///         let flash = FLASH.iter()
    ///             .filter(|&&f| {
    ///                 let flash = f as u8;
    ///                 value & flash == flash
    ///             })
    ///             .cloned()
    ///             .collect();
    ///
    ///         Ok(LightsStatus { light_up, flash })
    ///     }
    /// }
    ///
    /// impl SubCommandReplyData for LightsStatus {
    ///     type ArgsType = [u8; 0];
    ///     const SUB_COMMAND: SubCommand = SubCommand::GetPlayerLights;
    ///     const ARGS: Self::ArgsType = [];
    /// }
    /// ```
    pub trait SubCommandReplyData: TryFrom<[u8; 35], Error = JoyConError> {
        type ArgsType: 'static + Send + Copy + AsRef<[u8]>;
        const SUB_COMMAND: SubCommand;
        const ARGS: Self::ArgsType;

        /// The mode remains the same, sending commands and receiving replies.
        fn once<D>(
            driver: &mut D,
        ) -> JoyConResult<SubCommandReply<StandardInputReport<SubCommandReport<Self>>>>
        where
            Self: std::marker::Sized,
            D: JoyConDriver,
        {
            match driver.send_sub_command(Self::SUB_COMMAND, Self::ARGS.as_ref()) {
                Ok(reply) => {
                    let ok = match reply {
                        SubCommandReply::Checked(reply) => {
                            SubCommandReply::Checked(StandardInputReport::try_from(reply)?)
                        }
                        SubCommandReply::Unchecked => SubCommandReply::Unchecked,
                    };
                    Ok(ok)
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Replies to sub-commands
    #[derive(Clone)]
    pub struct SubCommandReport<RD>
    where
        RD: SubCommandReplyData,
    {
        pub ack_byte: AckByte,
        pub sub_command_id: u8,
        pub reply: RD,
    }

    impl<RD> Debug for SubCommandReport<RD>
    where
        RD: SubCommandReplyData + Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "SubCommandReport {{\
                            \n\t ack_byte: {:?},
                            \n\t sub_command_id: {},
                            \n\t reply: {:?},
                   }}",
                self.ack_byte, self.sub_command_id, &self.reply
            )
        }
    }

    impl<RD> TryFrom<[u8; 349]> for SubCommandReport<RD>
    where
        RD: SubCommandReplyData,
    {
        type Error = JoyConError;

        fn try_from(value: [u8; 349]) -> Result<Self, Self::Error> {
            let ack_byte = AckByte::from(value[0]);
            let sub_command_id = value[1];
            let mut reply = [0x00; 35];
            reply.copy_from_slice(&value[2..37]);
            let reply = RD::try_from(reply)?;

            Ok(SubCommandReport {
                ack_byte,
                sub_command_id,
                reply,
            })
        }
    }
}

/// Receive standard full report (standard input report with IMU(6-Axis sensor) data).
///
/// Pushes current state at 60Hz (ProCon: 120Hz).
pub mod standard_full_mode {
    use super::*;

    /// IMU(6-Axis sensor)'s value.
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct AxisData {
        /// Acceleration to X measured
        pub accel_x: i16,
        /// Acceleration to Y measured
        pub accel_y: i16,
        /// Acceleration to Z measured
        pub accel_z: i16,
        /// Rotation of X measured
        pub gyro_1: i16,
        /// Rotation of Y measured
        pub gyro_2: i16,
        /// Rotation of Z measured
        pub gyro_3: i16,
    }

    impl From<[u8; 12]> for AxisData {
        fn from(value: [u8; 12]) -> Self {
            let accel_x = i16::from_le_bytes([value[0], value[1]]);
            let accel_y = i16::from_le_bytes([value[2], value[3]]);
            let accel_z = i16::from_le_bytes([value[4], value[5]]);

            let gyro_1 = i16::from_le_bytes([value[6], value[7]]);
            let gyro_2 = i16::from_le_bytes([value[8], value[9]]);
            let gyro_3 = i16::from_le_bytes([value[10], value[11]]);

            AxisData {
                accel_x,
                accel_y,
                accel_z,
                gyro_1,
                gyro_2,
                gyro_3,
            }
        }
    }

    /// 6-Axis data. 3 frames of 2 groups of 3 Int16LE each. Group is Acc followed by Gyro.
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone)]
    pub struct IMUData {
        pub data: [AxisData; 3],
    }

    impl TryFrom<[u8; 349]> for IMUData {
        type Error = JoyConError;

        fn try_from(value: [u8; 349]) -> Result<Self, Self::Error> {
            let latest = {
                let mut latest = [0x00; 12];
                latest.copy_from_slice(&value[0..12]);
                latest
            };
            let a_5ms_older = {
                let mut latest = [0x00; 12];
                latest.copy_from_slice(&value[12..24]);
                latest
            };
            let a_10ms_older = {
                let mut latest = [0x00; 12];
                latest.copy_from_slice(&value[24..36]);
                latest
            };

            let axis_data = [
                AxisData::from(latest),
                AxisData::from(a_5ms_older),
                AxisData::from(a_10ms_older),
            ];

            Ok(IMUData { data: axis_data })
        }
    }

    /// Joy-Con emitting standard full report includes IMU(6-Axis sensor)
    ///
    /// # Example
    /// ```no_run
    /// use joycon_rs::prelude::*;
    ///
    /// let (sender, receiver) = std::sync::mpsc::channel();
    /// let _output = std::thread::spawn( move || {
    ///     while let Ok(update) = receiver.recv() {
    ///         dbg!(update);
    ///     }
    /// });
    ///
    /// let manager = JoyConManager::get_instance();
    ///
    /// let devices = {
    ///     let lock = manager.lock();
    ///     match lock {
    ///         Ok(manager) => manager.new_devices(),
    ///         Err(_) => return,
    ///     }
    /// };
    ///
    /// devices.iter()
    ///     .flat_map(|device| SimpleJoyConDriver::new(&device))
    ///     .try_for_each::<_, JoyConResult<()>>(|driver| {
    ///         let sender = sender.clone();
    ///
    ///         // Set Joy-Con's mode
    ///         let joycon = StandardFullMode::new(driver)?;
    ///
    ///         std::thread::spawn( move || {
    ///             loop {
    ///                 sender.send(joycon.read_input_report())
    ///                       .unwrap();
    ///             }
    ///         });
    ///
    ///         Ok(())
    ///     })
    ///     .unwrap();
    /// ```
    pub struct StandardFullMode<D: JoyConDriver> {
        driver: D,
    }

    impl<D> InputReportMode<D> for StandardFullMode<D>
    where
        D: JoyConDriver,
    {
        type Report = StandardInputReport<IMUData>;
        type ArgsType = [u8; 1];
        const SUB_COMMAND: SubCommand = SubCommand::SetInputReportMode;
        const ARGS: Self::ArgsType = [0x30];

        fn new(driver: D) -> JoyConResult<Self> {
            let mut driver = driver;
            // enable IMU(6-Axis sensor)
            let imf_enabled = driver.enabled_features().iter().any(|jf|
                matches!(jf, JoyConFeature::IMUFeature(_))
            );
            if !imf_enabled {
                driver.enable_feature(JoyConFeature::IMUFeature(IMUConfig::default()))?;
            }

            driver.set_valid_reply(false);

            driver.send_sub_command(Self::SUB_COMMAND, Self::ARGS.as_ref())?;

            Ok(StandardFullMode { driver })
        }

        fn driver(&self) -> &D {
            &self.driver
        }

        fn driver_mut(&mut self) -> &mut D {
            &mut self.driver
        }

        fn into_driver(self) -> D {
            self.driver
        }
    }
}

/// Receive simple HID report.
///
/// Simple HID report consists of input report id, button status,
/// stick direction, filter data.
///
/// Pushes updates with every button press.
pub mod simple_hid_mode {
    use super::*;

    #[allow(non_camel_case_types)]
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
    pub enum SimpleHIDButton {
        Down,
        Right,
        Left,
        Up,
        SL,
        SR,
        Minus,
        Plus,
        LeftStick,
        RightStick,
        Home,
        Capture,
        L_R,
        ZL_ZR,
    }

    const BUTTON1: [SimpleHIDButton; 6] = {
        use SimpleHIDButton::*;
        [Down, Right, Left, Up, SL, SR]
    };
    const BUTTON2: [SimpleHIDButton; 8] = {
        use SimpleHIDButton::*;
        [
            Minus, Plus, LeftStick, RightStick, Home, Capture, L_R, ZL_ZR,
        ]
    };

    /// Hold your controller sideways so that SL, SYNC, and SR line up with the screen. Pushing the stick towards a direction in this table will cause that value to be sent.
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
    pub enum StickDirection {
        Up,
        UpperRight,
        Right,
        BottomRight,
        Bottom,
        BottomLeft,
        Left,
        UpperLeft,
        Neutral,
    }

    impl TryFrom<u8> for StickDirection {
        type Error = JoyConError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            use StickDirection::*;

            let button = match value {
                0 => Up,
                1 => UpperRight,
                2 => Right,
                3 => BottomRight,
                4 => Bottom,
                5 => BottomLeft,
                6 => Left,
                7 => UpperLeft,
                8 => Neutral,
                v => return Err(InvalidSimpleHIDReport::InvalidStickDirection(v).into()),
            };

            Ok(button)
        }
    }

    /// Pushed buttons and stick direction.
    #[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone)]
    pub struct SimpleHIDReport {
        pub input_report_id: u8,
        pub pushed_buttons: Vec<SimpleHIDButton>,
        pub stick_direction: StickDirection,
        pub filler_data: [u8; 8],
    }

    impl TryFrom<[u8; 12]> for SimpleHIDReport {
        type Error = JoyConError;

        fn try_from(value: [u8; 12]) -> Result<Self, Self::Error> {
            let input_report_id = value[0];
            let pushed_buttons = {
                let byte_1 = value[1];
                let byte_2 = value[2];
                [].iter()
                    .chain(
                        BUTTON1
                            .iter()
                            .enumerate()
                            .filter(|(idx, _)| {
                                let idx = 2u8.pow(*idx as u32) as u8;
                                byte_1 & idx == idx
                            })
                            .map(|(_, b)| b),
                    )
                    .chain(
                        BUTTON2
                            .iter()
                            .enumerate()
                            .filter(|(idx, _)| {
                                let idx = 2u8.pow(*idx as u32) as u8;
                                byte_2 & idx == idx
                            })
                            .map(|(_, b)| b),
                    )
                    .cloned()
                    .collect()
            };
            let stick_direction = StickDirection::try_from(value[3])?;
            let filter_data = {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&value[4..12]);
                buf
            };

            Ok(SimpleHIDReport {
                input_report_id,
                pushed_buttons,
                stick_direction,
                filler_data: filter_data,
            })
        }
    }

    impl TryFrom<[u8; 362]> for SimpleHIDReport {
        type Error = JoyConError;

        fn try_from(value: [u8; 362]) -> Result<Self, Self::Error> {
            let mut buf = [0u8; 12];
            buf.copy_from_slice(&value[0..12]);

            Self::try_from(buf)
        }
    }

    /// Simple HID mode pushes updates with every button press.
    ///
    /// # Example
    /// ```no_run
    /// use joycon_rs::prelude::*;
    ///
    /// let (sender, receiver) = std::sync::mpsc::channel();
    ///
    /// // Receive all Joy-Con's simple HID reports
    /// while let Ok(simple_hid_report) = receiver.recv() {
    ///     // Output reports
    ///     dbg!(simple_hid_report);
    /// }
    ///
    /// let manager = JoyConManager::get_instance();
    ///
    /// let devices = {
    ///     let lock = manager.lock();
    ///     match lock {
    ///         Ok(manager) => manager.new_devices(),
    ///         Err(_) => return,
    ///     }
    /// };
    ///
    /// devices.iter()
    ///     .flat_map(|d| SimpleJoyConDriver::new(&d))
    ///     .try_for_each::<_, JoyConResult<()>>(|driver| {
    ///         let sender = sender.clone();
    ///
    ///         // Set Joy-Con's mode
    ///         let simple_hid_mode_joycon = SimpleHIDMode::new(driver)?;
    ///
    ///         std::thread::spawn( move || {
    ///             loop {
    ///                 sender.send(simple_hid_mode_joycon.read_input_report())
    ///                       .unwrap();
    ///             }
    ///         });
    ///
    ///         Ok(())
    ///     })
    ///     .unwrap();
    /// ```
    pub struct SimpleHIDMode<D: JoyConDriver> {
        driver: D,
    }

    impl<D> InputReportMode<D> for SimpleHIDMode<D>
    where
        D: JoyConDriver,
    {
        type Report = SimpleHIDReport;
        type ArgsType = [u8; 1];
        const SUB_COMMAND: SubCommand = SubCommand::SetInputReportMode;
        const ARGS: Self::ArgsType = [0x3F];

        fn new(driver: D) -> JoyConResult<Self> {
            let mut driver = driver;
            driver.send_sub_command(Self::SUB_COMMAND, Self::ARGS.as_ref())?;

            Ok(SimpleHIDMode { driver })
        }

        fn driver(&self) -> &D {
            &self.driver
        }

        fn driver_mut(&mut self) -> &mut D {
            &mut self.driver
        }

        fn into_driver(self) -> D {
            self.driver
        }
    }
}
