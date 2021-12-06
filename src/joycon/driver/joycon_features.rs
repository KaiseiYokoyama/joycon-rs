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
                accelerometer_anti_aliasing_filter_bandwidth as u8,
            ]
        }
    }

    #[allow(clippy::derive_hash_xor_eq)]
    impl Hash for IMUConfig {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            // Returns constant value.
            0.hash(state);
        }
    }
}
