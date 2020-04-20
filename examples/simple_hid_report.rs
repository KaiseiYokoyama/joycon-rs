#![allow(unused_must_use)]
use joycon_rs::prelude::*;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!

    let (tx, rx) =
        std::sync::mpsc::channel();

    let _output = std::thread::spawn(move || {
        // Push buttons or tilt the stick please.
        // Stop with `Cmd + C` or `Ctrl + C`
        while let Ok(message) = rx.recv() {
            dbg!(message);
        }
    });

    let manager = JoyConManager::new()?;

    let (managed_devices, new_devices) = {
        let lock = manager.lock();
        match lock {
            Ok(manager) => (manager.managed_devices(), manager.new_devices()),
            Err(_) => unreachable!(),
        }
    };

    managed_devices.into_iter()
        .chain(new_devices)
        .try_for_each::<_, JoyConResult<()>>(|d| {
            let driver = SimpleJoyConDriver::new(&d)?;

            let simple_hid_mode = SimpleHIDMode::new(driver)?;
            let tx = tx.clone();

            std::thread::spawn(move || {
                loop {
                    tx.send(simple_hid_mode.read_input_report()).unwrap();
                }
            });

            Ok(())
        })?;

    Ok(())
}