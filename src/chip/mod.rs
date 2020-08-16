mod display;
mod keyboard;
mod sound;

use display::Display;
use keyboard::KeyBoard;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sound::Sound;
use std::fs;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;

pub struct Cpu {
    sdl_context: sdl2::Sdl,
    display: Display,
    keyboard: KeyBoard,
    sound: Sound,
    event_pump: sdl2::EventPump,

    stack: Vec<u16>,
    ram: [u8; 4096],     // 4KB memory
    registers: [u8; 16], // 16 8-bit register
    index: u16,          // 1 16-bit register (store memory address)
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,      // Program counter
    sp: u8,       // Stack Pointer
    paused: bool, // Is cpu paused (for some instruction)
}

impl Cpu {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let display = Display::new(&sdl_context, "Chip 8", 16, 64, 32);
        let keyboard = KeyBoard::new();
        let sound = Sound::new(&sdl_context);
        let mut ram: [u8; 4096] = [0; 4096];

        //laod sprites into memory
        load_sprites(&mut ram);

        Self {
            sdl_context,
            display,
            keyboard,
            sound,
            event_pump,

            stack: vec![0],
            ram,
            registers: [0; 16],
            index: 0,
            sound_timer: 0,
            delay_timer: 0,
            pc: 0x200, // Program start at 0x200 on chip 8
            sp: 0,
            paused: false,
        }
    }

    // This will load program into memory
    pub fn load(&mut self, rom: &mut fs::File) {
        let mut temp = vec![0_u8];
        let count = rom.read_to_end(&mut temp).expect("Load method failed");

        for i in 0..count {
            self.ram[0x200 + i] = temp[i];
        }
    }

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
        print!("Plotting pixel {},{}\r", x, y);
    }
}

// helper function
#[inline] // don't know what i'm doing
fn load_sprites(memory: &mut [u8]) {
    let sprites: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    for i in 0..sprites.len() {
        memory[i] = sprites[i]
    }
}

impl Cpu {
    fn execute_instruction(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x000 => match opcode {
                0x00E0 => self.display.clear(), //CLS
                0x00EE => self.pc = self.stack.pop().unwrap(),
                _ => {}
            },

            0x1000 => self.pc = opcode & 0x0FFF,
            0x2000 => {
                let nnn = opcode & 0x0FFF;
                self.stack.push(self.pc);
                self.pc = nnn;
            } // call subroutine,
            0x3000 => {
                // 3xkk
                let x = opcode & 0x0F00;
                let kk = opcode & 0xFF;
                if self.registers[x as usize] == kk as u8 {
                    self.pc += 2
                }
            }
            0x4000 => {
                // 4xkk
                let x = opcode & 0x0F00;
                let kk = opcode & 0xFF;
                if self.registers[x as usize] != kk as u8 {
                    self.pc += 2
                }
            }

            0x5000 => {
                // 5xy0
                let x = opcode & 0x0F00;
                let y = opcode & 0x00F0;
                if x == y {
                    self.pc += 2
                }
            }

            0x6000 => {
                //6xkk
                let x = opcode & 0x0F00;
                let kk = opcode & 0xFF;
                self.registers[x as usize] = kk as u8;
            }

            0x7000 => {
                //7xkk
                let x = opcode & 0x0F00;
                let kk = opcode & 0xFF;
                self.registers[x as usize] = self.registers[x as usize] + kk as u8;
            }

            0x8000 => {
                //8xy0
                let x = opcode & 0x0F00;
                let y = opcode & 0x00F0;
                let x = x as usize;
                let y = y as usize;
                match opcode & 0xF {
                    0 => self.registers[x] = self.registers[x],
                    1 => self.registers[x] = self.registers[x] | self.registers[y],
                    2 => self.registers[x] = self.registers[x] & self.registers[y],
                    3 => self.registers[x] = self.registers[x] ^ self.registers[y],
                    4 => {
                        let sum: u16 = self.registers[x] as u16 + self.registers[y] as u16;
                        if sum > 255 {
                            // set Vf register (overflow)
                            self.registers[0xF] = 1;
                        } else {
                            // clear Vf register (no-overflow)
                            self.registers[0xF] = 0;
                        }
                        self.registers[x] = (sum & 0xFF) as u8;
                    }

                    5 => {
                        if self.registers[x] > self.registers[y] {
                            self.registers[x] = self.registers[x] - self.registers[y];
                            // set Vf flag
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[x] = self.registers[y] - self.registers[x];
                            // clear Vf flag
                            self.registers[0xF] = 0;
                        }
                    }
                    6 => {
                        // least significatn bit is 1
                        if self.registers[x] & 1 == 1 {
                            self.registers[0xF] = 1
                        } else {
                            self.registers[0xF] = 0
                        }
                        self.registers[x] /= 2;
                    }
                    7 => {
                        if self.registers[y] > self.registers[x] {
                            self.registers[x] = self.registers[y] - self.registers[x];
                            // set Vf flag
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[x] = self.registers[x] - self.registers[y];
                            // clear Vf flag
                            self.registers[0xF] = 0;
                        }
                    }
                    0xE => {
                        // most significatn bit is 1
                        if self.registers[x] & 1 << 7 == 1 {
                            self.registers[0xF] = 1
                        } else {
                            self.registers[0xF] = 0
                        }
                        self.registers[x] *= 2;
                    }
                    _ => {}
                }
            }

            0x9000 => {
                let x = opcode & 0x0F00;
                let y = opcode & 0x00F0;
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc += 2
                }
            }

            0xA000 => {
                self.index = opcode & 0x0FFF;
            }

            0xB000 => {
                self.pc = self.registers[0] as u16 + (opcode & 0x0FFF);
            }

            0xC000 => {
                let random_byte = rand::thread_rng().gen::<u8>();
                let x = opcode & 0x0F00;
                let kk = opcode & 0x00FF;
                self.registers[x as usize] = kk as u8 & random_byte;
            }

            0xD000 => {}

            0xE000 => match opcode & 0x00FF {
                0x9E => {}
                0xA1 => {}
                _ => {}
            },

            0xF000 => {
                let x = opcode & 0x0F00;
                let x = x as usize;
                match opcode & 0x00FF {
                    0x07 => self.registers[x] = self.delay_timer,
                    0x0A => { /*later*/ }
                    0x15 => self.delay_timer = self.registers[x],
                    0x18 => self.sound_timer = self.registers[x],
                    0x1E => self.index += self.registers[x] as u16,
                    0x29 => { /*later*/ }
                    0x33 => {
                        self.ram[self.index as usize] = (x / 100) as u8;
                        self.ram[(self.index + 1) as usize] = ((x / 10) % 10) as u8;
                        self.ram[(self.index + 2) as usize] = (x % 10) as u8;
                    }
                    0x55 => {
                        for i in 0..=x{
                            self.ram[(self.index + i as u16 ) as usize] = self.registers[i];
                        }
                    }
                    0x65 => {
                        for i in 0..=x{
                            self.registers[i] = self.ram[(self.index + i as u16) as usize];
                        }
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}
