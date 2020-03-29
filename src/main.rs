use joycon_rs::prelude::*;
use joycon_rs::joycon::{JoyConManager, simple_hid_mode::SimpleHidMode, SimpleJoyConDriver};
use joycon_rs::joycon::simple_hid_mode::SimpleHidUpdate;

fn main() -> JoyConResult<()> {
    let (send, receive) =
        // std::sync::mpsc::channel::<SimpleHidUpdate>();
        std::sync::mpsc::channel::<SimpleHidUpdate>();

    let manager = JoyConManager::new()?;
    // println!("{:?}", &manager.connected_joycon_devices);
    let _threads = manager.connected_joycon_devices.into_iter()
        .inspect(|jd| { dbg!(jd);} )
        .map(|j| SimpleJoyConDriver::new(j))
        .map(|driver| {
            let sender = send.clone();
            std::thread::spawn(move || {
                loop {
                    if let Ok(update) = driver.read_update() {
                        sender.send(update);
                    }
                }
            });
        })
        .collect::<Vec<_>>();

    loop {
        dbg!(receive.recv());
    }

    Ok(())
}