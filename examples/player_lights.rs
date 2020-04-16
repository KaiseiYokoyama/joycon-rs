use joycon_rs::prelude::{*, lights::*};

fn main() -> JoyConResult<()> {
    // First, connect your Joy-Cons to your computer!
    JoyConManager::new()?
        .connected_joycon_devices
        .into_iter()
        .try_for_each::<_,JoyConResult<()>>(|d| {
            let mut driver = SimpleJoyConDriver::new(d)?;

            let lights_status = LightsStatus {
                light_up: vec![LightUp::LED1,LightUp::LED2],
                flash: vec![Flash::LED0,Flash::LED3]
            };

            // Set player lights
            driver.set_player_lights(&lights_status.light_up, &lights_status.flash)?;

            // Get player lights
            let lights_status_received = driver.get_player_lights()?.extra.reply;

            assert_eq!(lights_status_received, lights_status);

            Ok(())
        })?;

    Ok(())
}