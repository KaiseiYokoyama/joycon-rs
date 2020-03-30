use joycon_rs::prelude::*;
use joycon_rs::joycon::{JoyConManager, SimpleJoyConDriver, Rotation, JoyConDriver, SubCommand};

fn main() -> JoyConResult<()> {
    // use joycon_rs::joycon::simple_hid_mode::SimpleHidMode;
    use joycon_rs::joycon::standard_input_report::StandardInputReportMode;

    let (send, receive) =
        // std::sync::mpsc::channel::<SimpleHidUpdate>();
        std::sync::mpsc::channel();

    let manager = JoyConManager::new()?;
    // println!("{:?}", &manager.connected_joycon_devices);
    let threads = manager.connected_joycon_devices.into_iter()
        .map(|j| SimpleJoyConDriver::new(j))
        .map(|mut driver| {
            driver.rotation = Rotation::Landscape;
            let sender = send.clone();
            std::thread::spawn(move || {
                // enable 6-axis sensor
                let _ = driver.send_sub_command(SubCommand::EnableIMU, &[0x01]);
                let _ = driver.set_standard_input_report_mode();
                loop {
                    if let Ok(update) = driver.read_update() {
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