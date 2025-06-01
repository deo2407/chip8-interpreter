use minifb::{Window, WindowOptions};

use crate::Result;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const SCALE: usize = 25;

pub struct Display {
    pub window: Window,
    pub pixels: [bool; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Result<Self> {
        let mut pixels = [false; WIDTH * HEIGHT];
        let mut window = Window::new("Test", WIDTH * SCALE, HEIGHT * SCALE, WindowOptions::default())?;

        Ok(Self { window, pixels })
    }
    
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if x < WIDTH && y < HEIGHT {
            self.pixels[y * WIDTH + x] = value;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        if x < WIDTH && y < HEIGHT {
            return self.pixels[y * WIDTH + x];
        }
        false
    }

    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    fn render(&self) -> Vec<u32> {
        let buffer_width = WIDTH * SCALE;
        let buffer_height = HEIGHT * SCALE;

        let white = Self::from_u8_rgb(255, 255, 255);

        let mut buffer: Vec<u32> = vec![0; buffer_width * buffer_height];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel_on = self.get(x, y);

                if pixel_on {
                    for dy in 0..SCALE {
                        for dx in 0..SCALE {
                            let buffer_x = x * SCALE + dx;
                            let buffer_y = y * SCALE + dy;
                            let index = buffer_y * WIDTH * SCALE + buffer_x;

                            buffer[index] = white;
                        }
                    }
                }
            }
        }
        buffer
    }

    pub fn draw(&mut self) -> Result<()> {
        let buffer = self.render(); 

        self.window.update_with_buffer(&buffer, HEIGHT * SCALE, WIDTH * SCALE).unwrap();
        Ok(())
    }

    pub fn clear(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.set(x, y, false);
            }
        }
    }
}
