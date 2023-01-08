use crate::pixel::Color;

/* Imports */
use crate::Pixel;

/* Constants */
const SIZE:usize = 1000_000;

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
        self.pixels.get(SIZE*y + x).and_then(|i| i.as_ref())
    }
    pub fn set(&mut self, x:usize, y:usize, color:Color) -> () {
        match self.pixels.get_mut(SIZE*y + x).and_then(|i| i.as_mut()) {
            Some(e) => *e = Pixel::new(x as u16, y as u16, color),
            None => ()
        }
    }
}