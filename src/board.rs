/* Imports */
use crate::pixel::{ Color, Pixel };

/* Constants */
pub const SIDE:usize = 100;
pub const SIZE:usize = SIDE*SIDE;

/* Board */
pub struct Board {
    /* One-dim board for better performance */
    pixels: [Option<Pixel>; SIZE]
}

/* Method implementations */
impl Board {
    pub fn new() -> Board {
        Board { pixels: [None; SIZE] }
    }
    pub fn get(&self, x:usize, y:usize) -> Option<&Pixel> {
        self.pixels.get(SIDE*y + x).and_then(|i| i.as_ref())
    }
    pub fn set(&mut self, x:usize, y:usize, color:Color) -> () {
        if x >= SIDE || y >= SIDE { return; }
        self.pixels[SIDE*y + x] = Some(Pixel::new(x as u16, y as u16, color));
    }

    pub fn get_pixels(&self) -> [Option<Pixel>; SIZE] {
        self.pixels
    }
}