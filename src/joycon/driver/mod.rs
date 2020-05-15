use super::*;
use std::convert::TryFrom;
use std::collections::HashSet;
pub use rumble::Rumble;
pub use global_packet_number::GlobalPacketNumber;
pub use joycon_features::{JoyConFeature, IMUConfig};
pub use simple_joycon_driver::SimpleJoyConDriver;
use std::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    ///    .-..WHHHHHWa-.
    ///     zrrrrrrrrrvvzzVHJ.
    ///   (gkrrrrrrrrrtrrrrrzWn.
    ///   dMKrQWNmrrtrrtrtrrrvXH,
    ///   WMRtZH#VrtrtrrrtrrtrrzW-
    ///   WMRttrrtrrwQmmyrrtrrrru.
    ///   WMRtrtttrrMHWWMkrrtrrrwl
    ///   WMRrtrrrrrTMMMBvrrrrrrrI
    ///   JMSrOdMMMmrrrrrQHMMHwrrI
    ///   dMktdHHd@#rtrrrWHWdMSrwI
    ///   dMSrrdHMBwrAQArwHMM8rrrI
    ///   dMSrrrrrrrHHWHNwrrrrtrrI
    ///   WM0ttrtttrMHgH#wrrtrrrrI
    ///   HM0rrtrtrrrw0wrrtrttrrrI
    ///   HM0rrrtrrtrrrrrtrrrrtrvI
    ///   HM0rtrrrrrvzzzvvvrrrrtvr
    ///  .HMkrrtrrvQgHNHmyzvvrrrwr
    ///  .HMZrtrrQMMHHHH##Nkwrrrvr
    ///  .@NZrrrwMMHHHHHHHH#Zvrrvr
    ///  .@NZrrrdMHHHHHHHH##uwrrvr
    ///  .@NwrrrrUMHHHHHH#MSXrrrvr
    ///  .@NwrtrrrXHMMMMM8Xwrrtrvr
    ///   dNwrtrrrrrvrrrrrrrrrrrw)
    ///   dNrrrtrrrrrrrrrrrrrrrrw%
    ///   dMrrrrAWHHkrrrrrrrrrrrw)
    ///  .WMrrrwWMMMNkrrrrrrrrrrw)
    ///  .@MrrrrVMMM9rrrrrrtrrrrz)
    ///  .HMrrrrrrrrrrrrrtrrrrrrw)
    ///  .@Mrrrrrtrrrrtrrrtrrrrrw}
    ///  ,HMrrrrrrrrtrrrrrrrrrrrX
    ///  ,HWrrrrrrrrrrrtrrrrrrrw}
    ///  .Bvrrrrtrrtrrrrrtrrrrw>
    ///    (rrrrrrrrrtrrrrrrvZ^
    ///    (vrrrrrrrrrrrrrvZ^
    ///    (vvvvrrvrvvwZ7^
    ///    (OzuuuXuuuX
    Portrait,
    ///  `````. ``  `  ` .WQQQQQQQQQmmggggga+gQQQQmQmmmmmmgggggg&(JJJJJ.+++&+++++J,` `
    ///   ````.._++zOOrrvwVUUUUUUUUUUUUUBBBBBWWWWWWWWWWHHHHHHHHHHHHHHHHHHHHHHHMMMM5((-
    ///   ````._(zuXvrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrttrrtttrtrrwwrrtrttrrwmOrrrvZ
    ///  ` ```.~(zuXvrrrrrrrrrrtrrtwAQmyrrrrrtrrwwwwrrrrrtrttrrtrrQMHMMkrrtrrdMHMSrrrw,
    ///   ````.~(zXXvrrrrrrrrrrrrrrWMNMNRrrrrwQdHMMMNmwvrrrrtrtrtrdHkkM8rtrtrrZ9rrrrvwF
    ///  ` ```.~(zuuvrrrrrrtrrtrrrrWMMMNSrrrvdMHHHHHHMNwvrrrrrAggyrZW90rAQgyrrtrrrrrrwF
    ///   ````.~(jVwzrrrrtrrrrtrtrrrrZZrrrrvwHHHHHHHHHM#uvrtrdMMMHRrrrrdHMMMKrrtrrtrvd]
    ///  `````.~(+z!OvrrrrrrrrrrrtrrrrrrrrvrwM#HHHHHHH#HzvrrrdMHHMSrrrrdMHNM9rtrrtrrvd!
    ///   ````.._<~ .zvrrrtrrtrrrrtrrrrrrrrrvXMH#HHHHMBzvrrrtrrwwrwgHHmrrZVrrrrrtrrvqF
    ///   ```...~`   .OvrrrrrrrrrrrrrtrrrrrrvvXXHMMMHUzvvrrrrttrrrdMkHMKrrrtrtrrrrvqY
    ///  ` ```.`      `?wvrrrrrrrrrrrrrrrrrrrrrrvwwvvrrrrrtrrrrtrrZM@@HXrrrrrrrrwXd^
    ///   `              _7XwvrrrrrrrrrrrrrrrrrrrrrrrrrrrrrtrrrrrrrrrrrrrrrrrwwXV=
    ///                      _?7777777777777777777777777777777777777OOOOO77=!(=`
    Landscape,
}

mod rumble;

pub mod joycon_features;

mod global_packet_number {
    use std::ops::Add;

    /// Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct GlobalPacketNumber(pub u8);

    impl GlobalPacketNumber {
        pub fn next(self) -> GlobalPacketNumber {
            self + GlobalPacketNumber(1)
        }
    }

    impl Default for GlobalPacketNumber {
        fn default() -> Self {
            GlobalPacketNumber(0x0)
        }
    }

    impl Add for GlobalPacketNumber {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let (GlobalPacketNumber(raw), GlobalPacketNumber(raw_rhs))
                = (self, rhs);
            let res = raw.wrapping_add(raw_rhs);

            GlobalPacketNumber(res)
        }
    }

    impl Into<u8> for GlobalPacketNumber {
        fn into(self) -> u8 {
            self.0
        }
    }
}

mod simple_joycon_driver;

pub trait JoyConDriver {
    /// If a sub-command is sent and no ACK packet is returned, tread again for the number of times of this value.
    const ACK_TRY: usize = 5;

    /// Send command to Joy-Con
    fn write(&self, data: &[u8]) -> JoyConResult<usize>;

    /// Read reply from Joy-Con
    fn read(&self, buf: &mut [u8]) -> JoyConResult<usize>;

    /// * timeout - milli seconds
    fn read_timeout(&self, buf: &mut [u8], timeout: i32) -> JoyConResult<usize>;

    /// Get global packet number
    fn global_packet_number(&self) -> u8;

    /// Increase global packet number. Increment by 1 for each packet sent. It loops in 0x0 - 0xF range.
    fn increase_global_packet_number(&mut self);

    /// Set rumble status.
    fn set_rumble_status(&mut self, rumble_l_r: (Option<Rumble>, Option<Rumble>));

    /// Set rumble status and send rumble command to JoyCon.
    /// If Joy-Con's rumble feature isn't activated, activate it.
    fn rumble(&mut self, rumble_l_r: (Option<Rumble>, Option<Rumble>)) -> JoyConResult<usize> {
        if !self.enabled_features().contains(&JoyConFeature::Vibration) {
            self.enable_feature(JoyConFeature::Vibration);
        }
        self.set_rumble_status(rumble_l_r);
        self.send_command_raw(Command::Rumble as u8, 0, &[])
    }

    /// Get rumble status.
    fn get_rumble_status(&self) -> (Option<Rumble>, Option<Rumble>);

    /// Send command, sub-command, and data (sub-command's arguments) with u8 integers
    /// This returns ACK packet for the command or Error.
    fn send_command_raw(&mut self, command: u8, sub_command: u8, data: &[u8]) -> JoyConResult<usize> {
        let mut buf = [0x0; 0x40];
        // set command
        buf[0] = command;
        // set packet number
        buf[1] = self.global_packet_number();
        // increase packet number
        self.increase_global_packet_number();

        // rumble
        let (rumble_l, rumble_r) = self.get_rumble_status();
        if let Some(rumble_l) = rumble_l {
            let rumble_left: [u8; 4] = rumble_l.into();
            buf[2..6].copy_from_slice(&rumble_left);
        }
        if let Some(rumble_r) = rumble_r {
            let rumble_right: [u8; 4] = rumble_r.into();
            buf[6..10].copy_from_slice(&rumble_right);
        }

        // set sub command
        buf[10] = sub_command;
        // set data
        buf[11..11 + data.len()].copy_from_slice(data);

        // send command
        self.write(&buf)
    }

    /// Send sub-command, and data (sub-command's arguments) with u8 integers
    /// This returns ACK packet for the command or Error.
    ///
    /// # Notice
    /// If you are using non-blocking mode,
    /// it is more likely to fail to validate the sub command reply.
    fn send_sub_command_raw(&mut self, sub_command: u8, data: &[u8]) -> JoyConResult<[u8; 362]> {
        use input_report_mode::sub_command_mode::AckByte;

        self.send_command_raw(1, sub_command, data)?;

        // check reply
        std::iter::repeat(())
            .take(Self::ACK_TRY)
            .flat_map(|()| {
                let mut buf = [0u8; 362];
                self.read(&mut buf).ok()?;
                let ack_byte = AckByte::from(buf[13]);

                match ack_byte {
                    AckByte::Ack { .. } => Some(buf),
                    AckByte::Nack => None
                }
            })
            .next()
            .ok_or(JoyConError::SubCommandError(sub_command, Vec::new()))
    }

    /// Send command, sub-command, and data (sub-command's arguments) with `Command` and `SubCommand`
    /// This returns ACK packet for the command or Error.
    ///
    /// # Notice
    /// If you are using non-blocking mode,
    /// it is more likely to fail to validate the sub command reply.
    fn send_command(&mut self, command: Command, sub_command: SubCommand, data: &[u8]) -> JoyConResult<usize> {
        let command = command as u8;
        let sub_command = sub_command as u8;

        self.send_command_raw(command, sub_command, data)
    }

    /// Send sub-command, and data (sub-command's arguments) with `Command` and `SubCommand`
    /// This returns ACK packet for the command or Error.
    fn send_sub_command(&mut self, sub_command: SubCommand, data: &[u8]) -> JoyConResult<[u8; 362]> {
        self.send_sub_command_raw(sub_command as u8, data)
    }

    /// Initialize Joy-Con's status
    fn reset(&mut self) -> JoyConResult<()> {
        // disable IMU (6-Axis sensor)
        self.send_sub_command(SubCommand::EnableIMU, &[0x00])?;
        // disable vibration
        self.send_sub_command(SubCommand::EnableVibration, &[0x00])?;

        Ok(())
    }

    /// Enable Joy-Con's feature. ex. IMU(6-Axis sensor), Vibration(Rumble)
    fn enable_feature(&mut self, feature: JoyConFeature) -> JoyConResult<()>;

    /// Get Enabled features.
    fn enabled_features(&self) -> &HashSet<JoyConFeature>;

    /// Get Joy-Con devices deal with.
    fn devices(&self) -> Vec<Arc<Mutex<JoyConDevice>>>;
}

pub mod input_report_mode;

pub mod lights;

pub mod device_info {
    use super::{*, input_report_mode::sub_command_mode::*};

    impl TryFrom<u8> for JoyConDeviceType {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            let kind = match value {
                0 => JoyConDeviceType::JoyConL,
                1 => JoyConDeviceType::JoyConR,
                2 => JoyConDeviceType::ProCon,
                _ => Err(())?
            };

            Ok(kind)
        }
    }

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct JoyConMacAddress(pub [u8; 6]);

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct JoyConDeviceInfo {
        pub firmware_version: u16,
        pub device_type: JoyConDeviceType,
        pub mac_address: JoyConMacAddress,
        pub colors_in_spi: bool,
    }

    impl TryFrom<[u8; 35]> for JoyConDeviceInfo {
        type Error = JoyConError;

        fn try_from(value: [u8; 35]) -> Result<Self, Self::Error> {
            let firmware_version = u16::from_be_bytes([value[0], value[1]]);
            let device_kind = JoyConDeviceType::try_from(value[2])
                .map_err(|()| {
                    JoyConError::SubCommandError(SubCommand::RequestDeviceInfo as u8, value.to_vec())
                })?;
            let mac_address = {
                let mut buf = [0u8; 6];
                buf.copy_from_slice(&value[4..10]);
                JoyConMacAddress(buf)
            };
            let colors_in_spi = value[12] == 1;

            Ok(JoyConDeviceInfo {
                firmware_version,
                device_type: device_kind,
                mac_address,
                colors_in_spi,
            })
        }
    }

    impl SubCommandReplyData for JoyConDeviceInfo {
        type ArgsType = [u8; 0];
        const SUB_COMMAND: SubCommand = SubCommand::RequestDeviceInfo;
        const ARGS: Self::ArgsType = [];
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Command {
    RumbleAndSubCommand = 1,
    NFC_IR_MCU_FW_Update = 3,
    Rumble = 16,
    RumbleAndRequestSpecificDataFromThe_NFC_IR_MCU = 17,
}

/// ref. https://github.com/dekuNukem/Nintendo_Switch_Reverse_Engineering/blob/master/bluetooth_hid_subcommands_notes.md
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum SubCommand {
    GetOnlyControllerState = 0,
    BluetoothManualPairing = 1,
    RequestDeviceInfo = 2,
    SetInputReportMode = 3,
    TriggerButtonsElapsedTime = 4,
    GetPageListState = 5,
    SetHCIState = 6,
    ResetPairingInfo = 7,
    SetShipmentLowPowerState = 8,
    SPIFlashRead = 10,
    SPIFlashWrite = 11,
    SPISectorErase = 12,
    ResetNFC_IR_MCU = 32,
    Set_NFC_IR_MCUConfiguration = 33,
    Set_NFC_IR_MCUState = 34,
    Get_x28_NFC_IR_MCUData = 41,
    Set_GPIO_PinOutputValue = 42,
    Get_x29_NFC_IR_MCUData = 43,
    SetPlayerLights = 48,
    GetPlayerLights = 49,
    SetHOMELight = 56,
    EnableIMU = 64,
    SetIMUSensitivity = 65,
    WriteToIMURegisters = 66,
    ReadIMURegisters = 67,
    EnableVibration = 72,
    GetRegulatedVoltage = 80,
    SetGPIOPinOutputValue = 81,
    GetGPIOPinInput_OutputValue = 82,
}
