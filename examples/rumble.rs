use joycon_rs::prelude::*;
use std::convert::TryInto;
use std::ops::Deref;
use joycon_rs::joycon::joycon_features::JoyConFeature;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!

    let manager =
        JoyConManager::get_instance();
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

            driver.enable_feature(JoyConFeature::Vibration)?;

            // let rumble = Rumble::new(80.0,0.2);
            let rumble = Rumble::new(300.0,0.9);
            driver.rumble((Some(rumble), Some(rumble)))?;

            Ok(())
        })?;

    Ok(())
}