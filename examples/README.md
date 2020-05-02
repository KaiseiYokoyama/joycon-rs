# Joycon-rs examples
## Run
```bash
cargo run --example {example's name ex.home_light}
```

## Examples
### [`scan_for_joycons`](scan_for_joycons.rs)
Outputs the product name of the connected JoyCon, 
and then does so each time a new JoyCon is connected.

### [`home_light.rs`](home_light.rs)
The connected Joy-Con's HOME button emits light.

![home_light_image](../images/home_light.gif)

### [`player_lights.rs`](player_lights.rs)
Set the emission pattern for the player lights of the connected Joy-Cons.

![player_lights_image](../images/player_lights.gif)

### [`rumble.rs`](rumble.rs)
Operates the vibrator of the detected JoyCon and shakes it.

### [`simple_hid_report.rs`](simple_hid_report.rs)
Receive simple HID report from connected Joy-Cons.

### [`standard_hid_report.rs`](standard_hid_report.rs)
Receive standard full report (includes 6-Axis sensor IMU data) from connected Joy-Cons.
