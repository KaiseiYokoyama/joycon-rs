#![allow(unused_must_use)]

use joycon_rs::prelude::{lights::*, *};
use std::convert::TryInto;
use std::ops::Deref;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!

    let manager = JoyConManager::get_instance();
    let devices = {
        let lock = manager.lock();
        match lock {
            Ok(manager) => manager.new_devices(),
            Err(_) => unreachable!(),
        }
    };

    devices
        .iter()
        .inspect(|d| {
            let lock = d.lock();
            let device = match lock {
                Ok(device) => device,
                Err(e) => e.into_inner(),
            };
            dbg!(&device);
            let hid_device: JoyConResult<&HidDevice> = device.deref().try_into();
            if let Ok(hid_device) = hid_device {
                println!("{:?}", hid_device.get_product_string())
            }
        })
        .try_for_each::<_, JoyConResult<()>>(|d| {
            let mut driver = SimpleJoyConDriver::new(&d)?;

            let lights_status = LightsStatus {
                light_up: vec![LightUp::LED1, LightUp::LED2],
                flash: vec![Flash::LED0, Flash::LED3],
            };

            // Set player lights
            driver.set_player_lights(&lights_status.light_up, &lights_status.flash)?;

            // Get player lights
            if let SubCommandReply::Checked(reply) = driver.get_player_lights()? {
                let lights_status_received = reply.extra.reply;
                assert_eq!(lights_status_received, lights_status);
            }

            Ok(())
        })?;

    Ok(())
}
