use super::*;

/// The controller user uses to play with.
/// If you're not happy with this implementation, you can use `JoyConDriver` trait.
///
/// # Examples
/// ```no_run
/// use joycon_rs::prelude::{JoyConManager, SimpleJoyConDriver, lights::*};
/// use joycon_rs::result::JoyConResult;
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
///     .try_for_each::<_,JoyConResult<()>>(|device| {
///         let mut driver = SimpleJoyConDriver::new(&device)?;
///
///         // set player's lights
///         driver.set_player_lights(&vec![SimpleJoyConDriver::LIGHT_UP[0]], &vec![])?;
///
///         Ok(())
///     })
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct SimpleJoyConDriver {
    /// The controller user uses
    pub joycon: Arc<Mutex<JoyConDevice>>,
    /// rotation of controller
    pub rotation: Rotation,
    rumble: (Option<Rumble>, Option<Rumble>),
    enabled_features: HashSet<JoyConFeature>,
    valid_reply: bool,
    /// Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
    global_packet_number: GlobalPacketNumber,
}

impl SimpleJoyConDriver {
    /// Constructs a new `SimpleJoyConDriver`.
    pub fn new(joycon: &Arc<Mutex<JoyConDevice>>) -> JoyConResult<Self> {
        // joycon.set_blocking_mode(true);
        // joycon.set_blocking_mode(false);

        let mut enabled_features = HashSet::new();
        enabled_features.insert(JoyConFeature::IMUFeature(IMUConfig::default()));
        enabled_features.insert(JoyConFeature::Vibration);

        let mut driver = Self {
            joycon: Arc::clone(joycon),
            rotation: Rotation::Portrait,
            rumble: (None, None),
            enabled_features,
            valid_reply: {
                let device = match joycon.lock() {
                    Ok(j) => j,
                    Err(e) => e.into_inner(),
                }
                .device_type();
                match device {
                    JoyConDeviceType::ProCon => false,
                    _ => true,
                }
            },
            global_packet_number: GlobalPacketNumber::default(),
        };

        let check_reply = {
            let device = match joycon.lock() {
                Ok(d) => d,
                Err(e) => e.into_inner(),
            };
            match device.device_type() {
                JoyConDeviceType::ProCon => false,
                _ => true,
            }
        };

        if check_reply {
            driver.reset()?;
        } else {
            let _ = driver.reset();
        }

        Ok(driver)
    }

    pub fn joycon(&self) -> MutexGuard<JoyConDevice> {
        // todo error handling
        match self.joycon.lock() {
            Ok(joycon) => joycon,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

impl JoyConDriver for SimpleJoyConDriver {
    fn valid_reply(&self) -> bool {
        self.valid_reply
    }

    fn set_valid_reply(&mut self, valid: bool) {
        self.valid_reply = valid;
    }

    fn write(&self, data: &[u8]) -> JoyConResult<usize> {
        let joycon = self.joycon();
        Ok(joycon.write(data)?)
    }

    fn read(&self, buf: &mut [u8]) -> JoyConResult<usize> {
        Ok(self.joycon().read(buf)?)
    }

    fn read_timeout(&self, buf: &mut [u8], timeout: i32) -> JoyConResult<usize> {
        Ok(self.joycon().read_timeout(buf, timeout)?)
    }

    fn global_packet_number(&self) -> u8 {
        self.global_packet_number.into()
    }

    fn increase_global_packet_number(&mut self) {
        self.global_packet_number = self.global_packet_number.next();
    }

    fn set_rumble_status(&mut self, rumble_l_r: (Option<Rumble>, Option<Rumble>)) {
        self.rumble = rumble_l_r;
    }

    fn get_rumble_status(&self) -> (Option<Rumble>, Option<Rumble>) {
        self.rumble
    }

    fn enable_feature(&mut self, feature: JoyConFeature) -> JoyConResult<()> {
        match feature {
            JoyConFeature::IMUFeature(feature) => {
                let data: [u8; 4] = feature.into();
                // enable IMU
                self.send_sub_command(SubCommand::EnableIMU, &[0x01])?;
                // set config
                self.send_sub_command(SubCommand::SetIMUSensitivity, &data)?;
            }
            JoyConFeature::Vibration => {
                // enable vibration
                self.send_sub_command(SubCommand::EnableVibration, &[0x01])?;
            }
        }

        self.enabled_features.insert(feature);

        Ok(())
    }

    fn enabled_features(&self) -> &HashSet<JoyConFeature> {
        &self.enabled_features
    }

    fn devices(&self) -> Vec<Arc<Mutex<JoyConDevice>>> {
        vec![Arc::clone(&self.joycon)]
    }
}
