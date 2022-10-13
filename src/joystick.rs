use debouncr::{debounce_4, Debouncer, Edge, Repeat4};
use input_linux::{sys, EventTime, Key, KeyEvent, KeyState, SynchronizeEvent, UInputHandle};
use rppal::gpio::{Gpio, InputPin, Level};
use std::fs::{self, File};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    Cps,
    Neogeo,
}

pub struct Button {
    input_pin: InputPin,
    pin: u8,
    state: Debouncer<u8, Repeat4>,
}

impl Button {
    pub fn new(gpio: &Gpio, pin: u8) -> Self {
        Self {
            input_pin: gpio.get(pin).expect("Error get pin").into_input_pullup(),
            pin,
            state: debounce_4(false),
        }
    }

    pub fn pin_to_key(&self, mode: &Mode) -> Key {
        match self.pin {
            4 => Key::ButtonDpadUp,
            17 => Key::ButtonDpadDown,
            27 => Key::ButtonDpadLeft,
            22 => Key::ButtonDpadRight,
            10 => Key::ButtonStart,
            9 => Key::ButtonSelect,
            25 => match mode {
                Mode::Cps => Key::ButtonSouth,
                Mode::Neogeo => Key::ButtonEast,
            },
            24 => match mode {
                Mode::Cps => Key::ButtonEast,
                Mode::Neogeo => Key::ButtonWest,
            },
            15 => match mode {
                Mode::Cps => Key::ButtonNorth,
                Mode::Neogeo => Key::ButtonSouth,
            },
            18 => match mode {
                Mode::Cps => Key::ButtonWest,
                Mode::Neogeo => Key::ButtonNorth,
            },
            14 => Key::ButtonTL,
            23 => Key::ButtonTR,
            8 => Key::ButtonTL2,
            16 => Key::ButtonTR2,
            21 => Key::ButtonMode,
            20 => Key::ButtonZ,
            _ => panic!("Pin not covered"),
        }
    }
}

pub struct Joystick {
    buttons: [Button; 16],
    device: UInputHandle<File>,
    mode: Mode,
}

impl Joystick {
    pub fn new() -> Joystick {
        let keys = [
            Key::ButtonDpadUp,
            Key::ButtonDpadDown,
            Key::ButtonDpadLeft,
            Key::ButtonDpadRight,
            Key::ButtonSelect,
            Key::ButtonStart,
            Key::ButtonMode,
            Key::ButtonSouth,
            Key::ButtonEast,
            Key::ButtonNorth,
            Key::ButtonWest,
            Key::ButtonTL,
            Key::ButtonTR,
            Key::ButtonTL2,
            Key::ButtonTR2,
            Key::ButtonZ,
        ];

        let pins = [4, 8, 9, 10, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24, 25, 27];

        let gpio = Gpio::new().expect("Error Gpio new");
        let uinput_file = fs::File::create("/dev/uinput").expect("Error file create /dev/uinput");
        let device = input_linux::UInputHandle::new(uinput_file);
        let input_id = input_linux::InputId {
            bustype: sys::BUS_VIRTUAL,
            vendor: 34,
            product: 10,
            version: 1,
        };

        device
            .set_evbit(input_linux::EventKind::Key)
            .expect("Set evbit");
        device
            .set_keybit(input_linux::Key::ButtonTrigger)
            .expect("Set keybit"); // informs linux that this is a joystick
        keys.map(|key| device.set_keybit(key).expect("Set evbit error"));
        device
            .create(&input_id, b"arduino-virtual-joystick", 0, &[])
            .expect("Create device");

        Joystick {
            buttons: pins.map(|pin| Button::new(&gpio, pin)),
            device,
            mode: Mode::Cps,
        }
    }

    pub fn read(&mut self) {
        for button in self.buttons.iter_mut() {
            let level = button.input_pin.read();
            let pressed = match level {
                Level::High => true,
                Level::Low => false,
            };

            let edge = button.state.update(pressed);
            if let Some(edge) = edge {
                let key_state = if edge == Edge::Falling {
                    if button.pin == 20 {
                        self.mode = match self.mode {
                            Mode::Cps => Mode::Neogeo,
                            Mode::Neogeo => Mode::Cps,
                        };
                    }

                    KeyState::PRESSED
                } else {
                    KeyState::RELEASED
                };
                let event = KeyEvent::new(
                    EventTime::new(0, 0),
                    button.pin_to_key(&self.mode),
                    key_state,
                );
                self.device
                    .write(&[*event.as_ref()])
                    .expect("Device write error");
            }
        }
        let event = SynchronizeEvent::report(EventTime::new(0, 0));
        self.device
            .write(&[*event.as_ref()])
            .expect("Device write error");
    }
}
