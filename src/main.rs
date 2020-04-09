use joycon_rs::prelude::*;
use joycon_rs::joycon::{JoyConManager, SimpleJoyConDriver, Rotation, JoyConDriver, SubCommand, input_report_mode::*};
// use joycon_rs::joycon::input_report_mode::simple_hid_mode::SimpleHIDMode;
// use joycon_rs::joycon::input_report_mode::standard_full_mode::StandardFullMode;
use joycon_rs::joycon::lights::*;

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
        .enumerate()
        .map(|(idx, mut driver)| {
            let sender = send.clone();
            std::thread::spawn(move || {
                // driver.set_lights(&vec![SimpleJoyConDriver::LIGHT_UP[idx % SimpleJoyConDriver::LIGHT_UP.len()]], &vec![]).unwrap();
                driver.set_lights(&vec![], &vec![SimpleJoyConDriver::FLASH[0]]).unwrap();
                let mode = driver.light_report_mode().unwrap();
                loop {
                    // every seconds, get lights status
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    sender.send(mode.read_input_report());
                }
                // enable 6-axis sensor
                // let simple_hid_mode = SimpleHIDMode::set(driver).unwrap();
                // let standard_full_mode = StandardFullMode::set(driver).unwrap();
                // loop {
                //     if let Ok(update) = standard_full_mode.read_input_report() {
                //         sender.send(update);
                //     }
                // }
            });
        })
        .collect::<Vec<_>>();

    if threads.is_empty() {
        println!("No Joy-Cons.");
    } else {
        loop {
            dbg!(receive.recv());
        }
    }

    Ok(())
}