//! Operate Joy-Con's player lights (LEDs). The gist of this module is [`Lights`].
//!
//! [`Lights`]: trait.Lights.html
//!
//! # Usage
//! ```no_run
//! use joycon_rs::prelude::{*, lights::*};
//!
//! let manager = JoyConManager::get_instance();
//!
//! let device = manager.lock()
//!                     .unwrap()
//!                     .managed_devices()
//!                     .remove(0);
//!
//! let mut joycon_driver = SimpleJoyConDriver::new(&device).unwrap();
//!
//! // Set player lights lightning and flashing.
//! joycon_driver.set_player_lights(&vec![LightUp::LED2], &vec![Flash::LED3]).unwrap();
//!
//! // Get status of player lights
//! if let Ok(SubCommandReply::Checked(checked_reply)) = joycon_driver.get_player_lights() {
//!     let player_lights_status = checked_reply.extra;
//!     dbg!(player_lights_status);
//! };
//! ```

use super::{input_report_mode::sub_command_mode::*, *};
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

const LIGHT_UP: [LightUp; 4] = [LightUp::LED0, LightUp::LED1, LightUp::LED2, LightUp::LED3];
const FLASH: [Flash; 4] = [Flash::LED0, Flash::LED1, Flash::LED2, Flash::LED3];

impl TryFrom<[u8; 35]> for LightsStatus {
    type Error = JoyConError;

    fn try_from(value: [u8; 35]) -> Result<Self, Self::Error> {
        let value = value[0];

        // parse reply
        let light_up = LIGHT_UP
            .iter()
            .filter(|&&l| {
                let light = l as u8;
                value & light == light
            })
            .cloned()
            .collect();
        let flash = FLASH
            .iter()
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
            } else {
                value
            }
        }
    }

    impl From<u4> for u8 {
        fn from(s: u4) -> u8 {
            s.0
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
        pub fn new(
            global_mini_cycle_duration: u8,
            led_start_intensity: u8,
            repeat_count: u4,
        ) -> Self {
            let global_mini_cycle_duration = if global_mini_cycle_duration == 0 {
                0.into()
            } else {
                ((global_mini_cycle_duration - 7) / 12 + 1).into()
            };

            let led_start_intensity = {
                let saturated = if 100 < led_start_intensity {
                    100
                } else {
                    led_start_intensity
                } as f32;
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

        pub fn phases(&self) -> &Vec<LightEmittingPhase> {
            &self.phases
        }

        /// Add emitting phase to pattern.
        ///
        /// `fading_transition_duration` and `led_duration` is represented by 4-bit unsigned int
        /// in field and is treated as a multiplier of the `LightEmittingPattern.global_mini_cycle_duration`
        /// specified by the first argument of LightEmittingPattern::new().
        ///
        /// Therefore, depending on the combination of the `LightEmittingPattern.global_mini_cycle_duration`
        /// and the specified value, different values may be regarded as the same value
        /// when converted to 4-bit unsigned int, and no difference may appear in the luminous pattern.
        ///
        /// * led_intensity (*%*) - 0 <= led_intensity <= 100
        /// * fading_transition_duration (*ms*) - 0 < fading_transition_duration < self.global_mini_cycle_duration (ms) * 15
        /// * led_duration (*ms*) - 0 < fading_transition_duration < self.global_mini_cycle_duration (ms) * 15
        pub fn add_phase(
            mut self,
            led_intensity: u8,
            fading_transition_duration: u16,
            led_duration: u16,
        ) -> Self {
            let led_intensity = {
                let saturated = if 100 < led_intensity {
                    100
                } else {
                    led_intensity
                } as f32;
                ((saturated / 6.25) as u8).into()
            };
            let fading_transition_duration: u4 = {
                let gmcd: u8 = self.global_mini_cycle_duration.into();
                (fading_transition_duration / gmcd as u16) as u8
            }
            .into();
            let led_duration = {
                let gmcd: u8 = self.global_mini_cycle_duration.into();
                (led_duration / gmcd as u16) as u8
            }
            .into();

            let phase = LightEmittingPhase {
                led_intensity,
                fading_transition_duration,
                led_duration,
            };

            self.push_phase(phase);

            self
        }

        /// Does the 1st phase and then the LED stays on with LED Start Intensity.
        ///
        /// For more information about the arguments,
        /// see the [new](#method.new) and [add_phase](#method.add_phase).
        pub fn once(
            global_mini_cycle_duration: u8,
            led_start_intensity: u8,
            led_intensity: u8,
            fading_transition_duration: u16,
            led_duration: u16,
        ) -> Self {
            let mut pattern = LightEmittingPattern::new(
                global_mini_cycle_duration,
                led_start_intensity,
                0u8.into(),
            );
            pattern.phases_len = Some(0u8.into());

            pattern.add_phase(led_intensity, fading_transition_duration, led_duration)
        }
    }

    impl From<LightEmittingPattern> for [u8; 25] {
        fn from(s: LightEmittingPattern) -> [u8; 25] {
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

            let number_of_phases = if let Some(p) = s.phases_len {
                p
            } else {
                (s.phases.len() as u8).into()
            };
            buf[0] = nibbles_to_u8(number_of_phases, s.global_mini_cycle_duration);

            buf[1] = nibbles_to_u8(s.led_start_intensity, s.repeat_count);

            let mut even_phases = s
                .phases
                .iter()
                .take(15)
                .enumerate()
                .filter(|(idx, _)| idx % 2 == 0)
                .map(|e| e.1);
            let mut odd_phases = s
                .phases
                .iter()
                .take(15)
                .enumerate()
                .filter(|(idx, _)| idx % 2 == 1)
                .map(|e| e.1);

            let mut buf_index = 2;
            while let (Some(even), odd) = (even_phases.next(), odd_phases.next()) {
                // LED intensities
                {
                    let even_led_intensity = even.led_intensity;
                    let odd_led_intensity = odd
                        .map(|odd| odd.led_intensity)
                        .unwrap_or_else(|| 0u8.into());

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
    /// # let manager = JoyConManager::get_instance();
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
    /// # let manager = JoyConManager::get_instance();
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
    /// # let manager = JoyConManager::get_instance();
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
    fn set_player_lights(
        &mut self,
        light_up: &[LightUp],
        flash: &[Flash],
    ) -> JoyConResult<SubCommandReply<[u8; 362]>> {
        let arg = light_up.iter().map(|&lu| lu as u8).sum::<u8>()
            + flash.iter().map(|&f| f as u8).sum::<u8>();

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
    /// # let manager = JoyConManager::get_instance();
    /// #
    /// # let device = manager.lock()
    /// #                     .unwrap()
    /// #                     .managed_devices()
    /// #                     .remove(0);
    /// #
    /// # let mut joycon_driver = SimpleJoyConDriver::new(&device).unwrap();
    /// if let Ok(SubCommandReply::Checked(checked_reply)) = joycon_driver.get_player_lights() {
    ///     let player_lights_status = checked_reply.extra;
    ///     dbg!(player_lights_status);
    /// };
    /// ```
    ///
    fn get_player_lights(
        &mut self,
    ) -> JoyConResult<SubCommandReply<StandardInputReport<SubCommandReport<LightsStatus>>>>
    where
        Self: std::marker::Sized,
    {
        LightsStatus::once(self)
    }

    /// Set HOME light.
    ///
    /// # Example
    /// ```no_run
    /// use joycon_rs::prelude::{*, lights::{*, home_button::*}};
    ///
    /// # let manager = JoyConManager::get_instance();
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
    fn set_home_light(
        &mut self,
        pattern: &home_button::LightEmittingPattern,
    ) -> JoyConResult<SubCommandReply<[u8; 362]>> {
        let arg: [u8; 25] = pattern.clone().into();
        self.send_sub_command(SubCommand::SetHOMELight, &arg)
    }
}

impl<D> Lights for D where D: JoyConDriver {}
