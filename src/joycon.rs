use crate::prelude::*;

pub use joycon_device::JoyConDevice;
pub use joycon::JoyCon;
pub use driver::{Rotation, JoyConDriver, GlobalPacketNumber};

use std::sync::Arc;
use std::fmt::{Debug, Formatter, Error};

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
        JoyConR(Arc<HidDevice>),
        JoyConL(Arc<HidDevice>),
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

            let device = Arc::new(device_info.open_device(&hidapi)?);

            match device_info.product_id() {
                Self::PRODUCT_ID_JOYCON_L => Ok(JoyConDevice::JoyConL(device)),
                Self::PRODUCT_ID_JOYCON_R => Ok(JoyConDevice::JoyConR(device)),
                other => Err(JoyConDeviceError::InvalidProductID(other))?,
            }
        }
    }

    impl Deref for JoyConDevice {
        type Target = Arc<HidDevice>;

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

mod joycon {
    use super::*;

    /// The controller user uses to play with.
    #[derive(Clone)]
    pub enum JoyCon {
        JoyConR(Arc<HidDevice>),
        JoyConL(Arc<HidDevice>),
        JoyConLR {
            joycon_l: Arc<HidDevice>,
            joycon_r: Arc<HidDevice>,
        },
        // note: I'll do it later.
        // Procon(Arc<HidDevice>)
        //     procon: Arc<HidDevice>,
        // }
    }

    impl From<JoyConDevice> for JoyCon {
        fn from(jd: JoyConDevice) -> Self {
            match jd {
                JoyConDevice::JoyConR(r) => JoyCon::JoyConR(r),
                JoyConDevice::JoyConL(l) => JoyCon::JoyConL(l),
            }
        }
    }

    impl Debug for JoyCon {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f,
                     "{}({:?})",
                     match self {
                         JoyCon::JoyConR { .. } => "JoyConR",
                         JoyCon::JoyConL { .. } => "JoyConL",
                         JoyCon::JoyConLR { .. } => "JoyConLR",
                     },
                     match self {
                         JoyCon::JoyConL(joycon) => vec![DebugHidDevice(joycon)],
                         JoyCon::JoyConR(joycon) => vec![DebugHidDevice(joycon)],
                         JoyCon::JoyConLR { joycon_l, joycon_r, .. } =>
                             vec![DebugHidDevice(joycon_l), DebugHidDevice(joycon_r)]
                     })
        }
    }
}

mod driver {
    use super::*;
    pub use global_packet_number::GlobalPacketNumber;

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

    mod global_packet_number {
        use std::ops::Add;

        /// Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct GlobalPacketNumber(u8);

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

    #[derive(Debug, Clone)]
    pub struct JoyConDriver {
        /// The controller user uses
        pub joycon: JoyCon,
        /// rotation of controller
        pub rotation: Rotation,
        /// Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
        global_packet_number: GlobalPacketNumber,
    }

    impl JoyConDriver {}
}

pub struct JoyConManager {
    hidapi: Arc<HidApi>,
    pub connected_joycon_devices: Vec<JoyConDevice>,
}

impl JoyConManager {
    pub fn new() -> JoyConResult<Self> {
        let hidapi = HidApi::new()?;
        let devices = hidapi.device_list()
            .flat_map(|di| JoyConDevice::new(di, &hidapi))
            .collect();

        Ok(Self {
            hidapi: Arc::new(hidapi),
            connected_joycon_devices: devices,
        })
    }
}