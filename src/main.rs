use joycon_rs::prelude::*;
use joycon_rs::joycon::{JoyConManager, SimpleJoyConDriver, Rotation, JoyConDriver, SubCommand, input_report_mode::*};
// use joycon_rs::joycon::input_report_mode::simple_hid_mode::SimpleHIDMode;
use joycon_rs::joycon::input_report_mode::standard_full_mode::StandardFullMode;

fn main() -> JoyConResult<()> {
    // use joycon_rs::joycon::simple_hid_mode::SimpleHidMode;
    // use joycon_rs::joycon::standard_input_report::StandardInputReportMode;
    //
    let (send, receive) =
        std::sync::mpsc::channel();
    //
    let manager = JoyConManager::new()?;
    // // println!("{:?}", &manager.connected_joycon_devices);
    let threads = manager.connected_joycon_devices.into_iter()
        .flat_map(|j| SimpleJoyConDriver::new(j))
        .map(|mut driver| {
            let sender = send.clone();
            std::thread::spawn(move || {
                // enable 6-axis sensor
                // let simple_hid_mode = SimpleHIDMode::set(driver).unwrap();
                let standard_full_mode = StandardFullMode::set(driver).unwrap();
                loop {
                    if let Ok(update) = standard_full_mode.read_input_report() {
                        sender.send(update);
                    }
                }
            });
        })
        .collect::<Vec<_>>();

    if threads.is_empty() {
        println!("No Joy-Cons.");
        Ok(())
    } else {
        loop {
            dbg!(receive.recv());
        }
    }
}