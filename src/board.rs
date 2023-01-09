/* Imports */
use crate::pixel::{ Color, Pixel };
use bincode::{ Encode, Decode, config::legacy };
use std::{fs::{OpenOptions, File}, io::{self, Write, Read}};

/* Constants */
pub const SIDE:usize = 100;
pub const SIZE:usize = SIDE*SIDE;
pub const PATH_TO_SAVE:&'static str = "./board.bin";

/* Board */
#[derive(Encode, Decode, Debug)]
pub struct Board {
    /* One-dim board for better performance */
    pixels: [Option<Pixel>; SIZE]
}

/* Method implementations */
impl Board {
    pub fn new() -> Self {
        Self { pixels: [None; SIZE] }
    }
    pub fn from_pixels(pixels:[Option<Pixel>; SIZE]) -> Self {
        Self { pixels }
    }
    pub fn get(&self, x:usize, y:usize) -> Option<&Pixel> {
        self.pixels.get(SIDE*y + x).and_then(|i| i.as_ref())
    }
    pub fn set(&mut self, x:usize, y:usize, color:Color) -> () {
        if x >= SIDE || y >= SIDE { return; };
        self.pixels[SIDE*y + x] = Some(Pixel::new(x as u16, y as u16, color));
    }

    pub fn get_pixels(&self) -> [Option<Pixel>; SIZE] {
        self.pixels
    }

    /* Encode / decode */
    pub fn encode(&self) -> Option<Vec<u8>> {
        bincode::encode_to_vec(&self.pixels, legacy().with_variable_int_encoding()).ok()
    }
    pub fn decode(slice:&[u8]) -> Option<Self> {
        match bincode::decode_from_slice::<[Option<Pixel>; SIZE], _>(slice, legacy().with_variable_int_encoding()) {
            Ok(e) => Some(Self::from_pixels(e.0)),
            Err(e) => panic!("{e}")
        }
    }

    /* Save / Getter */
    pub fn save(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(PATH_TO_SAVE)?;

        let encoded = match self.encode() {
            Some(e) => e,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Failed to encode board."))
        };

        file.write(&encoded)?;
        Ok(())
    }
    pub fn from_save() -> io::Result<Self> {
        let mut file = File::open(PATH_TO_SAVE)?;

        /* Pre-alloc */
        let mut buf:Vec<u8> = Vec::new();
        
        /* Return */
        file.read_to_end(&mut buf)?;
        match Self::decode(&buf) {
            Some(e) => Ok(e),
            None => Err(io::Error::new(io::ErrorKind::Other, "Failed to decode board."))
        }
    }
}