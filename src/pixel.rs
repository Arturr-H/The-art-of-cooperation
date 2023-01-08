/* Global allowances */
#![allow(dead_code, unused_imports)]

/* Imports */
use bincode::{ config::legacy, Encode, Decode };

/* Pixel struct. (x, y, col) */
#[derive(Debug, Encode, Decode)]
pub struct Pixel(u16, u16, Color);

/* Color */
#[derive(Debug, Encode, Decode)]
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