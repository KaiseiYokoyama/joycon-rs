use crate::prelude::*;

pub use device::{JoyConDevice, JoyConDeviceType};
pub use driver::{
    Rotation,
    Rumble,
    joycon_features,
    SubCommandReply,
    JoyConDriver,
    GlobalPacketNumber,
    SimpleJoyConDriver,
    Command,
    SubCommand,
    input_report_mode::{self, InputReportMode, SimpleHIDMode, StandardFullMode},
    lights,
    device_info,
};
pub use manager::{JoyConManager, JOYCON_RECEIVER};

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
mod manager;