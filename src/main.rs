mod chip8;
mod display;

use std::fs::File;
use chip8::Chip8;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let program: &[u8] = [0x0, 0xEE].as_slice();
    let mut chip = Chip8::new(program)?;
    chip.run()
}
