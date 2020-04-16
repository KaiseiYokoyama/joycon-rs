use joycon_rs::prelude::*;

fn main() -> JoyConResult<()>{
    // First, connect your Joy-Cons to your computer!
    JoyConManager::new()?
        .connected_joycon_devices
        .iter()
        .try_for_each::<_, JoyConResult<()>>(|d| {
            println!("{:?}", d.get_product_string()?);
            Ok(())
        })?;

    Ok(())
}
