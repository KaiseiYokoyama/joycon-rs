use joycon_rs::prelude::*;
use std::convert::TryInto;
use std::ops::Deref;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!
    JoyConManager::new()?
        .lock()
        .unwrap()
        .connected_devices()
        .iter()
        .try_for_each::<_, JoyConResult<()>>(|d| {
            if let Ok(device) = d.lock() {
                let device: &HidDevice = device.deref().try_into()?;
                println!("{:?}", device.get_product_string()?);
            }
            Ok(())
        })?;

    Ok(())
}
