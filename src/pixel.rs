/* Imports */
use bincode::config::legacy;
use bincode::{ Encode, Decode };

use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};

/* Constants */
pub(crate) const PIXELS_HISTORY_PATH: &'static str = "./pixels.bin";

/* Pixel struct. (x, y, col, de/encode config) */
#[derive(Debug, Encode, Decode, Copy, Clone)]
pub struct Pixel(u16, u16, Color);

/* Color */
#[derive(Debug, Encode, Decode, Copy, Clone)]
#[repr(u8)]
pub enum Color {
    DarkPurple, DarkRed, OrangeRed, Orange,
    Yellow, LightYellow, LightGreen, Green,
    DarkGreen, DarkTeal, Teal, Cyan, LightCyan,
    LightBlue, Blue, DarkBlue, Purple, Violet,
    LightViolet, Pink, DarkPink, DarkPink2,
    LightPink, LightPink2, White, LightGray,
    Gray, DarkGray, Black, Brown, Brown2, LightBrown,
}

/* Method implementations */
impl Pixel {
    /* Constructor */
    pub fn new(x:u16, y:u16, color:Color) -> Self {
        Self(x, y, color)
    }

    /* Encode pixel to bytes */
    pub fn encode(&self) -> Option<Vec<u8>> {
        bincode::encode_to_vec(&self, legacy().with_variable_int_encoding()).ok()
    }

    /* Decode */
    pub fn decode(slice:&[u8]) -> Option<Self> {
        match bincode::decode_from_slice::<Pixel, _>(slice, legacy().with_variable_int_encoding()) {
            Ok(e) => Some(
                Self::new( e.0.0, e.0.1, e.0.2 )
            ),
            Err(_) => None
        }
    }

    /* Getters */
    pub fn coordinate(&self) -> (&u16, &u16) {
        (&self.0, &self.1)
    }
    pub fn color(&self) -> &Color {
        &self.2
    }

    /* Append to history file */
    pub fn archive(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(PIXELS_HISTORY_PATH)?;
        let encoded = match self.encode() {
            Some(e) => e,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Failed to encode pixel."))
        };

        file.write(&encoded)?;
        file.write(&[0])?;
        Ok(())
    }

    /* Get pixels from history file */
    pub fn get_history() -> Vec<Pixel> {
        let pixels = File::open("./pixels.bin").unwrap();
        let pixels = pixels.bytes().map(|e| e.unwrap()).collect::<Vec<u8>>();
        let mut pixels = pixels.split(|e| *e == 0).collect::<Vec<&[u8]>>();
        pixels.pop();
    
        let pixels = pixels.iter().map(|e| Pixel::decode(e).unwrap()).collect::<Vec<Pixel>>();
        pixels
    }
}

/* Conversions */
impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::DarkPurple, 1 => Color::DarkRed, 2 => Color::OrangeRed, 3 => Color::Orange,
            4 => Color::Yellow, 5 => Color::LightYellow, 6 => Color::LightGreen, 7 => Color::Green,
            8 => Color::DarkGreen, 9 => Color::DarkTeal, 10 => Color::Teal, 11 => Color::Cyan,
            12 => Color::LightCyan, 13 => Color::LightBlue, 14 => Color::Blue, 15 => Color::DarkBlue,
            16 => Color::Purple, 17 => Color::Violet, 18 => Color::LightViolet, 19 => Color::Pink,
            20 => Color::DarkPink, 21 => Color::DarkPink2, 22 => Color::LightPink, 23 => Color::LightPink2,
            24 => Color::White, 25 => Color::LightGray, 26 => Color::Gray, 27 => Color::DarkGray,
            28 => Color::Black, 29 => Color::Brown, 30 => Color::Brown2, 31 => Color::LightBrown, _ => Color::Black,
        }
    }
}
impl Into<u8> for Color {
    fn into(self) -> u8 {
        self as u8
    }
}
