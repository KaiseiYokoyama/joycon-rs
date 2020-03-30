use crate::prelude::*;

pub use joycon_device::JoyConDevice;
// pub use joycon::JoyCon;
pub use driver::{Rotation, JoyConDriver, GlobalPacketNumber, SimpleJoyConDriver, simple_hid_mode, standard_input_report, Command, SubCommand};

use std::sync::Arc;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Buttons {
    A,
    X,
    Y,
    B,
    Plus,
    RStick,
    Home,
    R,
    ZR,
    Right,
    Up,
    Left,
    Down,
    Minus,
    LStick,
    Capture,
    L,
    ZL,
    SL,
    SR,
    ChargingGrip,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum StickDirection {
    Left,
    UpperLeft,
    Up,
    UpperRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Neutral,
}

struct DebugHidDevice<'a>(&'a HidDevice);

impl<'a> Debug for DebugHidDevice<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Ok(Some(product)) = self.0.get_product_string() {
            write!(f, "{}", product)
        } else {
            write!(f, "")
        }
    }
}

mod joycon_device {
    use super::*;
    use std::ops::Deref;

    /// The JoyCon device in user's hand.
    pub enum JoyConDevice {
        JoyConR(HidDevice),
        JoyConL(HidDevice),
        // note: I'll to it later
        // ProCon(Arc<HidDevice>),
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
                Self::PRODUCT_ID_JOYCON_L => Ok(JoyConDevice::JoyConL(device)),
                Self::PRODUCT_ID_JOYCON_R => Ok(JoyConDevice::JoyConR(device)),
                other => Err(JoyConDeviceError::InvalidProductID(other))?,
            }
        }
    }

    impl Deref for JoyConDevice {
        type Target = HidDevice;

        fn deref(&self) -> &Self::Target {
            match self {
                JoyConDevice::JoyConR(hd) => hd,
                JoyConDevice::JoyConL(hd) => hd,
            }
        }
    }

    impl Debug for JoyConDevice {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(f,
                   "{}({:?})",
                   match self {
                       JoyConDevice::JoyConR(_) => "JoyConR",
                       JoyConDevice::JoyConL(_) => "JoyConL",
                   },
                   DebugHidDevice(&*self))
        }
    }
}

// mod joycon {
//     use super::*;
//
//     /// The controller user uses to play with.
//     #[derive(Clone)]
//     pub enum JoyCon {
//         JoyConR(Arc<HidDevice>),
//         JoyConL(Arc<HidDevice>),
//         JoyConLR {
//             joycon_l: Arc<HidDevice>,
//             joycon_r: Arc<HidDevice>,
//         },
//         // note: I'll do it later.
//         // Procon(Arc<HidDevice>)
//         //     procon: Arc<HidDevice>,
//         // }
//     }
//
//     impl From<JoyConDevice> for JoyCon {
//         fn from(jd: JoyConDevice) -> Self {
//             match jd {
//                 JoyConDevice::JoyConR(r) => JoyCon::JoyConR(r),
//                 JoyConDevice::JoyConL(l) => JoyCon::JoyConL(l),
//             }
//         }
//     }
//
//     impl Debug for JoyCon {
//         fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//             write!(f,
//                    "{}({:?})",
//                    match self {
//                        JoyCon::JoyConR { .. } => "JoyConR",
//                        JoyCon::JoyConL { .. } => "JoyConL",
//                        JoyCon::JoyConLR { .. } => "JoyConLR",
//                    },
//                    match self {
//                        JoyCon::JoyConL(joycon) => vec![DebugHidDevice(joycon)],
//                        JoyCon::JoyConR(joycon) => vec![DebugHidDevice(joycon)],
//                        JoyCon::JoyConLR { joycon_l, joycon_r, .. } =>
//                            vec![DebugHidDevice(joycon_l), DebugHidDevice(joycon_r)]
//                    })
//         }
//     }
//
//     impl JoyCon {
//         pub fn write(&self, data: &[u8]) -> JoyConResult<usize> {
//             let u =
//                 match self {
//                     JoyCon::JoyConR(hd) => hd.write(data)?,
//                     JoyCon::JoyConL(hd) => hd.write(data)?,
//                     // note: need reconsideration
//                     JoyCon::JoyConLR { joycon_l, joycon_r } => {
//                         joycon_l.write(data)?;
//                         joycon_r.write(data)?
//                     }
//                 };
//
//             Ok(u)
//         }
//     }
// }

mod driver {
    use super::*;
    pub use global_packet_number::GlobalPacketNumber;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Rotation {
        ///    .-..WHHHHHWa-.
        ///     zrrrrrrrrrvvzzVHJ.
        ///   (gkrrrrrrrrrtrrrrrzWn.
        ///   dMKrQWNmrrtrrtrtrrrvXH,
        ///   WMRtZH#VrtrtrrrtrrtrrzW-
        ///   WMRttrrtrrwQmmyrrtrrrru.
        ///   WMRtrtttrrMHWWMkrrtrrrwl
        ///   WMRrtrrrrrTMMMBvrrrrrrrI
        ///   JMSrOdMMMmrrrrrQHMMHwrrI
        ///   dMktdHHd@#rtrrrWHWdMSrwI
        ///   dMSrrdHMBwrAQArwHMM8rrrI
        ///   dMSrrrrrrrHHWHNwrrrrtrrI
        ///   WM0ttrtttrMHgH#wrrtrrrrI
        ///   HM0rrtrtrrrw0wrrtrttrrrI
        ///   HM0rrrtrrtrrrrrtrrrrtrvI
        ///   HM0rtrrrrrvzzzvvvrrrrtvr
        ///  .HMkrrtrrvQgHNHmyzvvrrrwr
        ///  .HMZrtrrQMMHHHH##Nkwrrrvr
        ///  .@NZrrrwMMHHHHHHHH#Zvrrvr
        ///  .@NZrrrdMHHHHHHHH##uwrrvr
        ///  .@NwrrrrUMHHHHHH#MSXrrrvr
        ///  .@NwrtrrrXHMMMMM8Xwrrtrvr
        ///   dNwrtrrrrrvrrrrrrrrrrrw)
        ///   dNrrrtrrrrrrrrrrrrrrrrw%
        ///   dMrrrrAWHHkrrrrrrrrrrrw)
        ///  .WMrrrwWMMMNkrrrrrrrrrrw)
        ///  .@MrrrrVMMM9rrrrrrtrrrrz)
        ///  .HMrrrrrrrrrrrrrtrrrrrrw)
        ///  .@Mrrrrrtrrrrtrrrtrrrrrw}
        ///  ,HMrrrrrrrrtrrrrrrrrrrrX
        ///  ,HWrrrrrrrrrrrtrrrrrrrw}
        ///  .Bvrrrrtrrtrrrrrtrrrrw>
        ///    (rrrrrrrrrtrrrrrrvZ^
        ///    (vrrrrrrrrrrrrrvZ^
        ///    (vvvvrrvrvvwZ7^
        ///    (OzuuuXuuuX
        Portrait,
        ///  `````. ``  `  ` .WQQQQQQQQQmmggggga+gQQQQmQmmmmmmgggggg&(JJJJJ.+++&+++++J,` `
        ///   ````.._++zOOrrvwVUUUUUUUUUUUUUBBBBBWWWWWWWWWWHHHHHHHHHHHHHHHHHHHHHHHMMMM5((-
        ///   ````._(zuXvrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrttrrtttrtrrwwrrtrttrrwmOrrrvZ
        ///  ` ```.~(zuXvrrrrrrrrrrtrrtwAQmyrrrrrtrrwwwwrrrrrtrttrrtrrQMHMMkrrtrrdMHMSrrrw,
        ///   ````.~(zXXvrrrrrrrrrrrrrrWMNMNRrrrrwQdHMMMNmwvrrrrtrtrtrdHkkM8rtrtrrZ9rrrrvwF
        ///  ` ```.~(zuuvrrrrrrtrrtrrrrWMMMNSrrrvdMHHHHHHMNwvrrrrrAggyrZW90rAQgyrrtrrrrrrwF
        ///   ````.~(jVwzrrrrtrrrrtrtrrrrZZrrrrvwHHHHHHHHHM#uvrtrdMMMHRrrrrdHMMMKrrtrrtrvd]
        ///  `````.~(+z!OvrrrrrrrrrrrtrrrrrrrrvrwM#HHHHHHH#HzvrrrdMHHMSrrrrdMHNM9rtrrtrrvd!
        ///   ````.._<~ .zvrrrtrrtrrrrtrrrrrrrrrvXMH#HHHHMBzvrrrtrrwwrwgHHmrrZVrrrrrtrrvqF
        ///   ```...~`   .OvrrrrrrrrrrrrrtrrrrrrvvXXHMMMHUzvvrrrrttrrrdMkHMKrrrtrtrrrrvqY
        ///  ` ```.`      `?wvrrrrrrrrrrrrrrrrrrrrrrvwwvvrrrrrtrrrrtrrZM@@HXrrrrrrrrwXd^
        ///   `              _7XwvrrrrrrrrrrrrrrrrrrrrrrrrrrrrrtrrrrrrrrrrrrrrrrrwwXV=
        ///                      _?7777777777777777777777777777777777777OOOOO77=!(=`
        Landscape,
    }

    mod global_packet_number {
        use std::ops::Add;

        /// Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct GlobalPacketNumber(pub u8);

        impl GlobalPacketNumber {
            pub fn next(self) -> GlobalPacketNumber {
                self + GlobalPacketNumber(1)
            }
        }

        impl Default for GlobalPacketNumber {
            fn default() -> Self {
                GlobalPacketNumber(0x0)
            }
        }

        impl Add for GlobalPacketNumber {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                let (GlobalPacketNumber(raw), GlobalPacketNumber(raw_rhs))
                    = (self, rhs);
                let res = raw.wrapping_add(raw_rhs);

                GlobalPacketNumber(res)
            }
        }

        impl Into<u8> for GlobalPacketNumber {
            fn into(self) -> u8 {
                self.0
            }
        }
    }

    #[derive(Debug)]
    /// The controller user uses to play with.
    pub struct SimpleJoyConDriver {
        /// The controller user uses
        pub joycon: JoyConDevice,
        /// rotation of controller
        pub rotation: Rotation,
        /// Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
        global_packet_number: GlobalPacketNumber,
    }

    impl SimpleJoyConDriver {
        pub fn new(joycon: JoyConDevice) -> Self {
            Self {
                joycon,
                rotation: Rotation::Portrait,
                global_packet_number: GlobalPacketNumber::default(),
            }
        }
    }

    pub trait JoyConDriver {
        fn write(&self, data: &[u8]) -> JoyConResult<usize>;

        fn read(&self, buf: &mut [u8]) -> JoyConResult<usize>;

        fn global_packet_number(&self) -> u8;

        fn increase_global_packet_number(&mut self);

        fn send_command_raw(&mut self, command: u8, sub_command: u8, data: &[u8]) -> JoyConResult<usize> {
            let mut buf = [0x0; 0x40];
            // set command
            buf[0] = command;
            // set packet number
            buf[1] = self.global_packet_number();
            // increase packet number
            self.increase_global_packet_number();

            // set sub command
            buf[10] = sub_command;
            dbg!(buf.to_vec());
            dbg!(&data);
            // set data
            buf[11..11 + data.len()].copy_from_slice(data);

            self.write(&buf)
        }

        fn send_sub_command_raw(&mut self, sub_command: u8, data: &[u8]) -> JoyConResult<usize> {
            self.send_command_raw(1, sub_command, data)
        }

        fn send_command(&mut self, command: Command, sub_command: SubCommand, data: &[u8]) -> JoyConResult<usize> {
            let command = command as u8;
            let sub_command = sub_command as u8;

            self.send_command_raw(command, sub_command, data)
        }

        fn send_sub_command(&mut self, sub_command: SubCommand, data: &[u8]) -> JoyConResult<usize> {
            self.send_command(Command::RumbleAndSubCommand, sub_command, data)
        }
    }

    impl JoyConDriver for SimpleJoyConDriver {
        fn write(&self, data: &[u8]) -> JoyConResult<usize> {
            Ok(self.joycon.write(data)?)
        }

        fn read(&self, buf: &mut [u8]) -> JoyConResult<usize> {
            Ok(self.joycon.read(buf)?)
        }

        fn global_packet_number(&self) -> u8 {
            self.global_packet_number.into()
        }

        fn increase_global_packet_number(&mut self) {
            self.global_packet_number = self.global_packet_number.next();
        }
    }

    pub mod standard_input_report {
        use super::*;
        use std::convert::TryFrom;
        use std::fmt::Error;

        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
        pub enum BatteryLevel {
            Empty,
            Critical,
            Low,
            Medium,
            Full,
        }

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct Battery {
            pub level: BatteryLevel,
            pub is_charging: bool,
        }

        impl TryFrom<u8> for Battery {
            type Error = JoyConError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let is_charging = value % 2 == 1;
                let value = if is_charging {
                    value - 1
                } else { value };
                let level = match value {
                    0 => BatteryLevel::Empty,
                    2 => BatteryLevel::Critical,
                    4 => BatteryLevel::Low,
                    6 => BatteryLevel::Medium,
                    8 => BatteryLevel::Full,
                    _ => Err(JoyConReportError::InvalidStandardFullReport(InvalidStandardFullReport::Battery(value)))?
                };

                Ok(Battery { level, is_charging })
            }
        }

        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
        pub enum Device {
            JoyCon,
            ProConOrChargingGrip,
        }

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct ConnectionInfo {
            device: Device,
            is_powered: bool,
        }

        impl TryFrom<u8> for ConnectionInfo {
            type Error = JoyConError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let device = match (value >> 1) & 3 {
                    3 => Device::JoyCon,
                    0 => Device::ProConOrChargingGrip,
                    _ => Err(InvalidStandardFullReport::ConnectionInfo(value))?
                };
                let is_powered = (value & 1) == 1;

                Ok(ConnectionInfo {
                    device,
                    is_powered,
                })
            }
        }

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct PushedButtons {
            right: Vec<Buttons>,
            shared: Vec<Buttons>,
            left: Vec<Buttons>,
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
        }

        impl From<[u8; 3]> for PushedButtons {
            fn from(value: [u8; 3]) -> Self {
                let right_val = value[0];
                let shared_val = value[1];
                let left_val = value[2];

                let right = PushedButtons::RIGHT_BUTTONS.iter()
                    .enumerate()
                    .filter(|(idx, _)| {
                        let idx = 2u8.pow(*idx as u32) as u8;
                        right_val & idx == idx
                    })
                    .map(|(_, b)| b.clone())
                    .collect();
                let shared = PushedButtons::SHARED_BUTTONS.iter()
                    .enumerate()
                    .filter(|(idx, _)| {
                        let idx = 2u8.pow(*idx as u32) as u8;
                        shared_val & idx == idx
                    })
                    .map(|(_, b)| b.clone())
                    .collect();
                let left = PushedButtons::LEFT_BUTTONS.iter()
                    .enumerate()
                    .filter(|(idx, _)| {
                        let idx = 2u8.pow(*idx as u32) as u8;
                        left_val & idx == idx
                    })
                    .map(|(_, b)| b.clone())
                    .collect();

                PushedButtons {
                    right,
                    shared,
                    left,
                }
            }
        }

        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        pub struct AnalogStickData {
            horizontal: u16,
            vertical: u16,
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

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct AxisData {
            /// Acceleration to X measured in Gs
            pub accel_x: f32,
            /// Acceleration to Y measured in Gs
            pub accel_y: f32,
            /// Acceleration to Z measured in Gs
            pub accel_z: f32,
            /// Rotation of X measured in degree/s
            pub gyro_1: f32,
            /// Rotation of Y measured in degree/s
            pub gyro_2: f32,
            /// Rotation of Z measured in degree/s
            pub gyro_3: f32,
        }

        impl From<[u8; 12]> for AxisData {
            fn from(value: [u8; 12]) -> Self {
                const SENSOR_RES: f32 = 65535.0;
                fn accel(raw: [u8; 2]) -> f32 {
                    let raw = i16::from_le_bytes([raw[0], raw[1]]);
                    const G_RANGE: f32 = 16384.0;

                    raw as f32 * G_RANGE / SENSOR_RES / 1000.0
                }
                let accel_x = accel([value[0], value[1]]);
                let accel_y = accel([value[2], value[3]]);
                let accel_z = accel([value[4], value[5]]);

                fn gyro(raw: [u8; 2]) -> f32 {
                    let raw = i16::from_le_bytes([raw[0], raw[1]]);
                    const G_GAIN: f32 = 4588.0;

                    raw as f32 * G_GAIN / SENSOR_RES
                }
                let gyro_1 = gyro([value[6], value[7]]);
                let gyro_2 = gyro([value[8], value[9]]);
                let gyro_3 = gyro([value[10], value[11]]);

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

        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub enum ExtraData {
            /// Standard input reports used for subcommand replies.
            SubCommand {
                ack_byte: u8,
                sub_command_id: u8,
                reply: [u8; 35],
            },
            /// NFC/IR MCU FW update input report.
            NFC_IR {
                report: [u8; 37]
            },
            /// Standard full mode - input reports with IMU data instead of subcommand replies.
            /// Pushes current state @60Hz, or @120Hz if Pro Controller.
            Full {
                axis_data: [AxisData; 3],
            },
            /// NFC/IR MCU mode. Pushes large packets with standard input report + NFC/IR MCU data input report.
            Full_NFC_IR {
                axis_data: [AxisData; 3],
                report: [u8; 313],
            },
        }

        impl Debug for ExtraData {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f,
                       "{}",
                       match self {
                           ExtraData::SubCommand {
                               ack_byte,
                               sub_command_id,
                               reply
                           } => format!("SubCommand {{ ack_byte: {}, sub_command_id: {}, reply: {:?} }}",
                                         ack_byte,
                                         sub_command_id,
                                         &reply[..]),
                           ExtraData::NFC_IR {
                               report
                           } => format!("NFC_IR {{ report: {:?} }}", &report[..]),
                           ExtraData::Full {
                               axis_data,
                           } => format!("Full {{ axis_data: {:?} }}", &axis_data[..]),
                           ExtraData::Full_NFC_IR {
                               axis_data,
                               report,
                           } => format!("Full_NFC_IR {{ axis_data: {:?}, report: {:?} }}", axis_data, &report[..]),
                       }
                )
            }
        }

        /// get extra data from packet data
        impl TryFrom<&[u8]> for ExtraData {
            type Error = JoyConError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                let input_report_id = value.get(0)
                    .ok_or(InvalidStandardFullReport::InvalidReport(value.to_vec()))?
                    .clone();

                let len = match input_report_id {
                    // Sub-command reply
                    0x21 => 50,
                    // NFC/IR MCU FW update input report
                    0x23 => 50,
                    // 6-Axis data
                    0x30 | 0x32 | 0x33 => 49,
                    // NFC/IR MCU mode
                    0x31 => 362,
                    _ => Err(InvalidStandardFullReport::InvalidInputReportId(input_report_id))?,
                };
                if value.len() < len {
                    Err(InvalidStandardFullReport::InvalidReport(value.to_vec()))?
                }

                match input_report_id {
                    0x21 => {
                        let ack_byte = value[13];
                        let sub_command_id = value[14];
                        let mut reply = [0x00; 35];
                        reply.copy_from_slice(&value[15..50]);

                        Ok(ExtraData::SubCommand { ack_byte, sub_command_id, reply })
                    }
                    0x23 => {
                        let mut report = [0x00; 37];
                        report.copy_from_slice(&value[13..50]);
                        Ok(ExtraData::NFC_IR { report })
                    }
                    0x30 | 0x32 | 0x33 | 0x31 => {
                        let latest = {
                            let mut latest = [0x00; 12];
                            latest.copy_from_slice(&value[13..25]);
                            latest
                        };
                        let a_5ms_older = {
                            let mut latest = [0x00; 12];
                            latest.copy_from_slice(&value[25..37]);
                            latest
                        };
                        let a_10ms_older = {
                            let mut latest = [0x00; 12];
                            latest.copy_from_slice(&value[37..49]);
                            latest
                        };

                        let axis_data = [
                            AxisData::from(latest),
                            AxisData::from(a_5ms_older),
                            AxisData::from(a_10ms_older)
                        ];

                        match input_report_id {
                            0x31 => {
                                let mut report = [0; 313];
                                report.copy_from_slice(&value[49..363]);
                                Ok(ExtraData::Full_NFC_IR { axis_data, report })
                            }
                            0x30 | 0x32 | 0x33 => Ok(ExtraData::Full { axis_data }),
                            _ => unreachable!()
                        }
                    }
                    _ => unreachable!()
                }
            }
        }

        #[derive(Debug, Clone)]
        pub struct StandardInputReport {
            input_report_id: u8,
            timer: u8,
            battery: Battery,
            connection_info: ConnectionInfo,
            pushed_buttons: PushedButtons,
            left_analog_stick_data: AnalogStickData,
            right_analog_stick_data: AnalogStickData,
            vibrator_input_report: u8,
            extra_data: ExtraData,
        }

        impl StandardInputReport {
            pub fn parse<P>(data: &[u8], parser: P) -> JoyConResult<StandardInputReport>
                where P: Fn(&[u8]) -> JoyConResult<StandardInputReport> {
                parser(data)
            }
        }

        pub trait StandardInputReportMode: JoyConDriver {
            fn set_standard_input_report_mode(&mut self) -> JoyConResult<usize> {
                self.send_command(Command::RumbleAndSubCommand, SubCommand::SetInputReportMode, &[0x30])
            }

            fn read_update(&self) -> JoyConResult<StandardInputReport>;
        }

        impl StandardInputReportMode for SimpleJoyConDriver {
            fn read_update(&self) -> JoyConResult<StandardInputReport> {
                let mut buf = [0x00; 361];
                self.read(&mut buf)?;

                StandardInputReport::parse(&buf, |report| {
                    if report.len() < 13 {
                        Err(InvalidStandardFullReport::InvalidReport(report.to_vec()))?
                    }

                    let input_report_id = report[0];
                    let timer = report[1];

                    let (battery, connection_info) = {
                        let value = report[2];
                        let high_nibble = value / 16;
                        let low_nibble = value % 16;

                        (Battery::try_from(high_nibble)?, ConnectionInfo::try_from(low_nibble)?)
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

                    let extra_data = ExtraData::try_from(report)?;

                    Ok(StandardInputReport {
                        input_report_id,
                        timer,
                        battery,
                        connection_info,
                        pushed_buttons,
                        left_analog_stick_data,
                        right_analog_stick_data,
                        vibrator_input_report,
                        extra_data,
                    })
                })
            }
        }
    }

    pub mod simple_hid_mode {
        use super::*;
        use crate::result::JoyConResult;

        /// Pushed buttons and stick direction.
        #[derive(Debug, Clone)]
        pub struct SimpleHidUpdate {
            pub pushed_buttons: Vec<Buttons>,
            pub stick_direction: StickDirection,
        }

        impl SimpleHidUpdate {
            pub fn parse<P>(data: &[u8], parser: P) -> JoyConResult<Self>
                where P: Fn(&[u8]) -> JoyConResult<SimpleHidUpdate> {
                parser(data)
            }
        }

        pub trait SimpleHidMode {
            /// Pushes updates with every button press
            fn set_simple_hid_mode(&mut self) -> JoyConResult<usize>;

            fn read_update(&self) -> JoyConResult<SimpleHidUpdate>;
        }

        const BUTTONS_JOYCON_L_1: [Buttons; 6] = [
            Buttons::Left,
            Buttons::Down,
            Buttons::Up,
            Buttons::Right,
            Buttons::SL,
            Buttons::SR];
        const BUTTONS_JOYCON_L_2: [Buttons; 8] = [
            Buttons::Minus,
            Buttons::Plus,
            Buttons::LStick,
            Buttons::RStick,
            Buttons::Home,
            Buttons::Capture,
            Buttons::L,
            Buttons::ZL];
        const BUTTONS_JOYCON_R_1: [Buttons; 6] = [
            Buttons::A,
            Buttons::X,
            Buttons::B,
            Buttons::Y,
            Buttons::SL,
            Buttons::SR];
        const BUTTONS_JOYCON_R_2: [Buttons; 8] = [
            Buttons::Minus,
            Buttons::Plus,
            Buttons::LStick,
            Buttons::RStick,
            Buttons::Home,
            Buttons::Capture,
            Buttons::R,
            Buttons::ZR];
        const STICK_JOYCON_L_PORTRAIT: [StickDirection; 9] = [
            StickDirection::Right,
            StickDirection::BottomRight,
            StickDirection::Bottom,
            StickDirection::BottomLeft,
            StickDirection::Left,
            StickDirection::UpperLeft,
            StickDirection::Up,
            StickDirection::UpperRight,
            StickDirection::Neutral];
        const STICK_JOYCON_L_LANDSCAPE: [StickDirection; 9] = [
            StickDirection::Up,
            StickDirection::UpperRight,
            StickDirection::Right,
            StickDirection::BottomRight,
            StickDirection::Bottom,
            StickDirection::BottomLeft,
            StickDirection::Left,
            StickDirection::UpperLeft,
            StickDirection::Neutral];
        const STICK_JOYCON_R_PORTRAIT: [StickDirection; 9] = [
            StickDirection::Left,
            StickDirection::UpperLeft,
            StickDirection::Up,
            StickDirection::UpperRight,
            StickDirection::Right,
            StickDirection::BottomRight,
            StickDirection::Bottom,
            StickDirection::BottomLeft,
            StickDirection::Neutral];
        const STICK_JOYCON_R_LANDSCAPE: [StickDirection; 9] = [
            StickDirection::Up,
            StickDirection::UpperRight,
            StickDirection::Right,
            StickDirection::BottomRight,
            StickDirection::Bottom,
            StickDirection::BottomLeft,
            StickDirection::Left,
            StickDirection::UpperLeft,
            StickDirection::Neutral];

        impl SimpleHidMode for SimpleJoyConDriver {
            fn set_simple_hid_mode(&mut self) -> JoyConResult<usize> {
                self.send_command(Command::RumbleAndSubCommand, SubCommand::SetInputReportMode, &[0x3F])
            }

            fn read_update(&self) -> JoyConResult<SimpleHidUpdate> {
                let mut buf = [0x00; 0x40];
                self.read(&mut buf)?;

                SimpleHidUpdate::parse(&buf, |report| {
                    let buttons_1 = match &self.joycon {
                        JoyConDevice::JoyConL(_) => &BUTTONS_JOYCON_L_1,
                        JoyConDevice::JoyConR(_) => &BUTTONS_JOYCON_R_1,
                    };
                    let buttons_2 = match &self.joycon {
                        JoyConDevice::JoyConL(_) => &BUTTONS_JOYCON_L_2,
                        JoyConDevice::JoyConR(_) => &BUTTONS_JOYCON_R_2,
                    };
                    let stick_directions = match (&self.joycon, &self.rotation) {
                        (JoyConDevice::JoyConL(_), &Rotation::Portrait) => &STICK_JOYCON_L_PORTRAIT,
                        (JoyConDevice::JoyConL(_), &Rotation::Landscape) => &STICK_JOYCON_L_LANDSCAPE,
                        (JoyConDevice::JoyConR(_), &Rotation::Portrait) => &STICK_JOYCON_R_PORTRAIT,
                        (JoyConDevice::JoyConR(_), &Rotation::Landscape) => &STICK_JOYCON_R_LANDSCAPE,
                    };

                    let button_value_1 = report.get(1)
                        .ok_or(JoyConReportError::InvalidSimpleHIDReport(report.to_vec()))?;
                    let button_value_2 = report.get(2)
                        .ok_or(JoyConReportError::InvalidSimpleHIDReport(report.to_vec()))?;

                    let pushed_buttons = {
                        let mut pushed_buttons = Vec::new();

                        let mut pushed_buttons_1 = buttons_1.iter()
                            .enumerate()
                            .filter(|(i, _)| {
                                let idx = 2u8.pow(*i as u32) as u8;
                                button_value_1 & idx == idx
                            })
                            .map(|(_, b)| b.clone())
                            .collect::<Vec<_>>();
                        pushed_buttons.append(&mut pushed_buttons_1);

                        let mut pushed_buttons_2 = buttons_2.iter()
                            .enumerate()
                            .filter(|(i, _)| {
                                let idx = 2u8.pow(*i as u32) as u8;
                                button_value_2 & idx == idx
                            })
                            .map(|(_, b)| b.clone())
                            .collect::<Vec<_>>();
                        pushed_buttons.append(&mut pushed_buttons_2);

                        pushed_buttons
                    };

                    let stick_value = report.get(3)
                        .ok_or(JoyConReportError::InvalidSimpleHIDReport(report.to_vec()))?;

                    let stick_direction = stick_directions.get(stick_value.clone() as usize)
                        .ok_or(JoyConReportError::InvalidSimpleHIDReport(report.to_vec()))?
                        .clone();

                    Ok(SimpleHidUpdate {
                        pushed_buttons,
                        stick_direction,
                    })
                })
            }
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
    pub enum Command {
        RumbleAndSubCommand = 1,
        NFC_IR_MCU_FW_Update = 3,
        Rumble = 16,
        RumbleAndRequestSpecificDataFromThe_NFC_IR_MCU = 17,
    }

    /// ref. https://github.com/dekuNukem/Nintendo_Switch_Reverse_Engineering/blob/master/bluetooth_hid_subcommands_notes.md
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
    pub enum SubCommand {
        GetOnlyControllerState = 0,
        BluetoothManualPairing = 1,
        RequestDeviceInfo = 2,
        SetInputReportMode = 3,
        TriggerButtonsElapsedTime = 4,
        GetPageListState = 5,
        SetHCIState = 6,
        ResetPairingInfo = 7,
        SetShipmentLowPowerState = 8,
        SPIFlashRead = 10,
        SPIFlashWrite = 11,
        SPISectorErase = 12,
        ResetNFC_IR_MCU = 32,
        Set_NFC_IR_MCUConfiguration = 33,
        Set_NFC_IR_MCUState = 34,
        Get_x28_NFC_IR_MCUData = 41,
        Set_GPIO_PinOutputValue = 42,
        Get_x29_NFC_IR_MCUData = 43,
        SetPlayerLights = 48,
        GetPlayerLights = 49,
        SetHOMELight = 56,
        EnableIMU = 64,
        SetIMUSensitivity = 65,
        WriteToIMURegisters = 66,
        ReadIMURegisters = 67,
        EnableVibration = 72,
        GetRegulatedVoltage = 80,
        SetGPIOPinOutputValue = 81,
        GetGPIOPinInput_OutputValue = 82,
    }
}

pub struct JoyConManager {
    hidapi: Arc<HidApi>,
    pub connected_joycon_devices: Vec<JoyConDevice>,
}

impl JoyConManager {
    pub fn new() -> JoyConResult<Self> {
        let hidapi = HidApi::new()?;
        let devices = hidapi.device_list()
            .flat_map(|di|
                JoyConDevice::new(di, &hidapi)
            )
            .collect();

        Ok(Self {
            hidapi: Arc::new(hidapi),
            connected_joycon_devices: devices,
        })
    }
}