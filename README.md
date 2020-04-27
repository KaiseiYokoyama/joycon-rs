![joycon-rs](images/joycon-rs.png)

# Joycon-rs
![Test on mac](https://github.com/KaiseiYokoyama/joycon-rs/workflows/Test%20on%20mac/badge.svg?branch=master)
![Test on windows](https://github.com/KaiseiYokoyama/joycon-rs/workflows/Test%20on%20windows/badge.svg)
![Test on ubuntu](https://github.com/KaiseiYokoyama/joycon-rs/workflows/Test%20on%20ubuntu/badge.svg)

A framework for dealing with Nintendo Switch Joy-Con on Rust easily and efficiently.

`Joycon-rs` provides utility to find communicate with, and operate Joy-Con. 
Please see the documentation comments for detailed instructions on how to use it.

 Joycon-rs is in development and is still incomplete.
 Please be aware the update will include breaking changes for the time being. Pardon out dust!

# Setup
On macOS or Windows, there are no preparation.

On linux, 
```bash
$ sudo apt-get install libudev-dev libusb-1.0-0-dev libfox-1.6-dev
```

# Usage
First, add dependency to `Cargo.toml`

```toml
[dependencies]
joycon_rs = "*"
```

Then, `use` prelude on `.rs` file.
```rust
use joycon_rs::prelude::*;
```

Perfect! Now you have Joycon-rs available in code.

### Receive reports
For starters, let's take a simple signal from JoyCon.
If you use more than one JoyCon, [`mspc`] can be very helpful.

```rust no_run
use joycon_rs::prelude::*;

let (tx, rx) = std::sync::mpsc::channel();

let _output = std::thread::spawn(move || {
    // Push buttons or tilt the stick please.
    // Stop with `Cmd + C` or `Ctrl + C`
    while let Ok(message) = rx.recv() {
        dbg!(message);
    }
});

let manager = JoyConManager::new().unwrap();
let (managed_devices, new_devices) = {
let lock = manager.lock();
    match lock {
        Ok(m) => (m.managed_devices(),m.new_devices()),
        Err(_) => unreachable!()
    }
};

managed_devices.into_iter()
    .chain(new_devices)
    .flat_map(|dev| SimpleJoyConDriver::new(&dev))
    .try_for_each::<_, JoyConResult<()>>(|driver| {
        // Change JoyCon to Simple hid mode.
        let simple_hid_mode = SimpleHIDMode::new(driver)?;
    
        let tx = tx.clone();
    
        // Spawn thread
        std::thread::spawn( move || {
            loop {
                // Forward the report to the main thread
                tx.send(simple_hid_mode.read_input_report()).unwrap();
            }
        });
    
        Ok(())
    })
    .unwrap();
```

### Set player lights
Then, lets deal with player lights.

```rust no_run
use joycon_rs::prelude::{*, lights::*};

let (tx, rx) = std::sync::mpsc::channel();

let _output = std::thread::spawn(move || {
    // Stop with `Cmd + C` or `Ctrl + C`
    while let Ok(message) = rx.recv() {
        dbg!(message);
    }
});

let manager = JoyConManager::new().unwrap();
let (managed_devices, new_devices) = {
let lock = manager.lock();
    match lock {
        Ok(m) => (m.managed_devices(),m.new_devices()),
        Err(_) => unreachable!()
    }
};

managed_devices.into_iter()
    .chain(new_devices)
    .flat_map(|dev| SimpleJoyConDriver::new(&dev))
    .try_for_each::<_, JoyConResult<()>>(|mut driver| {
        // Set player lights
        // [SL BUTTON] 📸💡📸💡 [SR BUTTON]
        driver.set_player_lights(&vec![LightUp::LED1, LightUp::LED3], &vec![Flash::LED0, Flash::LED2]).unwrap();
        tx.send(driver.get_player_lights()).unwrap();
        Ok(())
    })
    .unwrap();
```

### Rumble
```rust no_run
use joycon_rs::prelude::*;
use std::convert::TryInto;
use std::ops::Deref;
use joycon_rs::joycon::joycon_features::JoyConFeature;

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!

    let manager = JoyConManager::new()?;
    let (managed_devices, new_devices) = {
        let lock = manager.lock();
        match lock {
            Ok(manager) =>
                (manager.managed_devices(), manager.new_devices()),
            Err(_) => unreachable!(),
        }
    };

    managed_devices.into_iter()
        .chain(new_devices)
        .inspect(|d| {
            let lock = d.lock();
            let device = match lock {
                Ok(device) => device,
                Err(e) => e.into_inner(),
            };
            let hid_device: JoyConResult<&HidDevice> = device.deref().try_into();
            if let Ok(hid_device) = hid_device {
                println!("{:?}", hid_device.get_product_string())
            }
        })
        .try_for_each::<_, JoyConResult<()>>(|d| {
            let mut driver = SimpleJoyConDriver::new(&d)?;

            driver.enable_feature(JoyConFeature::Vibration)?;

            // let rumble = Rumble::new(80.0,0.2);
            let rumble = Rumble::new(300.0,0.9);
            driver.rumble((Some(rumble), Some(rumble)))?;

            Ok(())
        })?;

    Ok(())
}
```

### More Examples
[Here](examples).

 # Features
 You can use `Joycon-rs` for...
 - [Manage Joy-Cons](examples/scan_for_joycons.rs)
     - Connection / Disconnection / Reconnection
 - Send / Receive raw packets (u8 array) to / from Joy-Con
 - Receive input to Joy-Con
     - [Receive pushed buttons, and stick directions (one of 8 directions) on every button pressed.](examples/simple_hid_report.rs)
     - [Receive pushed buttons, stick directions (analog value), and 6-Axis sensor at 60Hz.](examples/standard_full_report.rs)
     - Get/Set status of Joy-Con
 - [Deal with LED (Player lights)](examples/player_lights.rs)
 - [Vibration (Rumble)](examples/rumble.rs)
 - [HOME Light](examples/home_light.rs)

 ## Planning
 - Receive NFC/IR data
 - Deal with Pro Controller
 
[`mspc`]: https://doc.rust-lang.org/book/ch16-02-message-passing.html
