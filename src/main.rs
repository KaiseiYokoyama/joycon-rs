use joycon_rs::prelude::*;
use joycon_rs::joycon::{JoyConManager, SimpleJoyConDriver};
// use joycon_rs::joycon::input_report_mode::simple_hid_mode::SimpleHIDMode;
// use joycon_rs::joycon::input_report_mode::standard_full_mode::StandardFullMode;
use joycon_rs::joycon::lights::*;

fn main() -> JoyConResult<()> {
    // use joycon_rs::joycon::simple_hid_mode::SimpleHidMode;
    // use joycon_rs::joycon::input_report_mode::standard_full_mode::StandardFullMode;
    //
    let (send, receive) =
        std::sync::mpsc::channel();
    //
    let manager = JoyConManager::new()?;
    // // println!("{:?}", &manager.connected_joycon_devices);
    let threads = manager.lock()
        .unwrap()
        .connected_devices()
        .iter()
        .flat_map(|j| SimpleJoyConDriver::new(j))
        .enumerate()
        .map(|(_idx, mut driver)| {
            let sender = send.clone();
            std::thread::spawn(move || {
                driver.set_player_lights(&vec![], &vec![]).unwrap();
                driver.set_player_lights(&vec![LightUp::LED1], &vec![Flash::LED1]).unwrap();
                sender.send(driver.get_player_lights()).unwrap();
            });
        })
        .collect::<Vec<_>>();

    if threads.is_empty() {
        println!("No Joy-Cons.");
    } else {
        let _ = std::iter::repeat(())
            .take(20)
            .inspect(|_| { dbg!(receive.recv().unwrap().unwrap()); })
            .collect::<Vec<_>>();
    }

    Ok(())
}