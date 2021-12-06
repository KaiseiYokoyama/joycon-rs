#![allow(unused_must_use)]

use joycon_rs::prelude::*;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!

    let (tx, rx) = std::sync::mpsc::channel();

    let _output = std::thread::spawn(move || {
        // Push buttons or tilt the stick please.
        // Stop with `Cmd + C` or `Ctrl + C`
        while let Ok(message) = rx.recv() {
            dbg!(message);
        }
    });

    let manager = JoyConManager::get_instance();

    let devices = {
        let lock = manager.lock();
        match lock {
            Ok(manager) => manager.new_devices(),
            Err(_) => unreachable!(),
        }
    };

    devices.iter().try_for_each::<_, JoyConResult<()>>(|d| {
        let driver = SimpleJoyConDriver::new(&d)?;
        let standard_full_mode = StandardFullMode::new(driver)?;
        let tx = tx.clone();

        std::thread::spawn(move || loop {
            tx.send(standard_full_mode.read_input_report()).unwrap();
        });

        Ok(())
    })?;

    Ok(())
}
