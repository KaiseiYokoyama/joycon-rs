use super::*;
use std::collections::HashSet;
pub use global_packet_number::GlobalPacketNumber;
pub use joycon_features::{JoyConFeature, IMUConfig};

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

pub mod joycon_features {
    /// Features on Joy-Cons which needs to set up.
    /// ex. IMU(6-Axis sensor), NFC/IR, Vibration
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum JoyConFeature {
        IMUFeature(IMUConfig),
        Vibration,
    }

    pub use imu_sensitivity::IMUConfig;

    pub mod imu_sensitivity {
        use std::hash::Hash;

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

        /// # Notice
        /// `IMUConfig` returns constant hash value.
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct IMUConfig {
            pub gyroscope_sensitivity: GyroscopeSensitivity,
            pub accelerometer_sensitivity: AccelerometerSensitivity,
            pub gyroscope_performance_rate: GyroscopePerformanceRate,
            pub accelerometer_anti_aliasing_filter_bandwidth: AccelerometerAntiAliasingFilterBandwidth,
        }

        impl Into<[u8; 4]> for IMUConfig {
            fn into(self) -> [u8; 4] {
                let IMUConfig {
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

        impl Hash for IMUConfig {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                // Returns constant value.
                0.hash(state);
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

/// The controller user uses to play with.
/// If you're not happy with this implementation, you can use `JoyConDriver` trait.
///
/// # Examples
/// ```
/// use joycon_rs::prelude::{JoyConManager, SimpleJoyConDriver, lights::*};
/// use joycon_rs::result::JoyConResult;
///
/// let manager = JoyConManager::new().unwrap();
/// let mut simple_joycon_drivers = manager.connected_joycon_devices.into_iter()
///     .flat_map(|joycon_device| SimpleJoyConDriver::new(joycon_device))
///     .collect::<Vec<SimpleJoyConDriver>>();
///
/// // set player's lights
/// simple_joycon_drivers.iter_mut()
///     .try_for_each::<_, JoyConResult<()>>(|driver| {
///         driver.set_player_lights(&vec![SimpleJoyConDriver::LIGHT_UP[0]], &vec![])?;
///         Ok(())
///     })
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct SimpleJoyConDriver {
    /// The controller user uses
    pub joycon: JoyConDevice,
    /// rotation of controller
    pub rotation: Rotation,
    enabled_features: HashSet<JoyConFeature>,
    /// Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
    global_packet_number: GlobalPacketNumber,
}

impl SimpleJoyConDriver {
    /// Constructs a new `SimpleJoyConDriver`.
    pub fn new(joycon: JoyConDevice) -> JoyConResult<Self> {
        // joycon.set_blocking_mode(true);
        // joycon.set_blocking_mode(false);

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
    /// Send command to Joy-Con
    fn write(&self, data: &[u8]) -> JoyConResult<usize>;

    /// Read reply from Joy-Con
    fn read(&self, buf: &mut [u8]) -> JoyConResult<usize>;

    /// Get global packet number
    fn global_packet_number(&self) -> u8;

    /// Increase global packet number. Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
    fn increase_global_packet_number(&mut self);

    /// Send command, sub-command, and data (sub-command's arguments) with u8 integers
    /// This returns ACK packet for the command or Error.
    fn send_command_raw(&mut self, command: u8, sub_command: u8, data: &[u8]) -> JoyConResult<[u8; 362]> {
        use input_report_mode::sub_command_mode::AckByte;

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

        // send command
        self.write(&buf)?;

        // check reply
        let mut buf = [0u8; 362];
        self.read(&mut buf)?;
        let ack_byte = AckByte::from(buf[13]);

        match ack_byte {
            AckByte::Ack { .. } => {
                Ok(buf)
            }
            AckByte::Nack => {
                Err(JoyConError::SubCommandError(sub_command))
            }
        }
    }

    /// Send sub-command, and data (sub-command's arguments) with u8 integers
    /// This returns ACK packet for the command or Error.
    fn send_sub_command_raw(&mut self, sub_command: u8, data: &[u8]) -> JoyConResult<[u8; 362]> {
        self.send_command_raw(1, sub_command, data)
    }

    /// Send command, sub-command, and data (sub-command's arguments) with `Command` and `SubCommand`
    /// This returns ACK packet for the command or Error.
    fn send_command(&mut self, command: Command, sub_command: SubCommand, data: &[u8]) -> JoyConResult<[u8; 362]> {
        let command = command as u8;
        let sub_command = sub_command as u8;

        self.send_command_raw(command, sub_command, data)
    }

    /// Send sub-command, and data (sub-command's arguments) with `Command` and `SubCommand`
    /// This returns ACK packet for the command or Error.
    fn send_sub_command(&mut self, sub_command: SubCommand, data: &[u8]) -> JoyConResult<[u8; 362]> {
        self.send_command(Command::RumbleAndSubCommand, sub_command, data)
    }

    /// Initialize Joy-Con's status
    fn reset(&mut self) -> JoyConResult<()> {
        // disable IMU (6-Axis sensor)
        self.send_sub_command(SubCommand::EnableIMU, &[0x00])?;
        // disable vibration
        self.send_sub_command(SubCommand::EnableVibration, &[0x00])?;

        Ok(())
    }

    /// Enable Joy-Con's feature. ex. IMU(6-Axis sensor), Vibration(Rumble)
    fn enable_feature(&mut self, feature: JoyConFeature) -> JoyConResult<()>;

    /// Get Enabled features.
    fn enabled_features(&self) -> &HashSet<JoyConFeature>;

    /// Get Joy-Con devices deal with.
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

    fn enable_feature(&mut self, feature: JoyConFeature) -> JoyConResult<()> {
        match feature {
            JoyConFeature::IMUFeature(feature) => {
                let data: [u8; 4] = feature.into();
                // enable IMU
                self.send_sub_command(SubCommand::EnableIMU, &[0x01])?;
                // set config
                self.send_sub_command(SubCommand::SetIMUSensitivity, &data)?;
            }
            JoyConFeature::Vibration => {
                // enable vibration
                self.send_sub_command(SubCommand::EnableVibration, &[0x01])?;
            }
        }

        self.enabled_features.insert(feature);

        Ok(())
    }

    fn enabled_features(&self) -> &HashSet<JoyConFeature> {
        &self.enabled_features
    }

    fn devices(&self) -> Vec<&JoyConDevice> {
        vec![&self.joycon]
    }
}

/// JoyCon's input report mode is divided into five main categories.
///
/// | Struct | Remarks | Frequency |
/// | :-- | :-- | :-- |
/// | [`SimpleHIDMode<D>`] | Simple HID mode. Reports pushed buttons, and stick directions (one of [8 directions]). | (Every time some button pressed) |
/// | [`StandardFullMode<D>`] | IMU(6-Axis sensor) data with standard input report | 60Hz |
/// | [`SubCommandMode<D, RD>`] | SubCommand's reply with standard input report | ? |
/// | (Unimplemented) | NFC/IR MCU FW update with standard input report  | |
/// | (Unimplemented) | NFC/IR data with standard input report | 60Hz |
///
/// Standard input report consists of input report ID, Timer, [battery level],
/// [connection info], [button status], [analog stick data], and vibrator input report.
///
/// If you want to implement input report mode, you can use [`InputReportMode<D>`].
///
/// [`SimpleHIDMode<D>`]: simple_hid_mode/struct.SimpleHIDMode.html
/// [8 directions]: simple_hid_mode/enum.StickDirection.html
/// [`StandardFullMode<D>`]: standard_full_mode/struct.StandardFullMode.html
/// [`SubCommandMode<D, RD>`]: sub_command_mode/struct.SubCommandMode.html
/// [battery level]: struct.Battery.html
/// [connection info]: struct.ConnectionInfo.html
/// [button status]: struct.PushedButtons.html
/// [analog stick data]: struct.AnalogStickData.html
/// [`InputReportMode<D>`]: trait.InputReportMode.html
pub mod input_report_mode {
    use super::*;
    pub use common::*;
    pub use self::{sub_command_mode::SubCommandMode, standard_full_mode::StandardFullMode, simple_hid_mode::SimpleHIDMode};
    use std::convert::TryFrom;
    use std::ops::{Deref, DerefMut};
    use std::hash::Hash;

    mod common {
        use super::*;
        use std::convert::TryFrom;

        /// Battery level
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
        pub enum BatteryLevel {
            Empty,
            Critical,
            Low,
            Medium,
            Full,
        }

        /// Battery info
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

        /// Device info
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
        pub enum Device {
            JoyCon,
            ProConOrChargingGrip,
        }

        /// Connection info
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

        /// Button status
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

        /// Analog stick data
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

        /// Common parts of the standard input report
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
        type Report: TryFrom<[u8; 362], Error=JoyConError>;
        type ArgsType: AsRef<[u8]>;

        const SUB_COMMAND: SubCommand;
        const ARGS: Self::ArgsType;

        /// Mode's setup operation.
        /// ex. Enable 6-Axis sensor
        /// note: There are no needs to change Joy-Con's input report mode, leave it to `InputReportMode<D>::new()`.
        fn setup(driver: D) -> JoyConResult<D>;

        /// Construct instance simply. Typically, your implementation will be ->
        /// ```ignore
        /// fn construct(driver: D) -> Self::Mode {
        ///     Self::Mode { driver }
        /// }
        /// ```
        fn construct(driver: D) -> Self::Mode;

        /// set Joy-Con's input report mode and return instance
        fn new(driver: D) -> JoyConResult<Self::Mode> {
            let mut driver = Self::Mode::setup(driver)?;

            driver.send_sub_command(Self::SUB_COMMAND, Self::ARGS.as_ref())?;

            Ok(Self::construct(driver))
        }

        /// read Joy-Con's input report
        fn read_input_report(&self) -> JoyConResult<Self::Report> {
            let mut buf = [0u8; 362];
            self.read(&mut buf)?;

            Self::Report::try_from(buf)
        }
    }

    /// Standard input report with extra report.
    pub struct StandardInputReport<EX: TryFrom<[u8; 349], Error=JoyConError>> {
        pub common: CommonReport,
        pub extra: EX,
    }

    impl<EX> TryFrom<[u8; 362]> for StandardInputReport<EX>
        where EX: TryFrom<[u8; 349], Error=JoyConError>
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

            Ok(StandardInputReport {
                common,
                extra,
            })
        }
    }

    impl<EX> Debug for StandardInputReport<EX>
        where EX: TryFrom<[u8; 349], Error=JoyConError> + Debug
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "StandardInputReport {{ common: {:?}, extra: {:?} }}",
                &self.common,
                &self.extra,
            )
        }
    }

    impl<EX> Clone for StandardInputReport<EX>
        where EX: TryFrom<[u8; 349], Error=JoyConError> + Clone
    {
        fn clone(&self) -> Self {
            let common = self.common.clone();
            let extra = self.extra.clone();

            StandardInputReport { common, extra }
        }
    }

    impl<EX> Hash for StandardInputReport<EX>
        where EX: TryFrom<[u8; 349], Error = JoyConError> + Hash
    {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.common.hash(state);
            self.extra.hash(state);
        }
    }

    impl<EX> PartialEq for StandardInputReport<EX>
        where EX: TryFrom<[u8; 349], Error=JoyConError> + PartialEq
    {
        fn eq(&self, other: &Self) -> bool {
            self.common.eq(&other.common)
                && self.extra.eq(&other.extra)
        }
    }

    impl<EX> Eq for StandardInputReport<EX>
        where EX: TryFrom<[u8; 349], Error=JoyConError> + Eq
    {}


    /// Receive standard input report with sub-command's reply.
    ///
    /// If you want to send sub-command and get reply, imply [`SubCommandReplyData`] for your struct.
    ///
    /// [`SubCommandReplyData`]: trait.SubCommandReplyData.html
    pub mod sub_command_mode {
        use super::*;
        use std::marker::PhantomData;

        /// Ack byte. If it is ACK, it contains data type.
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
        pub enum AckByte {
            Ack {
                data_type: u8
            },
            Nack,
        }

        impl From<u8> for AckByte {
            fn from(u: u8) -> Self {
                if u >> 7 == 1 {
                    AckByte::Ack {
                        data_type: u & 0x7F
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
        pub trait SubCommandReplyData: TryFrom<[u8; 35], Error=JoyConError> {
            type ArgsType: AsRef<[u8]>;
            const SUB_COMMAND: SubCommand;
            const ARGS: Self::ArgsType;

            /// The mode remains the same, sending commands and receiving replies.
            fn once<D>(driver: &mut D) -> JoyConResult<StandardInputReport<SubCommandReport<Self>>>
                where Self: std::marker::Sized,
                      D: JoyConDriver
            {
                let reply = driver.send_sub_command(Self::SUB_COMMAND, Self::ARGS.as_ref())?;
                StandardInputReport::try_from(reply)
            }
        }

        /// Replies to sub-commands
        #[derive(Clone)]
        pub struct SubCommandReport<RD>
            where RD: SubCommandReplyData
        {
            pub ack_byte: AckByte,
            pub sub_command_id: u8,
            pub reply: RD,
        }

        impl<RD> Debug for SubCommandReport<RD>
            where RD: SubCommandReplyData + Debug
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f,
                       "SubCommandReport {{\
                                \n\t ack_byte: {:?},
                                \n\t sub_command_id: {},
                                \n\t reply: {:?},
                       }}",
                       self.ack_byte,
                       self.sub_command_id,
                       &self.reply
                )
            }
        }

        impl<RD> TryFrom<[u8; 349]> for SubCommandReport<RD>
            where RD: SubCommandReplyData
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

        /// Receive standard input report with sub-command's reply.
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

        impl<D, RD> InputReportMode<D> for SubCommandMode<D, RD>
            where D: JoyConDriver,
                  RD: SubCommandReplyData
        {
            type Mode = SubCommandMode<D, RD>;
            type Report = StandardInputReport<SubCommandReport<RD>>;
            type ArgsType = RD::ArgsType;
            const SUB_COMMAND: SubCommand = RD::SUB_COMMAND;
            const ARGS: Self::ArgsType = RD::ARGS;

            fn setup(driver: D) -> JoyConResult<D> {
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

    /// Receive standard full report (standard input report with IMU(6-Axis sensor) data).
    ///
    /// Pushes current state at 60Hz.
    pub mod standard_full_mode {
        use super::*;

        /// IMU(6-Axis sensor)'s value.
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
        ///
        /// # Example
        /// ```no_run
        /// use joycon_rs::prelude::*;
        ///
        /// let (sender, receiver) = std::sync::mpsc::channel();
        ///
        /// JoyConManager::new()
        ///     .unwrap()
        ///     .connected_joycon_devices
        ///     .into_iter()
        ///     .flat_map(|j| SimpleJoyConDriver::new(j))
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
        ///
        /// // Receive all Joy-Con's standard full reports
        /// while let Ok(standard_full_report) = receiver.recv() {
        ///     // Output reports
        ///     dbg!(standard_full_report);
        /// }
        /// ```
        pub struct StandardFullMode<D: JoyConDriver> {
            driver: D,
        }

        impl<D> Deref for StandardFullMode<D>
            where D: JoyConDriver
        {
            type Target = D;

            fn deref(&self) -> &Self::Target {
                &self.driver
            }
        }

        impl<D> DerefMut for StandardFullMode<D>
            where D: JoyConDriver
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.driver
            }
        }

        impl<D> InputReportMode<D> for StandardFullMode<D>
            where D: JoyConDriver
        {
            type Mode = StandardFullMode<D>;
            type Report = StandardInputReport<IMUData>;
            type ArgsType = [u8; 1];
            const SUB_COMMAND: SubCommand = SubCommand::SetInputReportMode;
            const ARGS: Self::ArgsType = [0x30];

            fn setup(driver: D) -> JoyConResult<D> {
                let mut driver = driver;
                // enable IMU(6-Axis sensor)
                let imf_enabled = driver.enabled_features()
                    .iter()
                    .any(|jf| match jf {
                        JoyConFeature::IMUFeature(_) => true,
                        _ => false,
                    });
                if !imf_enabled {
                    driver.enable_feature(JoyConFeature::IMUFeature(IMUConfig::default()))?;
                }

                Ok(driver)
            }

            fn construct(driver: D) -> Self::Mode {
                Self::Mode {
                    driver
                }
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
            [Down, Right, Left, Up, SL, SR, ]
        };
        const BUTTON2: [SimpleHIDButton; 8] = {
            use SimpleHIDButton::*;
            [Minus, Plus, LeftStick, RightStick, Home, Capture, L_R, ZL_ZR, ]
        };

        /// Hold your controller sideways so that SL, SYNC, and SR line up with the screen. Pushing the stick towards a direction in this table will cause that value to be sent.
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
                    v => Err(InvalidSimpleHIDReport::InvalidStickDirection(v))?,
                };

                Ok(button)
            }
        }

        /// Pushed buttons and stick direction.
        #[derive(Debug, Clone)]
        pub struct SimpleHIDReport {
            pub input_report_id: u8,
            pub pushed_buttons: Vec<SimpleHIDButton>,
            pub stick_direction: StickDirection,
            pub filter_data: [u8; 8],
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
                            BUTTON1.iter()
                                .enumerate()
                                .filter(|(idx, _)| {
                                    let idx = 2u8.pow(*idx as u32) as u8;
                                    byte_1 & idx == idx
                                })
                                .map(|(_, b)| b)
                        )
                        .chain(
                            BUTTON2.iter()
                                .enumerate()
                                .filter(|(idx, _)| {
                                    let idx = 2u8.pow(*idx as u32) as u8;
                                    byte_2 & idx == idx
                                })
                                .map(|(_, b)| b)
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
                    filter_data,
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
        /// JoyConManager::new()
        ///     .unwrap()
        ///     .connected_joycon_devices
        ///     .into_iter()
        ///     .flat_map(|d| SimpleJoyConDriver::new(d))
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
        ///         Ok(())
        ///     })
        ///     .unwrap();
        ///
        /// // Receive all Joy-Con's simple HID reports
        /// while let Ok(simple_hid_report) = receiver.recv() {
        ///     // Output reports
        ///     dbg!(simple_hid_report);
        /// }
        /// ```
        pub struct SimpleHIDMode<D: JoyConDriver> {
            driver: D,
        }

        impl<D: JoyConDriver> Deref for SimpleHIDMode<D> {
            type Target = D;

            fn deref(&self) -> &Self::Target {
                &self.driver
            }
        }

        impl<D: JoyConDriver> DerefMut for SimpleHIDMode<D> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.driver
            }
        }

        impl<D> InputReportMode<D> for SimpleHIDMode<D>
            where D: JoyConDriver
        {
            type Mode = SimpleHIDMode<D>;
            type Report = SimpleHIDReport;
            type ArgsType = [u8; 1];
            const SUB_COMMAND: SubCommand = SubCommand::SetInputReportMode;
            const ARGS: Self::ArgsType = [0x3F];

            fn setup(driver: D) -> JoyConResult<D> {
                // do nothing
                Ok(driver)
            }

            fn construct(driver: D) -> Self::Mode {
                Self::Mode {
                    driver
                }
            }
        }
    }
}

/// Operate Joy-Con's player lights (LEDs). The gist of this module is [`Lights`].
///
/// [`Lights`]: trait.Lights.html
///
/// # Usage
/// ```no_run
/// use joycon_rs::prelude::{*, lights::*};
///
/// let mut joycon_driver = JoyConManager::new()
///     .unwrap()
///     .connected_joycon_devices
///     .into_iter()
///     .flat_map(|d| SimpleJoyConDriver::new(d))
///     .next().unwrap();
///
/// // Set player lights lightning and flashing.
/// joycon_driver.set_player_lights(&vec![LightUp::LED2], &vec![Flash::LED3]).unwrap();
///
/// // Get status of player lights
/// let player_lights_status = joycon_driver.get_player_lights()
///     .unwrap()
///     .extra;
/// dbg!(player_lights_status);
/// ```
pub mod lights {
    use super::{*, input_report_mode::sub_command_mode::*};
    use std::convert::TryFrom;
    use crate::joycon::driver::input_report_mode::StandardInputReport;

    /// LED to keep on lightning up / lightning
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub enum LightUp {
        /// Closest led to SL Button
        LED0 = 0x01,
        /// Second closest led to SL Button
        LED1 = 0x02,
        /// Second closest led to SR Button
        LED2 = 0x04,
        /// Closest let to SR Button
        LED3 = 0x08,
    }

    /// LED to flash / flashing
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Flash {
        /// Closest led to SL Button
        LED0 = 0x10,
        /// Second closest led to SL Button
        LED1 = 0x20,
        /// Second closest led to SR Button
        LED2 = 0x40,
        /// Closest let to SR Button
        LED3 = 0x80,
    }

    /// Status of player lights.
    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct LightsStatus {
        pub light_up: Vec<LightUp>,
        pub flash: Vec<Flash>,
    }

    const LIGHT_UP: [LightUp; 4] =
        [LightUp::LED0, LightUp::LED1, LightUp::LED2, LightUp::LED3];
    const FLASH: [Flash; 4] =
        [Flash::LED0, Flash::LED1, Flash::LED2, Flash::LED3];

    impl TryFrom<[u8; 35]> for LightsStatus {
        type Error = JoyConError;

        fn try_from(value: [u8; 35]) -> Result<Self, Self::Error> {
            let value = value[0];

            // parse reply
            let light_up = LIGHT_UP.iter()
                .filter(|&&l| {
                    let light = l as u8;
                    value & light == light
                })
                .cloned()
                .collect();
            let flash = FLASH.iter()
                .filter(|&&f| {
                    let flash = f as u8;
                    value & flash == flash
                })
                .cloned()
                .collect();

            Ok(LightsStatus { light_up, flash })
        }
    }

    impl SubCommandReplyData for LightsStatus {
        type ArgsType = [u8; 0];
        const SUB_COMMAND: SubCommand = SubCommand::GetPlayerLights;
        const ARGS: Self::ArgsType = [];
    }

    /// Operations of player lights.
    pub trait Lights: JoyConDriver {
        const LIGHT_UP: [LightUp; 4] = LIGHT_UP;
        const FLASH: [Flash; 4] = FLASH;

        /// Light up or flash LEDs on controller, vice versa.
        ///
        /// # Example
        /// If you run this code,
        ///
        /// ```no_run
        /// use joycon_rs::prelude::{*, lights::*};
        ///
        /// // some code omitted
        ///
        /// # let mut joycon_driver = JoyConManager::new()
        /// #     .unwrap()
        /// #     .connected_joycon_devices
        /// #     .into_iter()
        /// #     .flat_map(|d| SimpleJoyConDriver::new(d))
        /// #     .next().unwrap();
        /// joycon_driver.set_player_lights(&vec![LightUp::LED0],&vec![]).unwrap();
        /// ```
        ///
        /// player lights will be...
        /// > [SL Button] 💡🤔🤔🤔 [SR Button]
        ///
        ///
        /// For another example,
        /// ```no_run
        /// # use joycon_rs::prelude::{*, lights::*};
        /// #
        /// # let mut joycon_driver = JoyConManager::new()
        /// #     .unwrap()
        /// #     .connected_joycon_devices
        /// #     .into_iter()
        /// #     .flat_map(|d| SimpleJoyConDriver::new(d))
        /// #     .next().unwrap();
        /// joycon_driver.set_player_lights(&vec![LightUp::LED2], &vec![Flash::LED3]).unwrap();
        /// ```
        ///
        /// player lights will be...
        /// > [SL Button] 🤔🤔💡📸 [SR Button]
        ///
        /// ## Duplication
        ///
        /// If a command to a certain LED is duplicated, the lighting command takes precedence.
        ///
        /// ```no_run
        /// # use joycon_rs::prelude::{*, lights::*};
        /// #
        /// # let mut joycon_driver = JoyConManager::new()
        /// #     .unwrap()
        /// #     .connected_joycon_devices
        /// #     .into_iter()
        /// #     .flat_map(|d| SimpleJoyConDriver::new(d))
        /// #     .next().unwrap();
        /// joycon_driver.set_player_lights(&vec![LightUp::LED1], &vec![Flash::LED1]).unwrap();
        /// ```
        ///
        /// Player lights will be...
        /// > [SL Button] 🤔💡🤔🤔 [SR Button]
        ///
        fn set_player_lights(&mut self, light_up: &Vec<LightUp>, flash: &Vec<Flash>) -> JoyConResult<[u8; 362]> {
            let arg = light_up.iter()
                .map(|&lu| lu as u8)
                .sum::<u8>()
                + flash.iter()
                .map(|&f| f as u8)
                .sum::<u8>();

            let reply = self.send_sub_command(SubCommand::SetPlayerLights, &[arg])?;
            let ack_byte = reply[13];

            match AckByte::from(ack_byte) {
                AckByte::Ack { .. } => {
                    Ok(reply)
                }
                AckByte::Nack => {
                    Err(JoyConError::SubCommandError(SubCommand::SetPlayerLights as u8))
                }
            }
        }

        /// Get status of player lights on controller.
        ///
        /// # Example
        ///
        /// ```no_run
        /// use joycon_rs::prelude::{*, lights::*};
        ///
        /// # let mut joycon_driver = JoyConManager::new()
        /// #     .unwrap()
        /// #     .connected_joycon_devices
        /// #     .into_iter()
        /// #     .flat_map(|d| SimpleJoyConDriver::new(d))
        /// #     .next().unwrap();
        /// let player_lights_status = joycon_driver.get_player_lights()
        ///     .unwrap()
        ///     .extra;
        /// dbg!(player_lights_status);
        /// ```
        ///
        fn get_player_lights(&mut self) -> JoyConResult<StandardInputReport<SubCommandReport<LightsStatus>>>
            where Self: std::marker::Sized
        {
            LightsStatus::once(self)
        }
    }

    impl<D> Lights for D where D: JoyConDriver {}
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
