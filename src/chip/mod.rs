#[allow(unused_imports, unused_variables, unused_braces, unused_import_braces)]
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
use std::io::Read;
use std::thread;
use std::time::Duration;

pub struct Cpu {
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
            //sdl_context,
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
            paused: false,
        }
    }

    // This will load program into memory
    pub fn load(&mut self, rom: &mut fs::File) {
        let mut temp = vec![0_u8];
        let count = rom.read_to_end(&mut temp).expect("Load failed");
        for i in 0..count {
            self.ram[0x200 + i] = temp[i + 1];
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
                            Keycode::A => self.keyboard.on_key_down(0xA),
                            Keycode::B => self.keyboard.on_key_down(0xB),
                            Keycode::C => self.keyboard.on_key_down(0xC),
                            Keycode::D => self.keyboard.on_key_down(0xD),
                            Keycode::E => self.keyboard.on_key_down(0xE),
                            Keycode::F => self.keyboard.on_key_down(0xF),
                            Keycode::Num1 => self.keyboard.on_key_down(1),
                            Keycode::Num2 => self.keyboard.on_key_down(2),
                            Keycode::Num3 => self.keyboard.on_key_down(3),
                            Keycode::Num4 => self.keyboard.on_key_down(4),
                            Keycode::Num5 => self.keyboard.on_key_down(5),
                            Keycode::Num6 => self.keyboard.on_key_down(6),
                            Keycode::Num7 => self.keyboard.on_key_down(7),
                            Keycode::Num8 => self.keyboard.on_key_down(8),
                            Keycode::Num9 => self.keyboard.on_key_down(9),
                            Keycode::Num0 => self.keyboard.on_key_down(0),
                            Keycode::Q => {
                                break 'main;
                            } //break 'main loop
                            _ => {} //ignore other keyboard char
                        }
                    }

                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => match key {
                        Keycode::A => self.keyboard.on_key_up(0xA),
                        Keycode::B => self.keyboard.on_key_up(0xB),
                        Keycode::C => self.keyboard.on_key_up(0xC),
                        Keycode::D => self.keyboard.on_key_up(0xD),
                        Keycode::E => self.keyboard.on_key_up(0xE),
                        Keycode::F => self.keyboard.on_key_up(0xF),
                        Keycode::Num1 => self.keyboard.on_key_up(1),
                        Keycode::Num2 => self.keyboard.on_key_up(2),
                        Keycode::Num3 => self.keyboard.on_key_up(3),
                        Keycode::Num4 => self.keyboard.on_key_up(4),
                        Keycode::Num5 => self.keyboard.on_key_up(5),
                        Keycode::Num6 => self.keyboard.on_key_up(6),
                        Keycode::Num7 => self.keyboard.on_key_up(7),
                        Keycode::Num8 => self.keyboard.on_key_up(8),
                        Keycode::Num9 => self.keyboard.on_key_up(9),
                        Keycode::Num0 => self.keyboard.on_key_up(0),

                        _ => {}
                    },
                    _ => {} // ignore mouse and other event
                }
            } // events matching end here
              // execute instruction
              //self.dummy_instruction();
            let mut instruction = self.ram[self.pc as usize] as u16;
            instruction <<= 8;
            instruction |= self.ram[(self.pc + 1) as usize] as u16;

            // if cpu is paused
            if self.paused {
                println!("Cpu is paused");
                if self.keyboard.should_wait_for_key == false {
                    let x = (instruction & 0x0F00) >> 8;
                    self.registers[x as usize] = self.keyboard.last_pressed_key.unwrap();
                    self.paused = false;
                }
            } else {
                // if cpu is not paused then execute instruction
                self.execute_instruction(instruction);
            }
            //self.dummy_instruction();
            //self.pc += 2;

            // update display after each instruction
            self.display.render();

            if self.sound_timer == 0 {
                self.sound.pause();
            } else {
                self.sound.resume();
                self.sound_timer -= 1;
            }

            if self.delay_timer != 0 {
                self.delay_timer -= 1;
            }

            thread::sleep(Duration::from_secs_f32(1.0 / 60_f32));
        } // main loop ends here
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
        self.pc += 2;
        //println!("Got opcode: {:#X}", opcode);
        match opcode & 0xF000 {
            0x000 => match opcode {
                // CLS
                0x00E0 => {
                    println!("0x00E0 : clearing screen");
                    self.display.clear();
                }
                //RET
                0x00EE => {
                    println!("RET: popping from stack");
                    self.pc = self.stack.pop().unwrap();
                }
                _ => println!("Zero instruction {} ", opcode),
            },

            //1nnn JP addr
            0x1000 => {
                println!("0x1000 JP {}", opcode & 0x0fff);
                self.pc = opcode & 0x0FFF
            }

            //2nnn CALL addr
            0x2000 => {
                println!("CALL addr");
                let nnn = opcode & 0x0FFF;
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            //3xkk SE vx,byte
            0x3000 => {
                println!("SE 3xkk");
                let x = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0x00FF;
                if self.registers[x as usize] == kk as u8 {
                    self.pc += 2
                }
            }
            //4xkk SNE vx,byte
            0x4000 => {
                println!("SNE 4xkk");
                let x = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0xFF;
                print!("{}, {}, {}", opcode, x, kk);
                if self.registers[x as usize] != kk as u8 {
                    self.pc += 2
                }
            }
            //5xy0 SE vx,vy
            0x5000 => {
                println!("SE 5xy0");
                let x = (opcode & 0x0F00) >> 8;
                let x = x as usize;
                let y = (opcode & 0x00F0) >> 4;
                let y = y as usize;
                if self.registers[x] == self.registers[y] {
                    self.pc += 2
                }
            }

            //6xkk LD vx,byte
            0x6000 => {
                println!("LD 6xkk");
                let x = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0xFF;
                self.registers[x as usize] = kk as u8;
            }

            //7xkk ADD vx,byte
            0x7000 => {
                println!("ADD 7xkk");
                let x = (opcode & 0x0F00) >> 8;
                let x = x as u8;
                let kk = (opcode & 0xFF) as u8;
                let sum: u32 = (self.registers[x as usize]) as u32 + (kk) as u32;
                self.registers[x as usize] = sum as u8;
            }

            0x8000 => {
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                let x = x as usize;
                let y = y as usize;
                match opcode & 0xF {
                    // 8xy0 LD vx,vy
                    0 => {
                        println!("LD 8xy0");
                        self.registers[x] = self.registers[y]
                    }

                    // 8xy1 OR vx,vy
                    1 => {
                        println!("OR 8xy1");
                        self.registers[x] = self.registers[x] | self.registers[y];
                    }

                    // 8xy2 AND vx,vy
                    2 => {
                        println!("AND 8xy2");
                        self.registers[x] = self.registers[x] & self.registers[y]
                    }

                    // 8xy3 XOR vx,vy
                    3 => {
                        println!("XOR 8xy3");
                        self.registers[x] = self.registers[x] ^ self.registers[y]
                    }

                    // 8xy4 ADD vx,vy
                    4 => {
                        println!("ADD 8xy4");
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

                    //8xy5 SUB vx,vy
                    5 => {
                        println!("SUB 8xy5");
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

                    //8xy6 SHR vx {,vy}
                    6 => {
                        println!("SHR 8xy6");
                        // least significatn bit is 1
                        if self.registers[x] & 1 == 1 {
                            self.registers[0xF] = 1
                        } else {
                            self.registers[0xF] = 0
                        }
                        self.registers[x] /= 2;
                    }

                    //8xy7 SUBN vx,vy
                    7 => {
                        println!("SUBN 8xy7");
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

                    //8xyE SHL vx {, vy}
                    0xE => {
                        println!("SHL 8xyE");
                        // most significatn bit is 1
                        if self.registers[x] & 1 << 7 == 1 {
                            self.registers[0xF] = 1
                        } else {
                            self.registers[0xF] = 0
                        }
                        let sum = self.registers[x] as u16 * 216;
                        self.registers[x] = sum as u8;
                    }
                    _ => {}
                }
            }

            //9xy0 SNE vx,vy
            0x9000 => {
                println!("SNE 9xy0");
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc += 2
                }
            }

            //Annn
            0xA000 => {
                println!("Annn Annn");
                self.index = opcode & 0x0FFF;
            }

            //Bnnn JP v0,addr
            0xB000 => {
                println!("JP Bnnn");
                self.pc = self.registers[0] as u16 + (opcode & 0x0FFF);
            }

            //cxkk RND vx,byte
            0xC000 => {
                println!("RND cxkk");
                let random_byte = rand::thread_rng().gen::<u8>();
                let x = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0x00FF;
                self.registers[x as usize] = kk as u8 & random_byte;
            }

            0xD000 => {
                //rlater
                //Dxyn
                println!("DRW Dxyn");
                self.registers[0xf] = 0;
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let n = (opcode & 0x000F) as usize;
                for row in 0..n {
                    let mut sprite = self.ram[(self.index + row as u16) as usize];
                    // get a row 10010011
                    for col in 0..8 {
                        // msb of the row is set the plot pixel
                        if sprite & 0b1000_0000u8 > 0 {
                            // draw given pixel at
                            if self.display.set_pixel(
                                (self.registers[x] + col) as usize,
                                (self.registers[y] + row as u8) as usize,
                            ) == 1
                            {
                                self.registers[0xf] = 1
                            }
                        }
                        // left shift by one
                        sprite <<= 1;
                    }
                }
            }

            0xE000 => match opcode & 0x00FF {
                //Ex9E SKP v /rlater
                0x9E => {
                    println!("SKP Ex9E");
                    let x = (opcode & 0x0F00) >> 8;
                    if self.keyboard.pressed_key[&(self.registers[x as usize])] {
                        self.pc += 2;
                    }
                }
                //ExA1 SKNP vx /rlater
                0xA1 => {
                    println!("SKNP ExA1");
                    let x = (opcode & 0x0F00) >> 8;
                    if !(self.keyboard.pressed_key[&(self.registers[x as usize])]) {
                        self.pc += 2;
                    }
                }
                _ => {}
            },

            0xF000 => {
                let x = (opcode & 0x0F00) >> 8;
                let x = x as usize;
                match opcode & 0x00FF {
                    //Fx07 LD vx,DT
                    0x07 => {
                        println!("LD Fx07");
                        self.registers[x] = self.delay_timer
                    }
                    //Fx0A LD vx,K /rlater
                    0x0A => {
                        println!("LD Fx0A wait for key");
                        self.paused = true;
                        self.keyboard.should_wait_for_key = true;
                    }
                    //Fx15 LD DT,vx
                    0x15 => {
                        println!("LD fx15");
                        self.delay_timer = self.registers[x]
                    }
                    //Fx18 LD ST,vx
                    0x18 => {
                        println! {"LD fx18"};
                        self.sound_timer = self.registers[x]
                    }
                    //Fx1E ADD I,vx
                    0x1E => {
                        println!("ADD fx1e");
                        self.index += self.registers[x] as u16
                    }
                    //Fx29 LD F,vx
                    0x29 => {
                        println!("LD fx29");
                        // load starting index of font for value(vx)
                        self.index = self.registers[x] as u16 * 5;
                    }
                    //Fx33 LD B,vx
                    0x33 => {
                        println!("LD Fx33");
                        self.ram[self.index as usize] = (x / 100) as u8;
                        self.ram[(self.index + 1) as usize] = ((x / 10) % 10) as u8;
                        self.ram[(self.index + 2) as usize] = (x % 10) as u8;
                    }

                    //Fx55 LD [I], Vx
                    0x55 => {
                        println!("LD fx55");
                        for i in 0..=x {
                            self.ram[(self.index + i as u16) as usize] = self.registers[i];
                        }
                    }

                    //Fx65 LD vx,[I]
                    0x65 => {
                        println!("LD Fx65");
                        for i in 0..=x {
                            self.registers[i] = self.ram[(self.index + i as u16) as usize];
                        }
                    }
                    _ => {}
                }
            }

            _ => {
                println!("INVALID INSTRUCTION");
            }
        }
    }
}

// impl Cpu {
//     pub fn dummy_instruction(&mut self) {
//         let mut rng = rand::thread_rng();
//         let x = rng.gen_range(0, 64);
//         let y = rng.gen_range(0, 32);
//         self.display.set_pixel(x, y);
//         thread::sleep(Duration::from_millis(17));
//         print!("Plotting pixel {},{}\r", x, y);
//     }
// }
