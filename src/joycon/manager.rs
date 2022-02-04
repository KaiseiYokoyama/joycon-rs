use super::*;

use std::collections::{HashMap, HashSet};
use std::option::Option::Some;
use std::sync::{Mutex, Once};
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JoyConSerialNumber(pub String);

/// A manager for dealing with Joy-Cons.
///
/// JoyConManager has a scanner that detects new connections/disconnections/reconnections
/// of JoyCon by scanning periodically
/// (every second: You can set the interval in [`JoyConManager::with_duration()`]).
///
/// You can get instance at [`JoyConManager::get_instance()`].
///
/// [`JoyConManager::with_duration()`]: #method.with_duration
/// [`JoyConManager::get_instance()`]: #method.get_instance
pub struct JoyConManager {
    devices: HashMap<JoyConSerialNumber, Arc<Mutex<JoyConDevice>>>,
    hid_api: Option<HidApi>,
    scanner: Option<JoinHandle<()>>,
    scan_interval: Duration,
    new_devices: crossbeam_channel::Receiver<Arc<Mutex<JoyConDevice>>>,
}

impl JoyConManager {
    /// Get `JoyConManager` instance.
    pub fn get_instance() -> Arc<Mutex<Self>> {
        static mut SINGLETON: Option<Arc<Mutex<JoyConManager>>> = None;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let instance = JoyConManager::new().unwrap();

                SINGLETON = Some(instance);
            });

            match SINGLETON.clone() {
                Some(manager) => manager,
                None => unreachable!(),
            }
        }
    }

    /// Constructor
    fn new() -> JoyConResult<Arc<Mutex<Self>>> {
        Self::with_interval(std::time::Duration::from_millis(100))
    }

    fn with_interval(interval: Duration) -> JoyConResult<Arc<Mutex<Self>>> {
        let (tx, rx) = crossbeam_channel::unbounded();
        // crossbeam_channel::bounded(0);

        let manager = {
            let mut manager = JoyConManager {
                devices: HashMap::new(),
                hid_api: None,
                scanner: None,
                scan_interval: interval,
                new_devices: rx,
            };

            // First scan
            manager.scan()?.into_iter().for_each(|new_device| {
                let _ = tx.send(new_device);
            });

            Arc::new(Mutex::new(manager))
        };

        let scanner = {
            let manager = Arc::downgrade(&manager);

            std::thread::spawn(move || {
                while let Some(manager) = manager.upgrade() {
                    let interval = {
                        // Get manager
                        let mut manager = match manager.lock() {
                            Ok(m) => m,
                            Err(m) => m.into_inner(),
                        };

                        // Send new devices
                        if let Ok(new_devices) = manager.scan() {
                            // If mpsc channel is disconnected, end this thread.
                            let send_result = new_devices.into_iter().try_for_each::<_, Result<
                                (),
                                crossbeam_channel::SendError<_>,
                            >>(
                                |new_device| tx.send(new_device),
                            );
                            if send_result.is_err() {
                                return;
                            }
                        }

                        manager.scan_interval
                    };

                    // Sleep
                    std::thread::sleep(interval)
                }
            })
        };

        // Set scanner
        if let Ok(mut manager) = manager.lock() {
            manager.scanner = Some(scanner);
        }

        Ok(manager)
    }

    /// Set scan interval
    pub fn set_interval(&mut self, interval: Duration) {
        self.scan_interval = interval;
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

        let detected_devices = hid_api
            .device_list()
            .filter(|&device_info| JoyConDevice::check_type_of_device(device_info).is_ok())
            .filter_map(|device_info| {
                device_info
                    .serial_number()
                    .map(|s| s.to_string())
                    .map(JoyConSerialNumber)
                    .map(|serial| (device_info, serial))
            })
            .collect::<Vec<_>>();

        // removed devices
        for removed_serial in self
            .devices
            .keys()
            .cloned()
            .collect::<HashSet<_>>()
            .difference(&detected_devices.iter().map(|(_, s)| s.clone()).collect())
        {
            if let Some(device) = self.devices.get(removed_serial) {
                let mut device = match device.lock() {
                    Ok(d) => d,
                    Err(e) => e.into_inner(),
                };
                device.forget_device();
            }
        }

        let mut new_devices = Vec::new();
        for (device_info, serial) in detected_devices {
            if self.devices.contains_key(&serial) {
                let mut previous_device = match self.devices.get(&serial).unwrap().lock() {
                    Ok(d) => d,
                    Err(d) => d.into_inner(),
                };
                if !previous_device.is_connected() {
                    // Reconnected device
                    if let Ok(device) = JoyConDevice::new(device_info, hid_api) {
                        *previous_device = device;
                    }
                }
            } else {
                // New device
                if let Ok(device) = JoyConDevice::new(device_info, hid_api) {
                    let device = Arc::new(Mutex::new(device));
                    let cloned_device = Arc::clone(&device);
                    new_devices.push(cloned_device);
                    self.devices.insert(serial.clone(), device);
                }
            }
        }

        Ok(new_devices)
    }

    /// Collection of managed JoyCons.
    /// It may contains disconnected ones.
    pub fn managed_devices(&self) -> Vec<Arc<Mutex<JoyConDevice>>> {
        self.devices.values().map(Arc::clone).collect()
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
    /// let manager = JoyConManager::get_instance();
    ///
    /// let devices = {
    ///     let lock = manager.lock();
    ///     match lock {
    ///         Ok(manager) => manager.new_devices(),
    ///         Err(_) => return,
    ///     }
    /// };
    ///
    /// devices.iter()
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

lazy_static! {
    pub static ref JOYCON_RECEIVER: crossbeam_channel::Receiver<Arc<Mutex<JoyConDevice>>> = {
        let manager = JoyConManager::get_instance();
        let manager = match manager.lock() {
            Ok(manager) => manager,
            Err(e) => e.into_inner(),
        };
        manager.new_devices()
    };
}
