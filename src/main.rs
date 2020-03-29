use joycon_rs::prelude::*;
use joycon_rs::joycon::JoyConManager;

fn main() -> JoyConResult<()>{
    let manager = JoyConManager::new()?;
    println!("{:?}", &manager.connected_joycon_devices);
    Ok(())
}