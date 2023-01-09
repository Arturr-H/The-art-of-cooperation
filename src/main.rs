/* Global allowances */
#![allow(dead_code, unused_imports)]

/* Imports */
mod pixel;
mod board;
mod auth;

use pixel::*;
use board::*;

use responder::prelude::*;
use std::{ fs::File, io::Read, env::var };
use std::path::Path;

use bincode::{ config::legacy, Encode, Decode };
use dotenv::dotenv;
use lazy_static::lazy_static;

/* Constants */
const COLORS: &[&'static str] = &[
    "#40002b", "#990033", "#ff4500", "#ff9000", "#ffd635", "#fff8b8", "#aff257", "#00b368",
    "#008064", "#004852", "#007780", "#00ccc0", "#91fff8", "#358de6", "#2446a4", "#312680",
    "#493fb0", "#8e68d9", "#e4abff", "#b44ac0", "#721b8c", "#ba0d8b", "#ff5392", "#ffbfbf",
    "#ffffff", "#d4d7d9", "#898d90", "#515252", "#000000", "#5c3921", "#9c6926", "#ffb470"
];
lazy_static! {
    pub static ref ACCOUNT_MANAGER_URL:String = var("ACCOUNT_MANAGER_URL").unwrap();
}

/* Initialize */
fn main() -> () {
    dotenv().unwrap();

    /* Load ENV variables and pre-warn about non-existence */
    var("ACCOUNT_MANAGER_URL").unwrap();

    let mut _board = Board::from_save().unwrap();
}
