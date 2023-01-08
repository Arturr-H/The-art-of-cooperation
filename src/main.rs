/* Global allowances */
#![allow(dead_code, unused_imports)]

/* Imports */
mod pixel;
mod board;

use pixel::*;
use board::*;

use responder::prelude::*;

use std::{fs::File, io::Read};
use std::path::Path;

use bincode::{ config::legacy, Encode, Decode };

/* Constants */
const COLORS: &[&'static str] = &[
    "#40002b", "#990033", "#ff4500", "#ff9000", "#ffd635", "#fff8b8", "#aff257", "#00b368",
    "#008064", "#004852", "#007780", "#00ccc0", "#91fff8", "#358de6", "#2446a4", "#312680",
    "#493fb0", "#8e68d9", "#e4abff", "#b44ac0", "#721b8c", "#ba0d8b", "#ff5392", "#ffbfbf",
    "#ffffff", "#d4d7d9", "#898d90", "#515252", "#000000", "#5c3921", "#9c6926", "#ffb470"
];

/* Initialize */
fn main() -> () {
    dbg!(Pixel::get_history());
}