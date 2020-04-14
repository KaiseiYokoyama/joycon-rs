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