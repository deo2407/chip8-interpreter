use std::fs::File;
use minifb::{Window, WindowOptions};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut window = Window::new("Test", 640, 320, WindowOptions {
        ..WindowOptions::default()
    })?;

    loop {
        window.update();
    }

    Ok(())
}
