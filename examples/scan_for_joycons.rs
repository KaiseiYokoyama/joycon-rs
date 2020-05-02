#![allow(unused_must_use)]

use joycon_rs::prelude::*;
use std::convert::TryInto;
use std::ops::Deref;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!

    let manager = JoyConManager::get_instance();
    let (managed_devices, new_devices) = {
        let lock = manager.lock();
        match lock {
            Ok(m) => (m.managed_devices(), m.new_devices()),
            Err(_) => unreachable!(),
        }
    };

    managed_devices.into_iter()
        .chain(new_devices)
        .try_for_each::<_, JoyConResult<()>>(|d| {
            if let Ok(device) = d.lock() {
                let device: &HidDevice = device.deref().try_into()?;
                println!("{:?}", device.get_product_string()?);
            }
            Ok(())
        })?;

    Ok(())
}
