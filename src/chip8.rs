use minifb::{Window, WindowOptions};
use std::time::Duration;
use std::{thread, time};

use crate::audio::Beeper;
use crate::display::{self, Display};
use crate::Result;

const FONTSET: [u8; 80] = [
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

fn first(instruction: u16) -> u16 {
    instruction & 0x000F
}

fn second(instruction: u16) -> u16 {
    (instruction & 0x00F0) >> 4
}

fn third(instruction: u16) -> u16 {
    (instruction & 0x0F00) >> 8
}

fn fourth(instruction: u16) -> u16 {
    (instruction & 0xF000) >> 12
}

fn nnn(instruction: u16) -> u16 {
    instruction & 0x0FFF
}

fn kk(instruction: u16) -> u8 {
    (instruction & 0x00FF) as u8
}

pub struct Chip8 {
    memory: [u8; 4096],
    stack: [u16; 16],
    pub V: [u8; 16],
    pub I: u16,

    pub sound_reg: u8,
    pub delay_reg: u8,
    
    pub pc: u16,
    pub sp: u8,
    
    pub beeper: Beeper,
    display: Display,

    pub log: bool,
    draw_screen: bool
}

impl Chip8 {
    pub fn new(program: &[u16]) -> Result<Self> {
        let mut memory = Self::init_memory(program)?;
        let mut display = Display::new()?;
        let mut beeper = Beeper::new(); 
        Ok(Self {
            memory,
            stack: [0; 16],
            V: [0; 16],
            I: 0,
            sound_reg: 0,
            delay_reg: 0,
            pc: 0x200,
            sp: 0,
            beeper,
            display,
            log: true,
            draw_screen: false,
        })
    }

    fn init_memory(program: &[u16]) -> Result<[u8; 4096]> {
        if program.len() > 4096 - 512 {
            return Err("Program too large to fit the memory".into());
        }
        
        let mut memory = [0; 4096];
        memory[..80].copy_from_slice(&FONTSET);
        
        let mut index = 0x200;
        for &word in program {
            let high = (word >> 8) as u8;
            let low = (word & 0xFf) as u8;
            memory[index] = high;
            memory[index + 1] = low;
            index += 2;
        }

        Ok(memory)
    }

    fn handle_timers(&mut self) {
        if self.delay_reg > 0 {
            self.delay_reg -= 1;
        }

        if self.sound_reg == 0 {
            self.beeper.stop_beep();
        }

        if self.sound_reg > 0 {
            self.beeper.start_beep();
            self.sound_reg -= 1;
        }
    }

    fn fetch(&self) -> u16 {
        let high = (self.memory[self.pc as usize] as u16) << 8;
        let low = self.memory[self.pc as usize + 1] as u16;
        high | low
    }

    fn disassemble(&self, instruction: u16, desc: &str) {
        if self.log {
            println!("[ {instruction:04X} ]: {desc}");
        }
    } 

    fn execute(&mut self, instruction: u16) -> Result<()> {
        match instruction {
            0x00E0 => { 
                self.display.clear();
                self.pc += 2;
                self.draw_screen = true;
                self.disassemble(instruction, "CLS");
            },
            0x00EE => {
                let addr = self.stack[self.sp as usize];
                self.pc = addr;
                self.sp -= 1;
                self.disassemble(instruction, "RET");
            },
            0x0000 => {},
            _ => match first(instruction) {
                1 => {
                    self.pc = nnn(instruction);
                    self.disassemble(instruction, format!("JP {:04X}", self.pc).as_str());
                },
                2 => {
                    self.sp += 1;
                    self.stack[self.sp as usize] = self.pc;
                    self.pc = nnn(instruction);
                    self.disassemble(instruction, format!("CALL {:04X}", self.pc).as_str());
                },
                3 => {
                    let x = third(instruction) as usize;
                    let kk = kk(instruction);
                    if self.V[x] == kk {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                    self.disassemble(instruction, format!("SE {}, {kk}", self.V[x]).as_str());
                },
                4 => {
                    let x = third(instruction) as usize;
                    let kk = kk(instruction);
                    if self.V[x] != kk {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                    self.disassemble(instruction, format!("SNE {}, {kk}", self.V[x]).as_str());
                },
                5 => {
                    let x = third(instruction) as usize;
                    let y = second(instruction) as usize;
                    if self.V[x] == self.V[y] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                    self.disassemble(instruction, format!("SE {}, {}", self.V[x], self.V[y]).as_str());
                },
                6 => {
                    let x = third(instruction) as usize;
                    let kk = kk(instruction);
                    self.V[x] = kk;
                    self.pc += 2;
                    self.disassemble(instruction, format!("LD {}, {kk}", self.V[x]).as_str());
                },
                7 => {
                    let x = third(instruction) as usize;
                    let kk = kk(instruction);
                    self.V[x] += kk;
                    self.pc += 2;
                    self.disassemble(instruction, format!("ADD {}, {kk}", self.V[x]).as_str());
                },
                8 => {
                    match first(instruction) {
                        0 => {
                            let x = third(instruction) as usize;
                            let y = third(instruction) as usize;
                            self.V[x] = self.V[y];
                            self.pc += 2;
                            self.disassemble(instruction, format!("LD {}, {}", self.V[x], self.V[y]).as_str());
                        },
                        1 => {
                            let x = third(instruction) as usize;
                            let y = third(instruction) as usize;
                            self.V[x] = self.V[x] | self.V[y];
                            self.pc += 2;
                            self.disassemble(instruction, format!("OR {}, {}", self.V[x], self.V[y]).as_str());
                        },
                        2 => {
                            let x = third(instruction) as usize;
                            let y = third(instruction) as usize;
                            self.V[x] = self.V[x] & self.V[y];
                            self.pc += 2;
                            self.disassemble(instruction, format!("AND {}, {}", self.V[x], self.V[y]).as_str());
                        },
                        3 => {
                            let x = third(instruction) as usize;
                            let y = third(instruction) as usize;
                            self.V[x] = self.V[x] ^ self.V[y];
                            self.pc += 2;
                            self.disassemble(instruction, format!("XOR {}, {}", self.V[x], self.V[y]).as_str());
                        },
                        4 => {
                            let x = third(instruction) as usize;
                            let y = third(instruction) as usize;
                            match self.V[x].checked_add(self.V[y]) {
                                Some(val) => self.V[x] = val,
                                None => { 
                                    self.V[0xF] = 1;
                                    self.V[x] = self.V[x] + self.V[y];
                                }
                            }
                            self.pc += 2;
                            self.disassemble(instruction, format!("ADD {}, {}", self.V[x], self.V[y]).as_str());
                        },
                        5 => {
                            let x = third(instruction) as usize;
                            let y = third(instruction) as usize;
                            if self.V[x] > self.V[y] {
                                self.V[0xF] = 1;
                            } else {
                                self.V[0xF] = 0;
                            }
                            self.V[x] -= self.V[y];
                            self.pc += 2;
                            self.disassemble(instruction, format!("SUB {}, {}", self.V[x], self.V[y]).as_str());
                        },
                        6 => {
                            let x = third(instruction) as usize;
                            if (self.V[x] % 2 == 1) {
                                self.V[0xF] = 1;
                            } else {
                                self.V[0xF] = 0;
                            }
                            self.V[x] = self.V[x] >> 1;
                            self.pc += 2;
                            self.disassemble(instruction, format!("SHR {}", self.V[x]).as_str());
                        },
                        7 => {
                            let x = third(instruction) as usize;
                            let y = third(instruction) as usize;
                            if self.V[x] >= self.V[y] {
                                self.V[0xF] = 0;
                            } else {
                                self.V[0xF] = 1;
                            }
                            self.V[y] -= self.V[x];
                            self.pc += 2;
                            self.disassemble(instruction, format!("SUBN {}, {}", self.V[x], self.V[y]).as_str());
                        },
                        0xE => {
                            let x = third(instruction) as usize;
                            if (self.V[x] % 2 == 1) {
                                self.V[0xF] = 1;
                            } else {
                                self.V[0xF] = 0;
                            }
                            self.V[x] = self.V[x] << 1;
                            self.pc += 2;
                            self.disassemble(instruction, format!("SLR {}", self.V[x]).as_str());
                        },
                        _ => {
                            return Err("Unkown command".into());
                        }
                    }
                },
                9 => {
                    let x = third(instruction) as usize;
                    let y = second(instruction) as usize;
                    if self.V[x] != self.V[y] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                    self.disassemble(instruction, format!("SNE {}, {}", self.V[x], self.V[y]).as_str());
                },
                0xA => {
                    self.I = nnn(instruction); 
                    self.disassemble(instruction, format!("LD I, {}", self.I).as_str());
                },
                0xB => {
                    self.pc = nnn(instruction) + self.V[0] as u16;
                    self.disassemble(instruction, format!("JP {}, nnn", self.V[0]).as_str());
                },
                0xC => {
                    todo!();
                },
                _ => {
                    return Err("Unkown instruction".into());
                },
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let frame_duration = Duration::from_micros(16_666);
        self.display.draw().unwrap();

        while self.display.window.is_open() {
            let time_start = time::Instant::now();
            self.handle_timers();

            let instruction = self.fetch(); 
            self.execute(instruction).unwrap();

            if self.draw_screen {
                self.display.draw().unwrap();
            }

            let elapsed = time_start.elapsed();
            if elapsed < frame_duration {
                thread::sleep(frame_duration - elapsed);
            }
        }

        Ok(())
    }
}
