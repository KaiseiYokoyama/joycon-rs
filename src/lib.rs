#![doc(html_logo_url = "https://user-images.githubusercontent.com/8509057/79100490-9a4a7900-7da1-11ea-9ee4-5e15439bbd0c.png")]
//! # Joycon-rs Library Documentation
//!
//! Hello, and welcome to joycon-rs documentation.
//!
//! Joycon-rs is a framework for dealing with Nintendo Switch Joy-Con on Rust easily and efficiently.
//! In a way, this library is a wrapper of [`hidapi-rs`].
//! This is a free and open source library, the source code is available for download on [Github].
//!
//! Joycon-rs is in development and is still incomplete.
//! Please be aware that the update will include breaking changes for the time being. Pardon out dust!
//!
//! # Usage
//! First, add dependency to `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! joycon_rs = "*"
//! ```
//!
//! Then, `use` prelude on `.rs` file.
//! ```
//! use joycon_rs::prelude::*;
//! ```
//!
//! Perfect! Now you have Joycon-rs available in code.
//!
//! ### Receive reports
//! For starters, let's take a simple signal from JoyCon.
//! If you have more than one JoyCon, [`mspc`] can be very helpful.
//!
//! ```no_run
//! use joycon_rs::prelude::*;
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! let _output = std::thread::spawn( move || {
//!     while let Ok(update) = rx.recv() {
//!         dbg!(update);
//!     }
//! });
//!
//! let manager = JoyConManager::new().unwrap();
//!
//! let (managed_devices, new_devices) = {
//!     let lock = manager.lock();
//!     match lock {
//!         Ok(manager) => (manager.managed_devices(), manager.new_devices()),
//!         Err(_) => return,
//!     }
//! };
//!
//! managed_devices.into_iter()
//!     .chain(new_devices)
//!     .flat_map(|dev| SimpleJoyConDriver::new(&dev))
//!     .try_for_each::<_, JoyConResult<()>>(|driver| {
//!         // Change JoyCon to Simple hid mode.
//!         let simple_hid_mode = SimpleHIDMode::new(driver)?;
//!
//!         let tx = tx.clone();
//!
//!         // Spawn thread
//!         std::thread::spawn( move || {
//!             loop {
//!                 // Forward the report to the main thread
//!                 tx.send(simple_hid_mode.read_input_report()).unwrap();
//!             }
//!         });
//!
//!         Ok(())
//!     })
//!     .unwrap();
//! ```
//!
//! ### Set player lights
//! Then, lets deal with player lights.
//!
//! ```no_run
//! use joycon_rs::prelude::{*, lights::*};
//! use joycon_rs::joycon::input_report_mode::StandardInputReport;
//! use joycon_rs::joycon::input_report_mode::sub_command_mode::SubCommandReport;
//!
//! let (tx, rx) =
//!     std::sync::mpsc::channel::<JoyConResult<StandardInputReport<SubCommandReport<LightsStatus>>>>();
//!
//! // Receive status of player lights
//! std::thread::spawn(move ||{
//!     while let Ok(Ok(light_status)) = rx.recv() {
//!         assert_eq!(
//!             light_status.extra.reply,
//!             LightsStatus {
//!                 light_up: vec![LightUp::LED1, LightUp::LED3],
//!                 flash: vec![Flash::LED0, Flash::LED2],
//!             }
//!         )
//!     }
//! });
//!
//! let manager = JoyConManager::new().unwrap();
//!
//! let (managed_devices, new_devices) = {
//!     let lock = manager.lock();
//!     match lock {
//!         Ok(manager) => (manager.managed_devices(), manager.new_devices()),
//!         Err(_) => return,
//!     }
//! };
//!
//! managed_devices.into_iter()
//!     .chain(new_devices)
//!     .flat_map(|dev| SimpleJoyConDriver::new(&dev))
//!     .try_for_each::<_, JoyConResult<()>>(|mut driver| {
//!         // Set player lights
//!         // [SL BUTTON] 📸💡📸💡 [SR BUTTON]
//!         driver.set_player_lights(&vec![LightUp::LED1, LightUp::LED3], &vec![Flash::LED0, Flash::LED2]).unwrap();
//!         tx.send(driver.get_player_lights()).unwrap();
//!         Ok(())
//!     })
//!     .unwrap();
//! ```
//!
//! # Features
//! You can use `Joycon-rs` for...
//! - [Send] / [Receive] raw packets (u8 array) to / from Joy-Con
//! - [Receive input to Joy-Con][input_report_mode]
//!     - [Receive pushed buttons, and stick directions (one of 8 directions) on every button pressed.][SimpleHIDMode<D>]
//!     - [Receive pushed buttons, stick directions (analog value), and 6-Axis sensor at 60Hz.][StandardFullMode<D>]
//!     - [Get status of Joy-Con][SubCommandMode<D, RD>]
//! - [Deal with LED (Player lights)]
//! - [Rumble]
//!
//! ## Planning
//! - Receive NFC/IR data
//! - Deal with HOME light
//!
//! [Github]: https://github.com/KaiseiYokoyama/joycon-rs
//! [`hidapi-rs`]: https://github.com/ruabmbua/hidapi-rs
//! [`mspc`]: https://doc.rust-lang.org/book/ch16-02-message-passing.html
//! [Send]: joycon/trait.JoyConDriver.html#tymethod.write
//! [Receive]: joycon/trait.JoyConDriver.html#tymethod.read
//! [input_report_mode]: joycon/input_report_mode/index.html
//! [SimpleHIDMode<D>]: joycon/input_report_mode/simple_hid_mode/struct.SimpleHIDMode.html
//! [StandardFullMode<D>]: joycon/input_report_mode/standard_full_mode/struct.StandardFullMode.html
//! [SubCommandMode<D, RD>]: joycon/input_report_mode/sub_command_mode/struct.SubCommandMode.html
//! [Deal with LED (Player lights)]: joycon/lights/index.html
//! [Rumble]:joycon/struct.Rumble.html
pub mod joycon;

#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;

#[cfg(doctest)]
doctest!("../README.md");

pub mod prelude {
    pub use hidapi::*;
    pub use crossbeam_channel;
    pub use crate::result::*;
    pub use crate::joycon::*;
}

pub mod result {
    // use crate::prelude::SubCommand;
    use hidapi::HidError;

    #[derive(Debug)]
    pub enum JoyConError {
        HidApiError(hidapi::HidError),
        // SubCommandError(SubCommand),
        SubCommandError(u8, Vec<u8>),
        JoyConDeviceError(JoyConDeviceError),
        JoyConReportError(JoyConReportError),
        Disconnected,
    }

    impl From<hidapi::HidError> for JoyConError {
        fn from(e: HidError) -> Self {
            JoyConError::HidApiError(e)
        }
    }

    #[derive(Debug)]
    pub enum JoyConDeviceError {
        InvalidVendorID(u16),
        InvalidProductID(u16),
    }

    impl From<JoyConDeviceError> for JoyConError {
        fn from(e: JoyConDeviceError) -> Self {
            JoyConError::JoyConDeviceError(e)
        }
    }

    #[derive(Debug)]
    pub enum JoyConReportError {
        InvalidSimpleHidReport(InvalidSimpleHIDReport),
        InvalidStandardInputReport(InvalidStandardInputReport),
    }

    impl From<JoyConReportError> for JoyConError {
        fn from(e: JoyConReportError) -> Self {
            JoyConError::JoyConReportError(e)
        }
    }

    #[derive(Debug)]
    pub enum InvalidSimpleHIDReport {
        InvalidReport(Vec<u8>),
        InvalidStickDirection(u8),
    }

    impl From<InvalidSimpleHIDReport> for JoyConReportError {
        fn from(e: InvalidSimpleHIDReport) -> Self {
            JoyConReportError::InvalidSimpleHidReport(e)
        }
    }

    impl From<InvalidSimpleHIDReport> for JoyConError {
        fn from(e: InvalidSimpleHIDReport) -> Self {
            let report_error = JoyConReportError::from(e);
            JoyConError::from(report_error)
        }
    }

    #[derive(Debug)]
    pub enum InvalidStandardInputReport {
        InvalidReport(Vec<u8>),
        InvalidExtraReport(Vec<u8>),
        Battery(u8),
        ConnectionInfo(u8),
        InvalidInputReportId(u8),
    }

    impl From<InvalidStandardInputReport> for JoyConReportError {
        fn from(e: InvalidStandardInputReport) -> Self {
            JoyConReportError::InvalidStandardInputReport(e)
        }
    }

    impl From<InvalidStandardInputReport> for JoyConError {
        fn from(e: InvalidStandardInputReport) -> Self {
            let report_error = JoyConReportError::from(e);
            JoyConError::from(report_error)
        }
    }

    pub type JoyConResult<T> = Result<T, JoyConError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
