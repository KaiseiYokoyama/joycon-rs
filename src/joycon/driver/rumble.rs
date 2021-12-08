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
/// let manager = JoyConManager::get_instance();
/// let devices = {
///     let lock = manager.lock();
///     match lock {
///         Ok(manager) => manager.new_devices(),
///         Err(_) => unreachable!(),
///     }
/// };
///
/// devices.iter()
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
    pub fn frequency(self) -> f32 {
        self.frequency
    }

    pub fn amplitude(self) -> f32 {
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
    pub fn is_safe(self) -> bool {
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

impl From<Rumble> for [u8; 4] {
    fn from(s: Rumble) -> [u8; 4] {
        let encoded_hex_freq = f32::round(f32::log2(s.frequency / 10.0) * 32.0) as u8;

        let hf_freq: u16 = (encoded_hex_freq as u16).saturating_sub(0x60) * 4;
        let lf_freq: u8 = encoded_hex_freq.saturating_sub(0x41) + 1;

        let encoded_hex_amp = if s.amplitude > 0.23 {
            f32::round(f32::log2(s.amplitude * 8.7) * 32.0) as u8
        } else if s.amplitude > 0.12 {
            f32::round(f32::log2(s.amplitude * 17.0) * 16.0) as u8
        } else {
            f32::round(((f32::log2(s.amplitude) * 32.0) - 96.0) / (4.0 - 2.0 * s.amplitude))
                as u8
        };

        let hf_amp: u16 = {
            let hf_amp: u16 = encoded_hex_amp as u16 * 2;
            if hf_amp > 0x01FC {
                0x01FC
            } else {
                hf_amp
            }
        }; // encoded_hex_amp<<1;
        let lf_amp: u8 = {
            let lf_amp = encoded_hex_amp / 2 + 64;
            if lf_amp > 0x7F {
                0x7F
            } else {
                lf_amp
            }
        }; // (encoded_hex_amp>>1)+0x40;

        let mut buf = [0u8; 4];

        // HF: Byte swapping
        buf[0] = (hf_freq & 0xFF) as u8;
        // buf[1] = (hf_amp + ((hf_freq >> 8) & 0xFF)) as u8; //Add amp + 1st byte of frequency to amplitude byte
        buf[1] = (hf_amp + (hf_freq.wrapping_shr(8) & 0xFF)) as u8; //Add amp + 1st byte of frequency to amplitude byte

        // LF: Byte swapping
        buf[2] = lf_freq.saturating_add(lf_amp.wrapping_shr(8));
        buf[3] = lf_amp;

        buf
    }
}
