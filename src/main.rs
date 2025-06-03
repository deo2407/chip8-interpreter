mod chip8;
mod display;
mod audio;

extern crate rand;

use std::fs::File;
use chip8::Chip8;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let program: &[u16] = [0x00E0].as_slice();
    let mut chip = Chip8::new(program)?;
    chip.run()
}
