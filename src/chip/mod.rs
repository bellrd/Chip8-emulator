mod display;
mod keyboard;
mod sound;

use display::Display;
use keyboard::KeyBoard;
use sound::Sound;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::Duration;

pub struct Cpu {
    sdl_context: sdl2::Sdl,
    display: Display,
    keyboard: KeyBoard,
    sound: Sound,
    event_pump: sdl2::EventPump,
}

impl Cpu {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let display = Display::new(&sdl_context, "Chip 8", 16, 64, 32);
        let keyboard = KeyBoard::new();
        let sound = Sound::new(&sdl_context);

        Self {
            sdl_context,
            display,
            keyboard,
            sound,
            event_pump,
        }
    }

    // This will load program into memory
    pub fn load(&mut self, _path: &str) {}

    // This will start execution of the program
    pub fn execute(&mut self) {
        'main: loop {
            if let Some(e) = self.event_pump.poll_event() {
                match e {
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        match key {
                            Keycode::A => self.keyboard.add_key('A'),
                            Keycode::B => self.keyboard.add_key('B'),
                            Keycode::C => self.keyboard.add_key('C'),
                            Keycode::D => self.keyboard.add_key('D'),
                            Keycode::E => self.keyboard.add_key('E'),
                            Keycode::F => self.keyboard.add_key('F'),
                            Keycode::Num1 => self.keyboard.add_key('1'),
                            Keycode::Num2 => self.keyboard.add_key('2'),
                            Keycode::Num3 => self.keyboard.add_key('3'),
                            Keycode::Num4 => self.keyboard.add_key('4'),
                            Keycode::Num5 => self.keyboard.add_key('5'),
                            Keycode::Num6 => self.keyboard.add_key('6'),
                            Keycode::Num7 => self.keyboard.add_key('7'),
                            Keycode::Num8 => self.keyboard.add_key('8'),
                            Keycode::Num9 => self.keyboard.add_key('9'),
                            Keycode::Num0 => self.keyboard.add_key('0'),
                            Keycode::Q => {
                                break 'main;
                            } //break 'main loop
                            _ => {} //ignore other keyboard char
                        }
                    }
                    _ => {} // ignore mouse and other event
                }
            } // events matching end here
              // execute instruction
            self.dummy_instruction();
            self.display.render();
            self.sound.resume();
        } // main loop ends here
    }
}

impl Cpu {
    pub fn dummy_instruction(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, 64);
        let y = rng.gen_range(0, 32);
        self.display.set_pixel(x, y);
        thread::sleep(Duration::from_millis(17));
        println!("Plotting pixel {},{}", x, y);
    }
}
