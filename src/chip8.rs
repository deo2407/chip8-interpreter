use crate::display::Display;
// 16bits program
// nnn - (addr) lowest 12 bit 
// n - lowest 4 bit of the instr 
// x - lowest 4 bits of the of the high byte instr
// y - upper 4 bits of the low byte instr
// kk - lowest 8 bits of the instr 

// Standart CHIP8 program
// 00E0 - CLS - clear the display
// 00EE - RET - return from a subroutine (function)
// 1nnn - JP addr - set the PC to nnn
// 2nnn - CALL addr - increment SP, store PC in the stack, set PC to nnn
// 3xkk - SE Vx, byte - skip next instruction if Vx (register x) = kk
// 4xkk - SNE Vx, byte - skip next instruction if Vx (register x) != kk 
// 5xy0 - SE Vx, Vy - skip next instruction  if Vx = Vy
// 6xkk - LD Vx, byte - set Vx = kk
// 7xkk - Add Vx, byte - Vx = Vx + kk

let FONTSET: [u8; 80] = [
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

pub struct Chip8 {
    memory: [u8; 4096],
    stack: [u16; 16]
    pub registers: [u8; 16],
    pub I: u16,
    pub sound: u8,
    pub delay: u8,
    pub PC: u16,
    pub SP: u16
    display: Display
}

impl Chip8 {
    fn init_memory(program: &[u8]) -> Result<[u8; 4096]> {
        if program.len() > 4096 - 512 {
            return Err("Program too large to fit the memory".into());
        }
        
        let memory = [u8; 4096];
        memory[..80].copy_from_slice(&FONTSET);
        memory[512..512 + program.len()].copy_from_slice(program);

        memory
    }

    pub fn new(program: &[u8]) -> Result<Self> {
        let memory = init_memory(program)?;
        Ok(Self {
            memory,
            stack: [0; 16],
            registers: [0; 16],
            sound: 0,
            delay: 0,
            PC: 0x200,
            SP: 0
        })
    }

    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
