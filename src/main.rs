mod joystick;
use joystick::Joystick;
use std::{thread, time::Duration};

fn main() {
    let mut joystick = Joystick::new();

    loop {
        joystick.read();

        thread::sleep(Duration::from_millis(2));
    }
}
