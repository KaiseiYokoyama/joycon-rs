use super::*;

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::Duration;
use std::thread::JoinHandle;
use std::option::Option::Some;
use crate::joycon::device::is_joycon;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JoyConSerialNumber(pub String);

/// A manager for dealing with Joy-Cons.
///
/// JoyConManager has a scanner that detects new connections/disconnections/reconnections
/// of JoyCon by scanning periodically
/// (every second: You can set the interval in [`JoyConManager::with_duration()`]).
///
/// [`JoyConManager::with_duration()`]: #method.with_duration
pub struct JoyConManager {
    devices: HashMap<JoyConSerialNumber, Arc<Mutex<JoyConDevice>>>,
    hid_api: Option<HidApi>,
    scanner: Option<JoinHandle<()>>,
    scan_interval: Duration,
    new_devices: crossbeam_channel::Receiver<Arc<Mutex<JoyConDevice>>>,
}

impl JoyConManager {
    /// Constructor
    pub fn new() -> JoyConResult<Arc<Mutex<Self>>> {
        Self::with_interval(std::time::Duration::from_millis(100))
    }

    pub fn with_interval(interval: Duration) -> JoyConResult<Arc<Mutex<Self>>> {
        let (tx, rx) = crossbeam_channel::bounded(0);

        let manager = {
            let mut manager = JoyConManager {
                devices: HashMap::new(),
                hid_api: None,
                scanner: None,
                scan_interval: interval,
                new_devices: rx,
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
                            .try_for_each::<_, Result<(), crossbeam_channel::SendError<_>>>(|new_device| {
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
        let hid_api = if let Some(hidapi) = &mut self.hid_api {
            // refresh
            hidapi.refresh_devices()?;
            hidapi
        } else {
            // initialize
            self.hid_api = Some(HidApi::new()?);
            match &mut self.hid_api {
                Some(hid_api) => hid_api,
                None => unreachable!(),
            }
        };

        let previous_device_serials = self.devices.keys()
            .cloned()
            .collect::<HashSet<_>>();

        let detected_device_serials = hid_api.device_list()
            .filter(|&device_info| is_joycon(device_info).is_ok())
            .flat_map(|device_info|
                device_info.serial_number()
                    .map(|s| s.to_string())
                    .map(JoyConSerialNumber)
            )
            .collect::<HashSet<_>>();

        let mut detected_devices = hid_api.device_list()
            .filter(|&device_info| is_joycon(device_info).is_ok())
            .flat_map(|di| {
                let serial_number = di.serial_number()
                    .map(|s| s.to_string())
                    .map(JoyConSerialNumber)?;
                let device = JoyConDevice::new(di, hid_api).ok()?;
                Some((serial_number, device))
            })
            .map(|(serial, device)| (serial,Arc::new(Mutex::new(device))))
            .collect::<HashMap<_,_>>();

        // removed
        {
            let removed_keys = previous_device_serials.difference(&detected_device_serials);
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
        }

        // reconnected
        {
            let reconnected_keys = previous_device_serials.intersection(&detected_device_serials)
                .into_iter()
                .filter(|&k| {
                    if let Some(device) = self.devices.get(k) {
                        !match device.lock() {
                            Ok(d) => d,
                            Err(d) => d.into_inner(),
                        }.is_connected()
                    } else {
                        unreachable!()
                    }
                }).collect::<HashSet<_>>();
            reconnected_keys.iter()
                .for_each(|&k| {
                    if let (Some(device), Some(new_device)) = (self.devices.get(k), detected_devices.remove(k)) {
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
        }

        let mut new_devices = Vec::new();
        // Connected
        {
            let connected_keys = detected_device_serials.difference(&previous_device_serials);
            connected_keys.for_each(|key| {
                if let Some(device) = detected_devices.remove(key) {
                    let device_cloned = Arc::clone(&device);
                    new_devices.push(device_cloned);

                    self.devices.insert(key.clone(), device);
                }
            });
        }

        Ok(new_devices)
    }

    /// Collection of managed JoyCons.
    /// It may contains disconnected ones.
    pub fn managed_devices(&self) -> Vec<Arc<Mutex<JoyConDevice>>> {
        self.devices.values()
            .into_iter()
            .map(|d| Arc::clone(d))
            .collect()
    }

    /// Receiver of new devices.
    /// This method provides receiver of **mpmc** channel.
    /// Since the channel has no capacity,
    /// the message will disappear if it is not received at the same time as it is sent.
    ///
    /// # Example
    /// ```no_run
    /// use joycon_rs::prelude::*;
    ///
    /// let (tx, rx) = std::sync::mpsc::channel();
    /// let _output = std::thread::spawn( move || {
    ///     while let Ok(update) = rx.recv() {
    ///         dbg!(update);
    ///     }
    /// });
    ///
    /// let manager = JoyConManager::new().unwrap();
    ///
    /// let (managed_devices, new_devices) = {
    ///     let lock = manager.lock();
    ///     match lock {
    ///         Ok(manager) => (manager.managed_devices(), manager.new_devices()),
    ///         Err(_) => return,
    ///     }
    /// };
    ///
    /// managed_devices.into_iter()
    ///     .chain(new_devices)
    ///     .flat_map(|device| SimpleJoyConDriver::new(&device))
    ///     .try_for_each::<_, JoyConResult<()>>(|driver| {
    ///         let simple_hid_mode = SimpleHIDMode::new(driver)?;
    ///         let tx = tx.clone();
    ///
    ///         let thread = std::thread::spawn(move || {
    ///             loop {
    ///                 tx.send(simple_hid_mode.read_input_report());
    ///             }
    ///         });
    ///
    ///         Ok(())
    ///     });
    /// ```
    pub fn new_devices(&self) -> crossbeam_channel::Receiver<Arc<Mutex<JoyConDevice>>> {
        self.new_devices.clone()
    }
}

