#![allow(unused_must_use)]

use std::{thread, time::Duration};

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
        let tx = tx.clone();

        std::thread::spawn(move || {
            loop {
                // Lock joycon and check if it is connected
                if match d.lock() {
                    Ok(d) => d,
                    Err(d) => d.into_inner(),
                }
                .is_connected()
                {
                    // Lock is now dropped
                    // Create driver and listen to standard reports
                    if let Ok(driver) = SimpleJoyConDriver::new(&d) {
                        if let Ok(standard_full_mode) = StandardFullMode::new(driver) {
                            // Report loop
                            loop {
                                match standard_full_mode.read_input_report() {
                                    Ok(report) => tx.send(report).unwrap(),
                                    Err(JoyConError::Disconnected) => {
                                        // Break out of report loop when device is disconnected
                                        break;
                                    }
                                    Err(_) => {} // Ignore other errors
                                };
                            }
                        }
                    }
                }
                // Joycon was disconnected, check for reconnection after 1 second
                thread::sleep(Duration::from_millis(1000));
            }
        });

        Ok(())
    })?;

    Ok(())
}
