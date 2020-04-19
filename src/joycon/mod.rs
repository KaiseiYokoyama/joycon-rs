use crate::prelude::*;

pub use device::JoyConDevice;
pub use driver::{
    Rotation,
    JoyConDriver,
    GlobalPacketNumber,
    SimpleJoyConDriver,
    Command,
    SubCommand,
    input_report_mode::{self, InputReportMode, SimpleHIDMode, StandardFullMode, SubCommandMode},
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

mod device;
mod driver;

/// A manager for dealing with Joy-Cons.
pub struct JoyConManager {
    hidapi: Arc<HidApi>,
    pub connected_joycon_devices: Vec<JoyConDevice>,
}

impl JoyConManager {
    /// Search Joy-Con devices and store.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use joycon_rs::prelude::JoyConManager;
    ///
    /// let manager = JoyConManager::new().unwrap();
    /// manager.connected_joycon_devices.into_iter()
    ///     .for_each(|joycon_device| {
    ///         // do something amazing with Joy-Con
    ///     });
    /// ```
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