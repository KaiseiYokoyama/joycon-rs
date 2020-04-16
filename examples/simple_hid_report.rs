use joycon_rs::prelude::*;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!

    let (tx, rx) = std::sync::mpsc::channel();

    let threads = JoyConManager::new()?
        .connected_joycon_devices
        .into_iter()
        .flat_map(|d| SimpleJoyConDriver::new(d))
        .flat_map::<JoyConResult<_>,_>(|driver| {
            let simple_hid_mode =SimpleHIDMode::new(driver)?;
            let tx = tx.clone();

            Ok(std::thread::spawn( move || {
                loop {
                    tx.send(simple_hid_mode.read_input_report()).unwrap();
                }
            }))
        })
        .collect::<Vec<_>>();

    if threads.is_empty() {
        println!("No Joy-Cons");
    } else {
        // Push buttons or tilt the stick please.
        // Stop with `Cmd + C` or `Ctrl + C`
        while let Ok(simple_hid_report) = rx.recv() {
            println!("{:?}", simple_hid_report);
        }
    }

    Ok(())
}