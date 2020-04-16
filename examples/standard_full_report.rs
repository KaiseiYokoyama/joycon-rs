use joycon_rs::prelude::*;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!

    let (tx, rx) = std::sync::mpsc::channel();

    let threads = JoyConManager::new()?
        .connected_joycon_devices
        .into_iter()
        .flat_map(|d| SimpleJoyConDriver::new(d))
        .flat_map::<JoyConResult<_>,_>(|driver| {
            let standard_full_mode = StandardFullMode::new(driver)?;
            let tx = tx.clone();

            Ok(std::thread::spawn( move || {
                loop {
                    tx.send(standard_full_mode.read_input_report()).unwrap();
                }
            }))
        })
        .collect::<Vec<_>>();

    if threads.is_empty() {
        println!("No Joy-Cons");
    } else {
        // Stop with `Cmd + C` or `Ctrl + C`
        while let Ok(standard_full_report) = rx.recv() {
            println!("{:?}", standard_full_report);
        }
    }

    Ok(())
}