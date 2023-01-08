/* Global allowances */
#![allow(dead_code, unused_imports)]

/* Imports */
mod pixel;
use responder::prelude::*;

use std::fs::File;
use std::path::Path;

use bincode::{ config::legacy, Encode, Decode };

/* Constants */
const PIXELS_HISTORY_PATH: &'static str = "./pixels";
const COLORS: &[&'static str] = &[
    "#40002b", "#990033", "#ff4500", "#ff9000", "#ffd635", "#fff8b8", "#aff257", "#00b368",
    "#008064", "#004852", "#007780", "#00ccc0", "#91fff8", "#358de6", "#2446a4", "#312680",
    "#493fb0", "#8e68d9", "#e4abff", "#b44ac0", "#721b8c", "#ba0d8b", "#ff5392", "#ffbfbf",
    "#ffffff", "#d4d7d9", "#898d90", "#515252", "#000000", "#5c3921", "#9c6926", "#ffb470"
];

/* Initialize */
fn main() -> () {
    /* Get bincode configuration */
    let _opt = legacy().with_variable_int_encoding();


}