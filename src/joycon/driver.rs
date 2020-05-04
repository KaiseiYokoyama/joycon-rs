use super::*;
use std::convert::TryFrom;
use std::collections::HashSet;
pub use global_packet_number::GlobalPacketNumber;
pub use joycon_features::{JoyConFeature, IMUConfig};
use std::sync::{Mutex, MutexGuard};

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

/// Rumble data for vibration.
///
/// # Notice
/// Constraints exist.
/// * frequency - 0.0 < freq < 1252.0
/// * amplitude - 0.0 < amp < 1.799.0
///
/// # Example
/// ```no_run
/// use joycon_rs::prelude::{*, joycon_features::JoyConFeature};
///
/// let manager = JoyConManager::new().unwrap();
/// let (managed_devices, new_devices) = {
///     let lock = manager.lock();
///     match lock {
///         Ok(manager) =>
///             (manager.managed_devices(), manager.new_devices()),
///         Err(_) => unreachable!(),
///     }
/// };
///
/// managed_devices.into_iter()
///     .chain(new_devices)
///     .try_for_each::<_, JoyConResult<()>>(|d| {
///         let mut driver = SimpleJoyConDriver::new(&d)?;
///
///         driver.enable_feature(JoyConFeature::Vibration)?;
///
///         let rumble = Rumble::new(300.0,0.9);
///         // ₍₍(ง˘ω˘)ว⁾⁾ Rumble! ₍₍(ง˘ω˘)ว⁾⁾
///         driver.rumble((Some(rumble), Some(rumble)))?;
///
///         Ok(())
///     })
///     .unwrap();
///```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rumble {
    frequency: f32,
    amplitude: f32,
}

impl Rumble {
    pub fn frequency(&self) -> f32 {
        self.frequency
    }

    pub fn amplitude(&self) -> f32 {
        self.amplitude
    }

    /// Constructor of Rumble.
    /// If arguments not in line with constraints, args will be saturated.
    pub fn new(freq: f32, amp: f32) -> Self {
        let freq = if freq < 0.0 {
            0.0
        } else if freq > 1252.0 {
            1252.0
        } else {
            freq
        };

        let amp = if amp < 0.0 {
            0.0
        } else if amp > 1.799 {
            1.799
        } else {
            amp
        };

        Self {
            frequency: freq,
            amplitude: amp,
        }
    }

    /// The amplitudes over 1.003 are not safe for the integrity of the linear resonant actuators.
    pub fn is_safe(&self) -> bool {
        self.amplitude < 1.003
    }

    /// Generates stopper of rumbling.
    ///
    /// # Example
    /// ```ignore
    /// # use joycon_rs::prelude::*;
    /// # let mut rumbling_controller_driver: SimpleJoyConDriver;
    /// // Make JoyCon stop rambling.
    /// rumbling_controller_driver.rumble((Some(Rumble::stop()),Some(Rumble::stop()))).unwrap();
    /// ```
    pub fn stop() -> Self {
        Self {
            frequency: 0.0,
            amplitude: 0.0,
        }
    }
}

impl Into<[u8; 4]> for Rumble {
    fn into(self) -> [u8; 4] {
        let encoded_hex_freq = f32::round(f32::log2(self.frequency / 10.0) * 32.0) as u8;

        let hf_freq: u16 = (encoded_hex_freq as u16).saturating_sub(0x60) * 4;
        let lf_freq: u8 = encoded_hex_freq.saturating_sub(0x40);

        let encoded_hex_amp = if self.amplitude > 0.23 {
            f32::round(f32::log2(self.amplitude * 8.7) * 32.0) as u8
        } else if self.amplitude > 0.12 {
            f32::round(f32::log2(self.amplitude * 17.0) * 16.0) as u8
        } else {
            // todo study
            f32::round(f32::log2(self.amplitude * 17.0) * 16.0) as u8
        };

        let hf_amp: u16 = {
            let hf_amp: u16 = (encoded_hex_freq as u16 - 0x60) * 4;
            if hf_amp > 0x01FC {
                0x01FC
            } else { hf_amp }
        }; // encoded_hex_amp<<1;
        let lf_amp: u8 = {
            let lf_amp = encoded_hex_amp / 2 + 64;
            if lf_amp > 0x7F {
                0x7F
            } else { lf_amp }
        };      // (encoded_hex_amp>>1)+0x40;

        let mut buf = [0u8; 4];

        // HF: Byte swapping
        buf[0] = (hf_freq & 0xFF) as u8;
        // buf[1] = (hf_amp + ((hf_freq >> 8) & 0xFF)) as u8; //Add amp + 1st byte of frequency to amplitude byte
        buf[1] = (hf_amp + (hf_freq.wrapping_shr(8) & 0xFF)) as u8; //Add amp + 1st byte of frequency to amplitude byte

        // LF: Byte swapping
        buf[2] = lf_freq.saturating_add(lf_amp.wrapping_shr(8) & 0xFF);
        buf[3] = lf_amp & 0xFF;

        buf
    }
}

pub mod calibration {
    use std::fmt::Debug;
    use std::hash::Hash;

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct AxisCalibration {
        max: u16,
        center: u16,
        min: u16,
    }

    impl AxisCalibration {
        /// Max above center
        pub fn max(&self) -> u16 {
            self.max
        }

        /// Center
        pub fn center(&self) -> u16 {
            self.center
        }

        /// Min above center
        pub fn min(&self) -> u16 {
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
    pub struct AnalogStickCalibrations {
        left: Option<StickCalibration>,
        right: Option<StickCalibration>,
    }

    impl From<[u8; 18]> for AnalogStickCalibrations {
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
                    .all(|u| u == &0xF) {
                    None
                } else {
                    let left_stick_data = stick_cal_to_data(&stick_cal[0..9]);

                    let left_stick_calibration =
                        StickCalibration {
                            x: AxisCalibration {
                                center: left_stick_data[2],
                                max: left_stick_data[2] + left_stick_data[0],
                                min: left_stick_data[2] - left_stick_data[4],
                            },
                            y: AxisCalibration {
                                center: left_stick_data[3],
                                max: left_stick_data[3] + left_stick_data[1],
                                min: left_stick_data[3] - left_stick_data[5],
                            },
                        };

                    Some(left_stick_calibration)
                }
            };

            let right_stick_calibration = {
                let right_stick_cal = &stick_cal[9..18];
                if right_stick_cal.iter()
                    .all(|u| u == &0xF) {
                    None
                } else {
                    let right_stick_data = stick_cal_to_data(right_stick_cal);

                    let right_stick_calibration =
                        StickCalibration {
                            x: AxisCalibration {
                                center: right_stick_data[0],
                                max: right_stick_data[0] + right_stick_data[4],
                                min: right_stick_data[0] - right_stick_data[2],
                            },
                            y: AxisCalibration {
                                center: right_stick_data[1],
                                max: right_stick_data[1] + right_stick_data[5],
                                min: right_stick_data[1] - right_stick_data[3],
                            },
                        };

                    Some(right_stick_calibration)
                }
            };

            AnalogStickCalibrations {
                left: left_stick_calibration,
                right: right_stick_calibration,
            }
        }
    }

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
    pub struct IMUCalibration {
        acc_origin_position: XYZ<i16>,
        acc_sensitivity_special_coeff: XYZ<i16>,
        gyro_origin_position: XYZ<i16>,
        gyro_sensitivity_special_coeff: XYZ<i16>,
    }

    impl IMUCalibration {
        /// Acc XYZ origin position when completely horizontal and stick is upside
        pub fn acc_origin_position(&self) -> &XYZ<i16> {
            &self.acc_origin_position
        }

        /// Acc XYZ sensitivity special coeff, for default sensitivity: ±8G.
        pub fn acc_sensitivity_special_coeff(&self) -> &XYZ<i16> {
            &self.acc_sensitivity_special_coeff
        }

        /// Gyro XYZ origin position when still
        pub fn gyro_origin_position(&self) -> &XYZ<i16> {
            &self.gyro_origin_position
        }

        /// Gyro XYZ sensitivity special coeff, for default sensitivity: ±2000dps.
        pub fn gyro_sensitivity_special_coeff(&self) -> &XYZ<i16> {
            &self.gyro_sensitivity_special_coeff
        }
    }

    impl From<[u8; 24]> for IMUCalibration {
        fn from(value: [u8; 24]) -> Self {
            use std::slice::Iter;
            use std::iter::Cloned;

            fn convert(big: u8, little: u8) -> i16 {
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

            Self {
                acc_origin_position,
                acc_sensitivity_special_coeff,
                gyro_origin_position,
                gyro_sensitivity_special_coeff,
            }
        }
    }
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
/// ```no_run
/// use joycon_rs::prelude::{JoyConManager, SimpleJoyConDriver, lights::*};
/// use joycon_rs::result::JoyConResult;
///
/// let manager = JoyConManager::new().unwrap();
///
/// let (managed_devices, new_devices) = {
///     let lock = manager.lock();
///     match lock {
///         Ok(manager) => (manager.managed_devices(), manager.new_devices()),
///         Err(_) => return,
///     }
/// };
///
/// managed_devices.into_iter()
///     .chain(new_devices)
///     .try_for_each::<_,JoyConResult<()>>(|device| {
///         let mut driver = SimpleJoyConDriver::new(&device)?;
///
///         // set player's lights
///         driver.set_player_lights(&vec![SimpleJoyConDriver::LIGHT_UP[0]], &vec![])?;
///
///         Ok(())
///     })
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct SimpleJoyConDriver {
    /// The controller user uses
    pub joycon: Arc<Mutex<JoyConDevice>>,
    /// rotation of controller
    pub rotation: Rotation,
    rumble: (Option<Rumble>, Option<Rumble>),
    enabled_features: HashSet<JoyConFeature>,
    /// Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
    global_packet_number: GlobalPacketNumber,
}

impl SimpleJoyConDriver {
    /// Constructs a new `SimpleJoyConDriver`.
    pub fn new(joycon: &Arc<Mutex<JoyConDevice>>) -> JoyConResult<Self> {
        // joycon.set_blocking_mode(true);
        // joycon.set_blocking_mode(false);

        let mut driver = Self {
            joycon: Arc::clone(joycon),
            rotation: Rotation::Portrait,
            rumble: (None, None),
            enabled_features: HashSet::new(),
            global_packet_number: GlobalPacketNumber::default(),
        };

        // driver.reset()?;

        Ok(driver)
    }

    pub fn joycon(&self) -> MutexGuard<JoyConDevice> {
        // todo error handling
        match self.joycon.lock() {
            Ok(joycon) => joycon,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

pub trait JoyConDriver {
    /// If a sub-command is sent and no ACK packet is returned, tread again for the number of times of this value.
    const ACK_TRY: usize = 5;

    /// Send command to Joy-Con
    fn write(&self, data: &[u8]) -> JoyConResult<usize>;

    /// Read reply from Joy-Con
    fn read(&self, buf: &mut [u8]) -> JoyConResult<usize>;

    /// Send feature report to Joy-Con
    fn send_feature_report(&self, data: &[u8]) -> JoyConResult<()>;

    /// Read feature report from Joy-Con
    fn get_feature_report(&self, buf: &mut [u8]) -> JoyConResult<usize>;

    /// Get global packet number
    fn global_packet_number(&self) -> u8;

    /// Increase global packet number. Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
    fn increase_global_packet_number(&mut self);

    /// Set rumble status.
    fn set_rumble_status(&mut self, rumble_l_r: (Option<Rumble>, Option<Rumble>));

    /// Set rumble status and send rumble command to JoyCon.
    fn rumble(&mut self, rumble_l_r: (Option<Rumble>, Option<Rumble>)) -> JoyConResult<usize> {
        self.set_rumble_status(rumble_l_r);
        self.send_command_raw(Command::Rumble as u8, 0, &[])
    }

    /// Get rumble status.
    fn get_rumble_status(&self) -> (Option<Rumble>, Option<Rumble>);

    /// Send command, sub-command, and data (sub-command's arguments) with u8 integers
    /// This returns ACK packet for the command or Error.
    fn send_command_raw(&mut self, command: u8, sub_command: u8, data: &[u8]) -> JoyConResult<usize> {
        let mut buf = [0x0; 0x40];
        // set command
        buf[0] = command;
        // set packet number
        buf[1] = self.global_packet_number();
        // increase packet number
        self.increase_global_packet_number();

        // rumble
        let (rumble_l, rumble_r) = self.get_rumble_status();
        if let Some(rumble_l) = rumble_l {
            let rumble_left: [u8; 4] = rumble_l.into();
            buf[2..6].copy_from_slice(&rumble_left);
        }
        if let Some(rumble_r) = rumble_r {
            let rumble_right: [u8; 4] = rumble_r.into();
            buf[6..10].copy_from_slice(&rumble_right);
        }

        // set sub command
        buf[10] = sub_command;
        // set data
        buf[11..11 + data.len()].copy_from_slice(data);

        // send command
        self.write(&buf)
    }

    /// Send sub-command, and data (sub-command's arguments) with u8 integers
    /// This returns ACK packet for the command or Error.
    fn send_sub_command_raw(&mut self, sub_command: u8, data: &[u8]) -> JoyConResult<[u8; 362]> {
        use input_report_mode::sub_command_mode::AckByte;

        self.send_command_raw(1, sub_command, data)?;

        // check reply
        std::iter::repeat(())
            .take(Self::ACK_TRY)
            .flat_map(|()| {
                let mut buf = [0u8; 362];
                self.read(&mut buf).ok()?;
                let ack_byte = AckByte::from(buf[13]);

                match ack_byte {
                    AckByte::Ack { .. } => Some(buf),
                    AckByte::Nack => None
                }
            })
            .next()
            .ok_or(JoyConError::SubCommandError(sub_command, Vec::new()))
    }

    /// Send command, sub-command, and data (sub-command's arguments) with `Command` and `SubCommand`
    /// This returns ACK packet for the command or Error.
    fn send_command(&mut self, command: Command, sub_command: SubCommand, data: &[u8]) -> JoyConResult<usize> {
        let command = command as u8;
        let sub_command = sub_command as u8;

        self.send_command_raw(command, sub_command, data)
    }

    /// Send sub-command, and data (sub-command's arguments) with `Command` and `SubCommand`
    /// This returns ACK packet for the command or Error.
    fn send_sub_command(&mut self, sub_command: SubCommand, data: &[u8]) -> JoyConResult<[u8; 362]> {
        self.send_sub_command_raw(sub_command as u8, data)
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
    fn devices(&self) -> Vec<Arc<Mutex<JoyConDevice>>>;
}

impl JoyConDriver for SimpleJoyConDriver {
    fn write(&self, data: &[u8]) -> JoyConResult<usize> {
        let joycon = self.joycon();
        Ok(joycon.write(data)?)
    }

    fn read(&self, buf: &mut [u8]) -> JoyConResult<usize> {
        Ok(self.joycon().read(buf)?)
    }

    fn send_feature_report(&self, data: &[u8]) -> JoyConResult<()> {
        Ok(self.joycon().send_feature_report(data)?)
    }

    fn get_feature_report(&self, buf: &mut [u8]) -> JoyConResult<usize> {
        Ok(self.joycon().get_feature_report(buf)?)
    }

    fn global_packet_number(&self) -> u8 {
        self.global_packet_number.into()
    }

    fn increase_global_packet_number(&mut self) {
        self.global_packet_number = self.global_packet_number.next();
    }

    fn set_rumble_status(&mut self, rumble_l_r: (Option<Rumble>, Option<Rumble>)) {
        self.rumble = rumble_l_r;
    }

    fn get_rumble_status(&self) -> (Option<Rumble>, Option<Rumble>) {
        self.rumble
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

    fn devices(&self) -> Vec<Arc<Mutex<JoyConDevice>>> {
        vec![Arc::clone(&self.joycon)]
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

    pub trait InputReportMode<D: JoyConDriver>: Sized {
        type Report: TryFrom<[u8; 362], Error=JoyConError>;
        type ArgsType: AsRef<[u8]>;

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

        /// Refference of driver.
        fn driver(&self) -> &D;

        /// Mutable refference of driver.
        fn driver_mut(&mut self) -> &mut D;

        /// Unwrap.
        fn into_driver(self) -> D;
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
        where EX: TryFrom<[u8; 349], Error=JoyConError> + Hash
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

        impl<D, RD> InputReportMode<D> for SubCommandMode<D, RD>
            where D: JoyConDriver,
                  RD: SubCommandReplyData
        {
            type Report = StandardInputReport<SubCommandReport<RD>>;
            type ArgsType = RD::ArgsType;
            const SUB_COMMAND: SubCommand = RD::SUB_COMMAND;
            const ARGS: Self::ArgsType = RD::ARGS;

            fn new(driver: D) -> JoyConResult<Self> {
                let mut driver = driver;
                driver.send_sub_command(Self::SUB_COMMAND, Self::ARGS.as_ref())?;

                Ok(SubCommandMode {
                    driver,
                    _phantom: PhantomData,
                })
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

    /// Receive standard full report (standard input report with IMU(6-Axis sensor) data).
    ///
    /// Pushes current state at 60Hz (ProCon: 120Hz).
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
        /// let _output = std::thread::spawn( move || {
        ///     while let Ok(update) = receiver.recv() {
        ///         dbg!(update);
        ///     }
        /// });
        ///
        /// let manager = JoyConManager::new().unwrap();
        ///
        /// let (managed_devices, new_devices) = {
        ///     let lock = manager.lock();
        ///     match lock {
        ///         Ok(manager) => (manager.managed_devices(), manager.new_devices()),
        ///         Err(_) => return,
        ///     }
        /// };
        ///
        /// managed_devices.into_iter()
        ///     .chain(new_devices)
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
            where D: JoyConDriver
        {
            type Report = StandardInputReport<IMUData>;
            type ArgsType = [u8; 1];
            const SUB_COMMAND: SubCommand = SubCommand::SetInputReportMode;
            const ARGS: Self::ArgsType = [0x30];

            fn new(driver: D) -> JoyConResult<Self> {
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

                driver.send_sub_command(Self::SUB_COMMAND, Self::ARGS.as_ref())?;

                Ok(StandardFullMode {
                    driver
                })
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
        /// // Receive all Joy-Con's simple HID reports
        /// while let Ok(simple_hid_report) = receiver.recv() {
        ///     // Output reports
        ///     dbg!(simple_hid_report);
        /// }
        ///
        /// let manager = JoyConManager::new().unwrap();
        ///
        /// let (managed_devices, new_devices) = {
        ///     let lock = manager.lock();
        ///     match lock {
        ///         Ok(manager) => (manager.managed_devices(), manager.new_devices()),
        ///         Err(_) => return,
        ///     }
        /// };
        ///
        /// managed_devices.into_iter()
        ///     .chain(new_devices)
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
            where D: JoyConDriver
        {
            type Report = SimpleHIDReport;
            type ArgsType = [u8; 1];
            const SUB_COMMAND: SubCommand = SubCommand::SetInputReportMode;
            const ARGS: Self::ArgsType = [0x3F];

            fn new(driver: D) -> JoyConResult<Self> {
                let mut driver = driver;
                driver.send_sub_command(Self::SUB_COMMAND, Self::ARGS.as_ref())?;

                Ok(SimpleHIDMode {
                    driver
                })
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
}

/// Operate Joy-Con's player lights (LEDs). The gist of this module is [`Lights`].
///
/// [`Lights`]: trait.Lights.html
///
/// # Usage
/// ```no_run
/// use joycon_rs::prelude::{*, lights::*};
///
/// let manager = JoyConManager::new().unwrap();
///
/// let device = manager.lock()
///                     .unwrap()
///                     .managed_devices()
///                     .remove(0);
///
/// let mut joycon_driver = SimpleJoyConDriver::new(&device).unwrap();
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

    pub mod home_button {
        use super::*;

        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
        pub struct u4(u8);

        impl u4 {
            const MAX: Self = u4(15);
        }

        impl From<u8> for u4 {
            fn from(v: u8) -> Self {
                let value = u4(v);

                if value > Self::MAX {
                    Self::MAX
                } else { value }
            }
        }

        impl Into<u8> for u4 {
            fn into(self) -> u8 {
                self.0
            }
        }

        /// Element of HOME light emitting pattern.
        /// The LED Duration Multiplier and the Fading Multiplier use the same algorithm:
        /// Global Mini Cycle Duration ms * Multiplier value.
        ///
        /// Example: GMCD is set to xF = 175ms and LED Duration Multiplier is set to x4.
        /// The Duration that the LED will stay on it's configured intensity is then 175 * 4 = 700ms.
        #[derive(Clone, Debug, Hash, Eq, PartialEq)]
        pub struct LightEmittingPhase {
            /// LED intensity. x0 -> 0%, xF -> 100%
            pub led_intensity: u4,
            /// Fading Transition Duration. Value is a Multiplier of Global Mini Cycle Duration.
            pub fading_transition_duration: u4,
            /// LED Duration Multiplier
            pub led_duration: u4,
        }

        /// HOME light emitting pattern.
        #[derive(Clone, Debug, Hash, Eq, PartialEq)]
        pub struct LightEmittingPattern {
            phases_len: Option<u4>,
            phases: Vec<LightEmittingPhase>,
            /// Global Mini Cycle Duration. 8ms - 175ms. Value x0 = 0ms/OFF
            global_mini_cycle_duration: u4,
            /// LED Start Intensity. Value x0=0% - xF=100%
            led_start_intensity: u4,
            repeat_count: u4,
        }

        impl LightEmittingPattern {
            /// Constructor of `LightEmittingPattern`.
            ///
            /// * global_mini_cycle_duration (*ms*) - 0 <= global_mini_cycle_duration <= 175
            /// * led_start_intensity (*%*) - 0 <= led_start_intensity <= 100
            /// * repeat_count - 0 <= repeat_count <= 15: Value `0` is repeat forever.
            pub fn new(global_mini_cycle_duration: u8, led_start_intensity: u8, repeat_count: u4) -> Self {
                let global_mini_cycle_duration = if global_mini_cycle_duration == 0 {
                    0.into()
                } else {
                    ((global_mini_cycle_duration - 7) / 12 + 1).into()
                };

                let led_start_intensity = {
                    let saturated = if 100 < led_start_intensity { 100 } else { led_start_intensity } as f32;
                    ((saturated / 6.25) as u8).into()
                };

                LightEmittingPattern {
                    phases_len: None,
                    phases: Vec::with_capacity(15),
                    global_mini_cycle_duration,
                    led_start_intensity,
                    repeat_count,
                }
            }

            pub fn push_phase(&mut self, phase: LightEmittingPhase) {
                self.phases.push(phase);
            }

            /// Add emitting phase to pattern.
            ///
            /// * led_intensity (*%*) - 0 <= led_intensity <= 100
            /// * fading_transition_duration (*ms*) - 0 < fading_transition_duration < self.global_mini_cycle_duration (ms) * 15
            /// * led_duration (*ms*) - 0 < fading_transition_duration < self.global_mini_cycle_duration (ms) * 15
            pub fn add_phase(mut self, led_intensity: u8, fading_transition_duration: u16, led_duration: u16) -> Self {
                let led_intensity = {
                    let saturated = if 100 < led_intensity { 100 } else { led_intensity } as f32;
                    ((saturated / 6.25) as u8).into()
                };
                let fading_transition_duration: u4 = {
                    let gmcd: u8 = self.global_mini_cycle_duration.into();
                    (fading_transition_duration / gmcd as u16) as u8
                }.into();
                let led_duration = {
                    let gmcd: u8 = self.global_mini_cycle_duration.into();
                    (led_duration / gmcd as u16) as u8
                }.into();

                let phase = LightEmittingPhase {
                    led_intensity,
                    fading_transition_duration,
                    led_duration,
                };

                self.push_phase(phase);

                self
            }

            /// Does the 1st phase and then the LED stays on with LED Start Intensity.
            pub fn emit_first_phase(&self) -> Self {
                let mut pattern = self.clone();
                pattern.phases_len = Some(0u8.into());
                pattern.repeat_count = 0u8.into();

                pattern
            }
        }

        impl Into<[u8; 25]> for LightEmittingPattern {
            fn into(self) -> [u8; 25] {
                fn nibbles_to_u8(high: u4, low: u4) -> u8 {
                    let high = {
                        let high: u8 = high.into();
                        (high & 0x0F) << 4
                    };
                    let low = {
                        let low: u8 = low.into();
                        low & 0x0F
                    };

                    high | low
                }

                let mut buf = [0u8; 25];

                let number_of_phases =
                    if let Some(p) = self.phases_len {
                        p
                    } else {
                        (self.phases.len() as u8).into()
                    };
                buf[0] = nibbles_to_u8(number_of_phases, self.global_mini_cycle_duration);

                buf[1] = nibbles_to_u8(self.led_start_intensity, self.repeat_count);

                let mut even_phases = self.phases.iter()
                    .take(15)
                    .enumerate()
                    .filter(|(idx, _)| idx % 2 == 0)
                    .map(|e| e.1);
                let mut odd_phases = self.phases.iter()
                    .take(15)
                    .enumerate()
                    .filter(|(idx, _)| idx % 2 == 1)
                    .map(|e| e.1);


                let mut buf_index = 2;
                while let (Some(even), odd) = (even_phases.next(), odd_phases.next()) {
                    // LED intensities
                    {
                        let even_led_intensity = even.led_intensity;
                        let odd_led_intensity = odd.map(|odd| odd.led_intensity)
                            .unwrap_or(0u8.into());

                        buf[buf_index] = nibbles_to_u8(even_led_intensity, odd_led_intensity);
                        buf_index += 1;
                    }

                    // Even fading & led
                    {
                        let fading = even.fading_transition_duration;
                        let led = even.led_duration;
                        buf[buf_index] = nibbles_to_u8(fading, led);
                        buf_index += 1;
                    }

                    // Odd fading & led
                    if let Some(odd) = odd {
                        let fading = odd.fading_transition_duration;
                        let led = odd.led_duration;
                        buf[buf_index] = nibbles_to_u8(fading, led);
                        buf_index += 1;
                    }
                }

                buf
            }
        }
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
        /// # let manager = JoyConManager::new().unwrap();
        /// #
        /// # let device = manager.lock()
        /// #                     .unwrap()
        /// #                     .managed_devices()
        /// #                     .remove(0);
        /// #
        /// # let mut joycon_driver = SimpleJoyConDriver::new(&device).unwrap();
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
        /// # let manager = JoyConManager::new().unwrap();
        /// #
        /// # let device = manager.lock()
        /// #                     .unwrap()
        /// #                     .managed_devices()
        /// #                     .remove(0);
        /// #
        /// # let mut joycon_driver = SimpleJoyConDriver::new(&device).unwrap();
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
        /// # let manager = JoyConManager::new().unwrap();
        /// #
        /// # let device = manager.lock()
        /// #                     .unwrap()
        /// #                     .managed_devices()
        /// #                     .remove(0);
        /// #
        /// # let mut joycon_driver = SimpleJoyConDriver::new(&device).unwrap();
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
            Ok(reply)
        }

        /// Get status of player lights on controller.
        ///
        /// # Example
        ///
        /// ```no_run
        /// use joycon_rs::prelude::{*, lights::*};
        ///
        /// # let manager = JoyConManager::new().unwrap();
        /// #
        /// # let device = manager.lock()
        /// #                     .unwrap()
        /// #                     .managed_devices()
        /// #                     .remove(0);
        /// #
        /// # let mut joycon_driver = SimpleJoyConDriver::new(&device).unwrap();
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

        /// Set HOME light.
        ///
        /// # Example
        /// ```no_run
        /// use joycon_rs::prelude::{*, lights::{*, home_button::*}};
        ///
        /// # let manager = JoyConManager::new().unwrap();
        /// #
        /// # let device = manager.lock()
        /// #                     .unwrap()
        /// #                     .managed_devices()
        /// #                     .remove(0);
        /// #
        /// # let mut joycon_driver = SimpleJoyConDriver::new(&device).unwrap();
        /// let pattern =
        ///     // loop pattern forever
        ///     LightEmittingPattern::new(100, 0, 0u8.into())
        ///         // 0.5 seconds to light up
        ///         .add_phase(100,500,0)
        ///         // 0.5 seconds to turn off
        ///         .add_phase(0,500,0);
        /// let player_lights_status = joycon_driver.set_home_light(&pattern);
        /// ```
        fn set_home_light(&mut self, pattern: &home_button::LightEmittingPattern) -> JoyConResult<[u8; 362]> {
            let arg: [u8; 25] = pattern.clone().into();
            self.send_sub_command(SubCommand::SetHOMELight, &arg)
        }
    }

    impl<D> Lights for D where D: JoyConDriver {}
}

pub mod device_info {
    use super::{*, input_report_mode::sub_command_mode::*};

    impl TryFrom<u8> for JoyConDeviceType {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            let kind = match value {
                0 => JoyConDeviceType::JoyConL,
                1 => JoyConDeviceType::JoyConR,
                2 => JoyConDeviceType::ProCon,
                _ => Err(())?
            };

            Ok(kind)
        }
    }

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct JoyConMacAddress(pub [u8; 6]);

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct JoyConDeviceInfo {
        pub firmware_version: u16,
        pub device_type: JoyConDeviceType,
        pub mac_address: JoyConMacAddress,
        pub colors_in_spi: bool,
    }

    impl TryFrom<[u8; 35]> for JoyConDeviceInfo {
        type Error = JoyConError;

        fn try_from(value: [u8; 35]) -> Result<Self, Self::Error> {
            let firmware_version = u16::from_be_bytes([value[0], value[1]]);
            let device_kind = JoyConDeviceType::try_from(value[2])
                .map_err(|()| {
                    JoyConError::SubCommandError(SubCommand::RequestDeviceInfo as u8, value.to_vec())
                })?;
            let mac_address = {
                let mut buf = [0u8; 6];
                buf.copy_from_slice(&value[4..10]);
                JoyConMacAddress(buf)
            };
            let colors_in_spi = value[12] == 1;

            Ok(JoyConDeviceInfo {
                firmware_version,
                device_type: device_kind,
                mac_address,
                colors_in_spi,
            })
        }
    }

    impl SubCommandReplyData for JoyConDeviceInfo {
        type ArgsType = [u8; 0];
        const SUB_COMMAND: SubCommand = SubCommand::RequestDeviceInfo;
        const ARGS: Self::ArgsType = [];
    }
}

pub mod spi {
    use super::{*,
                input_report_mode::{
                    sub_command_mode::*,
                    StandardInputReport,
                },
    };

    // todo replace with `.to_le_bytes()` after it is stabilised.

    /// Convert i32 address and i8 length to u8 array
    ///
    /// # Notice
    /// ***`Length <= 0x1D`***
    pub const fn spi_target(address: u32, length: u8) -> [u8; 5] {
        // let address_le_bytes: [u8;4] = address.to_le_bytes();
        // let length: [u8;1] = if length > 0x1D {
        //     0x1D
        // } else {
        //     length
        // }.to_le_bytes();

        let address_le_bytes = [
            (address & 0xFF) as u8,
            ((address & 0xFF00) >> 8) as u8,
            ((address & 0xFF0000) >> 16) as u8,
            ((address & 0xFF000000) >> 24) as u8,
        ];

        [
            address_le_bytes[0],
            address_le_bytes[1],
            address_le_bytes[2],
            address_le_bytes[3],
            length
        ]
    }

    #[test]
    fn test_spi_target() {
        let target = spi_target(0x6080, 0x18);
        assert_eq!(target, [0x80, 0x60, 0x00, 0x00, 0x18]);
    }

    pub trait SPIData: TryFrom<[u8; 35], Error=JoyConError> {
        const ADDRESS: u32;
        const LENGTH: u8;
    }

    fn setup_memory_read_target(address: u32, length: u8) -> [u8; 8] {
        let args = spi_target(address + 0xF8000000, length);
        [
            // FEATURE 0x71: Setup memory read
            0x71,
            // UInt32LE address
            args[0],
            args[1],
            args[2],
            args[3],
            // UInt16LE size. Max xF9 bytes.
            args[4],
            0x00,
            // Checksum (8-bit 2s Complement): calculated as 0x100 - `Sum of Bytes`.
            {
                let acc = ((0x71
                    + args[0] as u32
                    + args[1] as u32
                    + args[2] as u32
                    + args[3] as u32
                    + args[4] as u32) & 0xFF) as u8;
                let cmp = std::u8::MAX - acc + 1;

                cmp
            }
        ]
    }

    #[test]
    fn test_setup_memory_read_target() {
        let target = setup_memory_read_target(0x1FF4, 0x08);
        assert_eq!(target, [0x71, 0xF4, 0x1F, 0x00, 0xF8, 0x08, 0x00, 0x7C]);

        let target = setup_memory_read_target(0x603D,18);
        assert_eq!(target, [0x71,0x3D,0x60,0x0,0xF8,0x12,0x0,0xE8]);
    }

    impl<T: SPIData> SubCommandReplyData for T {
        type ArgsType = [u8; 5];
        const SUB_COMMAND: SubCommand = SubCommand::SPIFlashRead;
        const ARGS: Self::ArgsType = spi_target(Self::ADDRESS, Self::LENGTH);

        fn once<D>(driver: &mut D) -> JoyConResult<StandardInputReport<SubCommandReport<Self>>>
            where Self: std::marker::Sized,
                  D: JoyConDriver
        {
            // let args = spi_target(Self::ADDRESS + 0xF8000000, Self::LENGTH);
            // let mut report = [
            //     // FEATURE 0x71: Setup memory read
            //     0x71,
            //     // UInt32LE address
            //     args[0],
            //     args[1],
            //     args[2],
            //     args[3],
            //     // UInt16LE size. Max xF9 bytes.
            //     args[4],
            //     0x00,
            //     // Checksum (8-bit 2s Complement): calculated as 0x100 - `Sum of Bytes`.
            //     {
            //         let acc = ((0x71
            //             + args[0] as u32
            //             + args[1] as u32
            //             + args[2] as u32
            //             + args[3] as u32
            //             + args[4] as u32) & 0xFF) as u8;
            //         let cmp = std::u8::MAX - acc + 1;
            //
            //         cmp
            //     }
            // ];
            // println!("[0x{:X},0x{:X},0x{:X},0x{:X},0x{:X},0x{:X},0x{:X},0x{:X}]",
            //          report[0],
            //          report[1],
            //          report[2],
            //          report[3],
            //          report[4],
            //          report[5],
            //          report[6],
            //          report[7], );
            //
            // driver.send_feature_report(&report)?;
            // std::thread::sleep(std::time::Duration::from_millis(50));
            // let mut buf = [0u8;50];
            // driver.get_feature_report(&mut buf[..])?;
            // dbg!(buf.to_vec());
            dbg!(line!());
            let reply = driver.send_sub_command(Self::SUB_COMMAND, Self::ARGS.as_ref())?;
            dbg!(line!());
            StandardInputReport::try_from(reply)
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
