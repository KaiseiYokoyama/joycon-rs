#![allow(unused_must_use)]

use joycon_rs::prelude::{*, lights::*};
use std::convert::TryInto;
use std::ops::Deref;
use joycon_rs::joycon::lights::home_button::LightEmittingPattern;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!

    let manager =
        JoyConManager::new()?;
    let (managed_devices, new_devices) = {
        let lock = manager.lock();
        match lock {
            Ok(manager) =>
                (manager.managed_devices(), manager.new_devices()),
            Err(_) => unreachable!(),
        }
    };

    managed_devices.into_iter()
        .chain(new_devices)
        .inspect(|d| {
            let lock = d.lock();
            let device = match lock {
                Ok(device) => device,
                Err(e) => e.into_inner(),
            };
            let hid_device: JoyConResult<&HidDevice> = device.deref().try_into();
            if let Ok(hid_device) = hid_device {
                println!("{:?}", hid_device.get_product_string())
            }
        })
        .try_for_each::<_, JoyConResult<()>>(|d| {
            let mut driver = SimpleJoyConDriver::new(&d)?;

            let pattern =
                LightEmittingPattern::new(100, 0, 0u8.into())
                    .add_phase(100,500,0)
                    .add_phase(0,500,0);
            driver.set_home_light(&pattern);

            Ok(())
        })?;

    Ok(())
}