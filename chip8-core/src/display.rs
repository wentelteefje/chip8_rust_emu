extern crate piston_window;
use piston_window::*;

pub struct Display {
    pixels: [[u8; 32]; 64],
}

impl Display {
    pub fn new() -> Self {
        Display { pixels: [[0u8; 32]; 64] }
    }

    pub fn draw_graphics(&self, window: &mut PistonWindow, e: &Event) {
        window.draw_2d(e, |c, g, _| {
            clear([1.0, 218.0 / 255.0, 244.0 / 255.0, 1.0], g); // Clear the screen to (255,218,244)
            for x in 0..64 {
                for y in 0..32 {
                    if self.pixels[x][y] != 0 {
                        rectangle(
                            [1.0, 1.0, 1.0, 1.0],                           // White color
                            [x as f64 * 10.0, y as f64 * 10.0, 10.0, 10.0], // Scale each pixel to 10x10
                            c.transform,
                            g,
                        );
                    }
                }
            }
        });
    }

    pub fn update_pixel(&mut self, x: usize, y: usize, val: u8) {
        self.pixels[x][y] = val;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> u8 {
        self.pixels[x][y]
    }


}