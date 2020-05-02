use joycon_rs::prelude::{*,
                         spi::*,
                         input_report_mode::sub_command_mode::SubCommandReplyData,
                         calibration::AnalogStickCalibrations};

use std::convert::TryFrom;
use std::convert::TryInto;
use std::ops::Deref;

#[derive(Debug)]
struct FactoryStickCalibrations(AnalogStickCalibrations);

impl TryFrom<[u8; 35]> for FactoryStickCalibrations {
    type Error = JoyConError;

    fn try_from(value: [u8; 35]) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 18];
        buf.copy_from_slice(&value[0..18]);

        let cal = AnalogStickCalibrations::from(buf);

        Ok(Self(cal))
    }
}

impl SPIData for FactoryStickCalibrations {
    const ADDRESS: u32 = 0x603D;
    const LENGTH: u8 = 18;
}

fn main() -> JoyConResult<()> {
    let manager =
        JoyConManager::new()?;
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
            let device = match d.lock() {
                Ok(device) => device,
                Err(e) => e.into_inner(),
            };
            device.send_feature_report(&[0x71,0x3D,0x60,0x0,0xF8,0x12,0x0,0xE8])?;
            dbg!(line!());
            {
                let mut buf = [0u8;64];
                device.get_feature_report(&mut buf)?;
                dbg!([buf[0],buf[1],buf[2],buf[3],buf[4]]);
            }

            let mut driver = SimpleJoyConDriver::new(&d)?;
            dbg!(line!());

            let cal = FactoryStickCalibrations::once(&mut driver)?;
            dbg!(cal);

            Ok(())
        })?;

    Ok(())
}