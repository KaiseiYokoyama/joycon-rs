use super::*;
use crate::joycon::{
    driver::device_info::{JoyConDeviceInfo, JoyConMacAddress},
    input_report_mode::sub_command_mode::SubCommandReplyData,
};

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::thread::JoinHandle;
use std::option::Option::Some;

/// A manager for dealing with Joy-Cons.
///
/// JoyConManager has a scanner that detects new connections/disconnections/reconnections
/// of JoyCon by scanning periodically
/// (every second: You can set the interval in [`JoyConManager::with_duration()`]).
///
/// [`JoyConManager::with_duration()`]: #method.with_duration
pub struct JoyConManager {
    devices: HashMap<JoyConMacAddress, Arc<Mutex<JoyConDevice>>>,
    scanner: Option<JoinHandle<()>>,
    scan_interval: Duration,
    new_device_receiver: Receiver<Arc<Mutex<JoyConDevice>>>,
}

impl JoyConManager {
    /// Constructor
    pub fn new() -> JoyConResult<Arc<Mutex<Self>>> {
        Self::with_duration(std::time::Duration::from_millis(100))
    }

    pub fn with_duration(duration: Duration) -> JoyConResult<Arc<Mutex<Self>>> {
        let scanner = None;
        let scan_cycle = duration;
        let (tx, rx) = std::sync::mpsc::channel();

        let manager = {
            let mut manager = JoyConManager {
                devices: HashMap::new(),
                scanner,
                scan_interval: scan_cycle,
                new_device_receiver: rx,
            };

            // First scan
            manager.scan()?;

            Arc::new(Mutex::new(manager))
        };

        let scanner = {
            let manager = Arc::downgrade(&manager);

            std::thread::spawn(move || {
                while let Some(manager) = manager.upgrade() {
                    // Get manager
                    let mut manager = match manager.lock() {
                        Ok(m) => m,
                        Err(m) => m.into_inner(),
                    };

                    // Send new devices
                    if let Ok(new_devices) = manager.scan() {
                        // If mspc channel is disconnected, end this thread.
                        let send_result = new_devices.into_iter()
                            .try_for_each::<_, Result<(), std::sync::mpsc::SendError<_>>>(|new_device| {
                                tx.send(new_device)
                            });
                        if send_result.is_err() {
                            return ();
                        }
                    }

                    // Sleep
                    std::thread::sleep(manager.scan_interval)
                }

                return ();
            })
        };

        // Set scanner
        if let Ok(mut manager) = manager.lock() {
            manager.scanner = Some(scanner);
        }

        Ok(manager)
    }

    /// Scan the JoyCon connected to your computer.
    /// This returns new Joy-Cons.
    pub fn scan(&mut self) -> JoyConResult<Vec<Arc<Mutex<JoyConDevice>>>> {
        let hidapi = HidApi::new()?;
        let mut devices = hidapi.device_list()
            .flat_map(|di| JoyConDevice::new(di, &hidapi))
            .map(|device| Arc::new(Mutex::new(device)))
            .flat_map::<JoyConResult<(JoyConMacAddress, Arc<Mutex<JoyConDevice>>)>, _>(|device| {
                let mut driver = SimpleJoyConDriver::new(&device)?;
                let mac_address = JoyConDeviceInfo::once(&mut driver)?
                    .extra
                    .reply
                    .mac_address;
                Ok((mac_address, device))
            })
            .collect::<HashMap<_, _>>();
        let old_devices = &self.devices;

        let keys = devices.keys().cloned().collect::<HashSet<_>>();
        let old_keys = old_devices.keys().cloned().collect::<HashSet<_>>();

        let removed_keys = old_keys.difference(&keys);
        removed_keys.for_each(|key| {
            self.devices
                .get(&key)
                .map(|device| {
                    let mut device = match device.lock() {
                        Ok(d) => d,
                        Err(e) => e.into_inner()
                    };

                    *device = JoyConDevice::Disconnected;
                });
        });

        let may_reconnected_keys = old_keys.intersection(&keys);
        may_reconnected_keys
            .filter(|k| {
                if let Some(device) = old_devices.get(k) {
                    !match device.lock() {
                        Ok(d) => d,
                        Err(d) => d.into_inner(),
                    }.is_connected()
                } else {
                    unreachable!()
                }
            })
            .for_each(|k| {
                if let (Some(device), Some(new_device)) = (self.devices.get(k), devices.remove(k)) {
                    let mut device = match device.lock() {
                        Ok(d) => d,
                        Err(e) => e.into_inner()
                    };

                    let new_device = {
                        if let Ok(new_device) = Arc::try_unwrap(new_device) {
                            match new_device.into_inner() {
                                Ok(d) => d,
                                Err(e) => e.into_inner()
                            }
                        } else { unreachable!() }
                    };

                    *device = new_device;
                } else { unreachable!() }
            });

        let mut new_devices = Vec::new();
        let added_keys = keys.difference(&old_keys);
        added_keys.for_each(|key| {
            if let Some(new_device) = devices.remove(key) {
                let device_cloned = Arc::clone(&new_device);
                new_devices.push(device_cloned);

                self.devices.insert(key.clone(), new_device);
            }
        });

        Ok(new_devices)
    }

    /// Collection of managed JoyCons.
    /// It includes disconnected devices.
    pub fn managed_devices(&self) -> Vec<Arc<Mutex<JoyConDevice>>> {
        self.devices.values()
            .into_iter()
            .map(|d| Arc::clone(d))
            .collect()
    }

    /// Collection of managed and connected JoyCons.
    pub fn connected_devices(&self) -> Vec<Arc<Mutex<JoyConDevice>>> {
        self.devices.values()
            .into_iter()
            .filter(|&d| {
                let device = match d.lock() {
                    Ok(d) => d,
                    Err(d) => d.into_inner(),
                };
                device.is_connected()
            })
            .map(|d| Arc::clone(d))
            .collect()
    }

    /// Receiver of new devices.
    /// The receiver keeps a record *from the construction of the manager* to the present.
    /// For more information, please refer to [the official Rust documentation].
    ///
    /// # Example
    /// ```no_run
    /// use joycon_rs::prelude::*;
    ///
    /// let (tx, rx) = std::sync::mpsc::channel();
    /// let output = std::thread::spawn( move || {
    ///     while let Ok(update) = rx.recv() {
    ///         dbg!(update);
    ///     }
    /// });
    ///
    /// let manager = JoyConManager::new().unwrap();
    /// let joycons = manager.lock().unwrap()
    ///     .new_device_receiver()
    ///     .iter()
    ///     .flat_map(|d| SimpleJoyConDriver::new(&d))
    ///     .flat_map::<JoyConResult<_>,_>(|driver| {
    ///         let simple_hid_mode = SimpleHIDMode::new(driver)?;
    ///         let tx = tx.clone();
    ///
    ///         let thread = std::thread::spawn( move || loop {
    ///                 if tx.send(simple_hid_mode.read_input_report()).is_err() {
    ///                     return;
    ///                 }
    ///             });
    ///
    ///         Ok(thread)
    ///     })
    ///     .collect::<Vec<_>>();
    ///
    /// output.join();
    ///
    /// ```
    ///
    /// [the official Rust documentation]: https://ykomatsu.github.io/rust/std/sync/mpsc/index.html
    pub fn new_device_receiver(&self) -> &Receiver<Arc<Mutex<JoyConDevice>>> {
        &self.new_device_receiver
    }
}

