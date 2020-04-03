use crate::prelude::*;

pub use joycon_device::JoyConDevice;
// pub use joycon::JoyCon;
pub use driver::{
    Rotation,
    JoyConDriver,
    GlobalPacketNumber,
    SimpleJoyConDriver,
    Command,
    SubCommand,
    input_report_mode,
    lights,
};

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

mod driver {
    use super::*;
    pub use global_packet_number::GlobalPacketNumber;
    pub use joycon_features::{JoyConFeatures, IMUFeature};
    use std::collections::HashSet;

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

    /// Features on Joy-Cons which needs to set up.
    /// ex. IMU(6-Axis sensor), NFC/IR, LED, Vibration
    pub mod joycon_features {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum JoyConFeatures {
            IMUFeature(IMUFeature),
            Vibration,
        }

        pub use imu_sensitivity::IMUFeature;

        pub mod imu_sensitivity {
            /// Gyroscope sensitivity
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum GyroscopeSensitivity {
                PM250dps = 0x00,
                PM500dps = 0x01,
                PM1000dps = 0x02,
                PM2000dps = 0x03,
            }

            impl Default for GyroscopeSensitivity {
                fn default() -> Self {
                    GyroscopeSensitivity::PM2000dps
                }
            }

            /// Accelerometer sensitivity
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum AccelerometerSensitivity {
                PM8G = 0x00,
                PM4G = 0x01,
                PM2G = 0x02,
                PM16G = 0x03,
            }

            impl Default for AccelerometerSensitivity {
                fn default() -> Self {
                    AccelerometerSensitivity::PM8G
                }
            }

            /// Gyroscope performance rate
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum GyroscopePerformanceRate {
                F833Hz = 0x00,
                F208Hz = 0x01,
            }

            impl Default for GyroscopePerformanceRate {
                fn default() -> Self {
                    GyroscopePerformanceRate::F208Hz
                }
            }

            /// Accelerometer Anti-aliasing filter bandwidth
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum AccelerometerAntiAliasingFilterBandwidth {
                F200Hz = 0x00,
                F100Hz = 0x01,
            }

            impl Default for AccelerometerAntiAliasingFilterBandwidth {
                fn default() -> Self {
                    AccelerometerAntiAliasingFilterBandwidth::F100Hz
                }
            }

            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct IMUFeature {
                pub gyroscope_sensitivity: GyroscopeSensitivity,
                pub accelerometer_sensitivity: AccelerometerSensitivity,
                pub gyroscope_performance_rate: GyroscopePerformanceRate,
                pub accelerometer_anti_aliasing_filter_bandwidth: AccelerometerAntiAliasingFilterBandwidth,
            }

            impl Into<[u8; 4]> for IMUFeature {
                fn into(self) -> [u8; 4] {
                    let IMUFeature {
                        gyroscope_sensitivity,
                        accelerometer_sensitivity,
                        gyroscope_performance_rate,
                        accelerometer_anti_aliasing_filter_bandwidth,
                    } = self;

                    [
                        gyroscope_sensitivity as u8,
                        accelerometer_sensitivity as u8,
                        gyroscope_performance_rate as u8,
                        accelerometer_anti_aliasing_filter_bandwidth as u8
                    ]
                }
            }
        }
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
        enabled_features: HashSet<JoyConFeatures>,
        /// Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
        global_packet_number: GlobalPacketNumber,
    }

    impl SimpleJoyConDriver {
        pub fn new(joycon: JoyConDevice) -> JoyConResult<Self> {
            let mut driver = Self {
                joycon,
                rotation: Rotation::Portrait,
                enabled_features: HashSet::new(),
                global_packet_number: GlobalPacketNumber::default(),
            };

            driver.reset()?;

            Ok(driver)
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

        /// disable Joy-Con's features
        fn reset(&mut self) -> JoyConResult<()> {
            // disable IMU (6-Axis sensor)
            self.send_sub_command(SubCommand::EnableIMU, &[0x00])?;
            // disable vibration
            self.send_sub_command(SubCommand::EnableVibration, &[0x00])?;

            Ok(())
        }

        fn enable_features(&mut self, feature: JoyConFeatures) -> JoyConResult<()>;

        fn enabled_features(&self) -> &HashSet<JoyConFeatures>;

        fn devices(&self) -> Vec<&JoyConDevice>;
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

        fn enable_features(&mut self, feature: JoyConFeatures) -> JoyConResult<()> {
            match feature {
                JoyConFeatures::IMUFeature(feature) => {
                    let data: [u8; 4] = feature.into();
                    // enable IMU
                    self.send_sub_command(SubCommand::EnableIMU, &[0x01])?;
                    // set config
                    self.send_sub_command(SubCommand::SetIMUSensitivity, &data)?;
                }
                JoyConFeatures::Vibration => {
                    // enable vibration
                    self.send_sub_command(SubCommand::EnableVibration, &[0x01])?;
                }
            }

            self.enabled_features.insert(feature);

            Ok(())
        }

        fn enabled_features(&self) -> &HashSet<JoyConFeatures> {
            &self.enabled_features
        }

        fn devices(&self) -> Vec<&JoyConDevice> {
            vec![&self.joycon]
        }
    }

    pub mod input_report_mode {
        use super::*;
        pub use common::*;
        use std::convert::TryFrom;
        use std::ops::{Deref, DerefMut};

        mod common {
            use super::*;
            use std::convert::TryFrom;

            #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
            pub enum BatteryLevel {
                Empty,
                Critical,
                Low,
                Medium,
                Full,
            }

            #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
                        _ => Err(JoyConReportError::InvalidStandardInputReport(InvalidStandardInputReport::Battery(value)))?
                    };

                    Ok(Battery { level, is_charging })
                }
            }

            #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
            pub enum Device {
                JoyCon,
                ProConOrChargingGrip,
            }

            #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
                        _ => Err(InvalidStandardInputReport::ConnectionInfo(value))?
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

            #[derive(Debug, Clone, PartialEq)]
            pub struct CommonReport {
                input_report_id: u8,
                timer: u8,
                battery: Battery,
                connection_info: ConnectionInfo,
                pushed_buttons: PushedButtons,
                left_analog_stick_data: AnalogStickData,
                right_analog_stick_data: AnalogStickData,
                vibrator_input_report: u8,
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

        pub trait InputReportMode<D: JoyConDriver>: Deref<Target=D> + DerefMut<Target=D> {
            type Mode: InputReportMode<D>;
            type Report;

            /// set Joy-Con's input report mode and return instance
            fn set(driver: D) -> JoyConResult<Self::Mode>;

            /// read Joy-Con's input report
            fn read_input_report(&self) -> JoyConResult<Self::Report>;
            // {
            //     // read
            //     let mut buf = [0x00; 362];
            //     self.read(&mut buf)?;
            //     // convert
            //     Self::Report::try_from(buf)
            // }
        }

        /// Standard Input Report
        pub struct StandardInputReport<EX: TryFrom<[u8; 349], Error=JoyConError>> {
            common: CommonReport,
            extra: EX,
        }

        impl<EX: TryFrom<[u8; 349], Error=JoyConError>> TryFrom<[u8; 362]> for StandardInputReport<EX> {
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

                Ok(StandardInputReport {
                    common,
                    extra,
                })
            }
        }

        impl<EX: TryFrom<[u8; 349], Error=JoyConError> + Debug> Debug for StandardInputReport<EX> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "StandardInputReport {{ common: {:?}, extra: {:?} }}",
                    &self.common,
                    &self.extra,
                )
            }
        }

        /// Modes reply standard input report
        pub trait StandardInputReportMode<D>: Deref<Target=D> + DerefMut<Target=D> where D: JoyConDriver {
            const SUB_COMMAND: SubCommand;
            const ARGS: Self::ArgsType;
            type ArgsType: AsRef<[u8]>;
            type Mode: StandardInputReportMode<D>;
            type ExtraReport: TryFrom<[u8; 349], Error=JoyConError>;

            /// Mode's setup operation.
            /// There are no needs to change Joy-Con's input report mode, leave it to `InputReportMode<D>::set()`.
            /// ex. Enable 6-Axis sensor
            fn setup(driver: D) -> JoyConResult<D>;

            /// Construct instance simply. Your implementation will be ->
            /// ```ignore
            /// fn construct(driver: SimpleJoyConDriver) -> Self::Mode{
            ///     SimpleJoyConDriver { driver }
            /// }
            /// ```
            fn construct(driver: D) -> Self::Mode;
        }

        /// - M: Actual Mode
        /// - D: JoyCon Driver
        /// - EXR: Actual Mode's Extra Report
        impl<M, D, EXR> InputReportMode<D> for M
            where M: StandardInputReportMode<D, ExtraReport=EXR>,
                  D: JoyConDriver,
                  EXR: TryFrom<[u8; 349], Error=JoyConError>
        {
            type Mode = M::Mode;
            type Report = StandardInputReport<EXR>;

            fn set(driver: D) -> JoyConResult<Self::Mode> {
                let mut driver = M::setup(driver)?;

                driver.send_sub_command(M::SUB_COMMAND, M::ARGS.as_ref())?;

                Ok(M::construct(driver))
            }

            fn read_input_report(&self) -> JoyConResult<Self::Report> {
                let mut buf = [0u8; 362];
                self.read(&mut buf)?;

                Self::Report::try_from(buf)
            }
        }

        mod sub_command_mode {
            use super::*;
            use std::marker::PhantomData;

            // todo write docs
            pub trait SubCommandReplyData: TryFrom<[u8; 35], Error=JoyConError> {
                type ArgsType: AsRef<[u8]>;
                const SUB_COMMAND: SubCommand;
                const ARGS: Self::ArgsType;
            }

            /// Replies to sub-commands
            #[derive(Clone)]
            pub struct SubCommandReport<RD>
                where RD: SubCommandReplyData
            {
                ack_byte: u8,
                sub_command_id: u8,
                reply: RD,
            }

            impl<RD> TryFrom<[u8; 349]> for SubCommandReport<RD>
                where RD: SubCommandReplyData
            {
                type Error = JoyConError;

                fn try_from(value: [u8; 349]) -> Result<Self, Self::Error> {
                    let ack_byte = value[0];
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

            pub struct SubCommandMode<D, RD>
                where D: JoyConDriver, RD: SubCommandReplyData
            {
                driver: D,
                _phantom: PhantomData<RD>,
            }

            impl<D, RD> Deref for SubCommandMode<D, RD>
                where D: JoyConDriver,
                      RD: SubCommandReplyData
            {
                type Target = D;

                fn deref(&self) -> &Self::Target {
                    &self.driver
                }
            }

            impl<D, RD> DerefMut for SubCommandMode<D, RD>
                where D: JoyConDriver,
                      RD: SubCommandReplyData
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.driver
                }
            }

            impl<D, RD> StandardInputReportMode<D> for SubCommandMode<D, RD>
                where D: JoyConDriver,
                      RD: SubCommandReplyData
            {
                const SUB_COMMAND: SubCommand = RD::SUB_COMMAND;
                const ARGS: Self::ArgsType = RD::ARGS;
                type ArgsType = RD::ArgsType;
                type Mode = SubCommandMode<D, RD>;
                type ExtraReport = SubCommandReport<RD>;

                fn setup(driver: D) -> JoyConResult<D> {
                    // do nothing
                    Ok(driver)
                }

                fn construct(driver: D) -> Self::Mode {
                    SubCommandMode {
                        driver,
                        _phantom: PhantomData,
                    }
                }
            }
        }
        // pub mod sub_command_mode {
        //     use super::*;
        //
        //     pub trait SubCommandMode<D: JoyConDriver> {
        //         const SUB_COMMAND: SubCommand;
        //         type ExtraReport: TryFrom<[u8; 349], Error=JoyConError>;
        //     }
        //
        //     impl<T: SubCommandMode<D, ExtraReport=EXR>, D: JoyConDriver, EXR: TryFrom<[u8; 349], Error=JoyConError>> InputReportMode<D> for T {
        //         type Mode = T;
        //         type Report = StandardInputReport<T::ExtraReport>;
        //
        //         fn set(driver: D) -> JoyConResult<Self::Mode> {
        //             unimplemented!()
        //         }
        //
        //         fn read_input_report(&self) -> JoyConResult<Self::Report> {
        //             unimplemented!()
        //         }
        //     }
        //
        //     // Joy-Con emitting standard input report with sub-command reply
        //     // pub struct SubCommandMode<D: JoyConDriver> {
        //     //     driver: D
        //     // }
        //     //
        //     // #[derive(Clone)]
        //     // pub struct SubCommandReply {
        //     //     ack_byte: u8,
        //     //     sub_command_id: u8,
        //     //     reply: [u8; 35],
        //     // }
        //     //
        //     // impl Debug for SubCommandReply {
        //     //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //     //         write!(f,
        //     //                "SubCommandReply {{ ack_byte: {}, sub_command_id: {}, reply: [{}] }}",
        //     //                self.ack_byte,
        //     //                self.sub_command_id,
        //     //                self.reply.iter().map(|&u| u as char).collect::<String>()
        //     //         )
        //     //     }
        //     // }
        //     //
        //     // impl TryFrom<[u8; 349]> for SubCommandReply {
        //     //     type Error = JoyConError;
        //     //
        //     //     fn try_from(value: [u8; 349]) -> Result<Self, Self::Error> {
        //     //         let ack_byte = value[0];
        //     //         let sub_command_id = value[1];
        //     //         let mut reply = [0x00; 35];
        //     //         reply.copy_from_slice(&value[2..37]);
        //     //
        //     //         Ok(SubCommandReply {
        //     //             ack_byte,
        //     //             sub_command_id,
        //     //             reply,
        //     //         })
        //     //     }
        //     // }
        //     //
        //     // impl<D: JoyConDriver> InputReportMode<D> for SubCommandMode<D> {
        //     //     type Mode = SubCommandMode<D>;
        //     //     type Report = StandardInputReport<SubCommandReply>;
        //     //
        //     //     fn set(driver: D) -> JoyConResult<Self::Mode> {
        //     //         let mut driver = driver;
        //     //         // set input report mode to sub command mode
        //     //         driver.send_sub_command(SUB_COMMAND::SetInputReportMode, &[0x21])?;
        //     //
        //     //         Ok(SubCommandMode { driver })
        //     //     }
        //     //
        //     //     fn read_input_report(&self) -> JoyConResult<Self::Report> {
        //     //         // read
        //     //         let mut buf = [0x00; 362];
        //     //         self.read(&mut buf)?;
        //     //         // convert
        //     //         Self::Report::try_from(buf)
        //     //     }
        //     // }
        //     //
        //     // impl<D: JoyConDriver> Deref for SubCommandMode<D> {
        //     //     type Target = D;
        //     //
        //     //     fn deref(&self) -> &Self::Target {
        //     //         &self.driver
        //     //     }
        //     // }
        //     //
        //     // impl<D: JoyConDriver> DerefMut for SubCommandMode<D> {
        //     //     fn deref_mut(&mut self) -> &mut Self::Target {
        //     //         &mut self.driver
        //     //     }
        //     // }
        // }

        pub mod standard_full_mode {
            use super::*;

            /// 6-Axis data. 3 frames of 2 groups of 3 Int16LE each. Group is Acc followed by Gyro.
            #[derive(Debug, Clone)]
            pub struct IMUData {
                data: [AxisData; 3]
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
                        AxisData::from(a_10ms_older)
                    ];

                    Ok(IMUData {
                        data: axis_data,
                    })
                }
            }

            /// Joy-Con emitting standard full report includes IMU(6-Axis sensor)
            pub struct StandardFullMode<D: JoyConDriver> {
                driver: D,
            }

            impl<D: JoyConDriver> Deref for StandardFullMode<D> {
                type Target = D;

                fn deref(&self) -> &Self::Target {
                    &self.driver
                }
            }

            impl<D: JoyConDriver> DerefMut for StandardFullMode<D> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.driver
                }
            }

            impl<D: JoyConDriver> InputReportMode<D> for StandardFullMode<D> {
                type Mode = StandardFullMode<D>;
                type Report = StandardInputReport<IMUData>;

                fn set(driver: D) -> JoyConResult<Self::Mode> {
                    let mut driver = driver;
                    // enable IMU(6-Axis sensor)
                    let imf_enabled = driver.enabled_features()
                        .iter()
                        .any(|jf| match jf {
                            JoyConFeatures::IMUFeature(_) => true,
                            _ => false,
                        });
                    if !imf_enabled {
                        driver.enable_features(JoyConFeatures::IMUFeature(IMUFeature::default()))?;
                    }
                    // set input report mode to standard full mode
                    driver.send_sub_command(SubCommand::SetInputReportMode, &[0x30])?;

                    Ok(StandardFullMode { driver })
                }

                fn read_input_report(&self) -> JoyConResult<Self::Report> {
                    // read
                    let mut buf = [0x00; 362];
                    self.read(&mut buf)?;
                    // convert
                    Self::Report::try_from(buf)
                }
            }
        }

        // pub mod simple_hid_mode {
        //     use super::*;
        //
        //     #[allow(non_camel_case_types)]
        //     #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
        //     pub enum SimpleHIDButton {
        //         Down,
        //         Right,
        //         Left,
        //         Up,
        //         SL,
        //         SR,
        //         Minus,
        //         Plus,
        //         LeftStick,
        //         RightStick,
        //         Home,
        //         Capture,
        //         L_R,
        //         ZL_ZR,
        //     }
        //
        //     const BUTTON1: [SimpleHIDButton; 6] = {
        //         use SimpleHIDButton::*;
        //         [Down, Right, Left, Up, SL, SR, ]
        //     };
        //     const BUTTON2: [SimpleHIDButton; 8] = {
        //         use SimpleHIDButton::*;
        //         [Minus, Plus, LeftStick, RightStick, Home, Capture, L_R, ZL_ZR, ]
        //     };
        //
        //     /// Hold your controller sideways so that SL, SYNC, and SR line up with the screen. Pushing the stick towards a direction in this table will cause that value to be sent.
        //     #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
        //     pub enum StickDirection {
        //         Up,
        //         UpperRight,
        //         Right,
        //         BottomRight,
        //         Bottom,
        //         BottomLeft,
        //         Left,
        //         UpperLeft,
        //         Neutral,
        //     }
        //
        //     impl TryFrom<u8> for StickDirection {
        //         type Error = JoyConError;
        //
        //         fn try_from(value: u8) -> Result<Self, Self::Error> {
        //             use StickDirection::*;
        //
        //             let button = match value {
        //                 0 => Up,
        //                 1 => UpperRight,
        //                 2 => Right,
        //                 3 => BottomRight,
        //                 4 => Bottom,
        //                 5 => BottomLeft,
        //                 6 => Left,
        //                 7 => UpperLeft,
        //                 8 => Neutral,
        //                 v => Err(InvalidSimpleHIDReport::InvalidStickDirection(v))?,
        //             };
        //
        //             Ok(button)
        //         }
        //     }
        //
        //     /// Pushed buttons and stick direction.
        //     #[derive(Debug, Clone)]
        //     pub struct SimpleHIDUpdate {
        //         pub input_report_id: u8,
        //         pub pushed_buttons: Vec<SimpleHIDButton>,
        //         pub stick_direction: StickDirection,
        //         pub filter_data: [u8; 8],
        //     }
        //
        //     impl TryFrom<[u8; 12]> for SimpleHIDUpdate {
        //         type Error = JoyConError;
        //
        //         fn try_from(value: [u8; 12]) -> Result<Self, Self::Error> {
        //             let input_report_id = value[0];
        //             let pushed_buttons = {
        //                 let byte_1 = value[1];
        //                 let byte_2 = value[2];
        //                 [].iter()
        //                     .chain(
        //                         BUTTON1.iter()
        //                             .enumerate()
        //                             .filter(|(idx, _)| {
        //                                 let idx = 2u8.pow(*idx as u32) as u8;
        //                                 byte_1 & idx == idx
        //                             })
        //                             .map(|(_, b)| b)
        //                     )
        //                     .chain(
        //                         BUTTON2.iter()
        //                             .enumerate()
        //                             .filter(|(idx, _)| {
        //                                 let idx = 2u8.pow(*idx as u32) as u8;
        //                                 byte_2 & idx == idx
        //                             })
        //                             .map(|(_, b)| b)
        //                     )
        //                     .cloned()
        //                     .collect()
        //             };
        //             let stick_direction = StickDirection::try_from(value[3])?;
        //             let filter_data = {
        //                 let mut buf = [0u8; 8];
        //                 buf.copy_from_slice(&value[4..12]);
        //                 buf
        //             };
        //
        //             Ok(SimpleHIDUpdate {
        //                 input_report_id,
        //                 pushed_buttons,
        //                 stick_direction,
        //                 filter_data,
        //             })
        //         }
        //     }
        //
        //     pub struct SimpleHIDMode<D: JoyConDriver> {
        //         driver: D,
        //     }
        //
        //     impl<D: JoyConDriver> Deref for SimpleHIDMode<D> {
        //         type Target = D;
        //
        //         fn deref(&self) -> &Self::Target {
        //             &self.driver
        //         }
        //     }
        //
        //     impl<D: JoyConDriver> DerefMut for SimpleHIDMode<D> {
        //         fn deref_mut(&mut self) -> &mut Self::Target {
        //             &mut self.driver
        //         }
        //     }
        //
        //     impl<D: JoyConDriver> InputReportMode<D> for SimpleHIDMode<D> {
        //         type Mode = SimpleHIDMode<D>;
        //         type Report = SimpleHIDUpdate;
        //
        //         fn set(driver: D) -> JoyConResult<Self::Mode> {
        //             let mut driver = driver;
        //             // set input report mode to simple hid mode
        //             driver.send_sub_command(SubCommand::SetInputReportMode, &[0x3F])?;
        //
        //             Ok(SimpleHIDMode { driver })
        //         }
        //
        //         fn read_input_report(&self) -> JoyConResult<Self::Report> {
        //             // read
        //             let mut buf = [0x00; 12];
        //             self.read(&mut buf)?;
        //             // convert
        //             Self::Report::try_from(buf)
        //         }
        //     }
        // }
    }

    pub mod lights {
        use super::*;

        /// LED to keep on lightning up / lightning
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
        pub enum LightUp {
            /// closest led to SL Button
            LED0 = 0x01,
            /// second closest led to SL Button
            LED1 = 0x02,
            /// second closest led to SR Button
            LED2 = 0x04,
            /// closest let to SR Button
            LED3 = 0x08,
        }

        /// LED to flash / flashing
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Flash {
            /// closest led to SL Button
            LED0 = 0x10,
            /// second closest led to SL Button
            LED1 = 0x20,
            /// second closest led to SR Button
            LED2 = 0x40,
            /// closest let to SR Button
            LED3 = 0x80,
        }

        pub trait Lights: JoyConDriver {
            const LIGHT_UP: [LightUp; 4] =
                [LightUp::LED0, LightUp::LED1, LightUp::LED2, LightUp::LED3];
            const FLASH: [Flash; 4] =
                [Flash::LED0, Flash::LED1, Flash::LED2, Flash::LED3];
            /// light up or flash LEDs on controller, vice versa.
            fn set_lights(&mut self, light_up: &Vec<LightUp>, flash: &Vec<Flash>) -> JoyConResult<usize> {
                let arg = light_up.iter()
                    .map(|&lu| lu as u8)
                    .sum::<u8>()
                    + flash.iter()
                    .map(|&f| f as u8)
                    .sum::<u8>();

                self.send_sub_command(SubCommand::SetPlayerLights, &[arg])
            }

            /// Get status of LEDs on controller
            fn get_lights(&mut self) -> JoyConResult<(Vec<LightUp>, Vec<Flash>)> {
                self.send_sub_command(SubCommand::GetPlayerLights, &[])?;

                // read reply
                let mut buf = [0; 16];
                // Joy-Con send standard input report with sub-command reply
                self.read(&mut buf)?;
                let value = buf[15];


                // parse reply
                let light_up = Self::LIGHT_UP.iter()
                    .filter(|&&l| {
                        let light = l as u8;
                        value & light == light
                    })
                    .cloned()
                    .collect();
                let flash = Self::FLASH.iter()
                    .filter(|&&f| {
                        let flash = f as u8;
                        value & flash == flash
                    })
                    .cloned()
                    .collect();

                Ok((light_up, flash))
            }
        }

        impl Lights for SimpleJoyConDriver {}
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