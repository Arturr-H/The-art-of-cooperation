/* Global allowances */
#![allow(dead_code, unused_imports)]

/* Imports */
mod pixel;
mod board;
mod auth;

use pixel::*;
use board::*;

use responder::prelude::*;
use std::thread;
use std::{ fs::File, io::Read, env::var };
use std::path::Path;

use bincode::{ config::legacy, Encode, Decode };
use dotenv::dotenv;
use lazy_static::lazy_static;

/* Constants */
const STACK_SIZE:usize = 50 * 1024 * 1024;
const COLORS: &[&'static str] = &[
    "#40002b", "#990033", "#ff4500", "#ff9000", "#ffd635", "#fff8b8", "#aff257", "#00b368",
    "#008064", "#004852", "#007780", "#00ccc0", "#91fff8", "#358de6", "#2446a4", "#312680",
    "#493fb0", "#8e68d9", "#e4abff", "#b44ac0", "#721b8c", "#ba0d8b", "#ff5392", "#ffbfbf",
    "#ffffff", "#d4d7d9", "#898d90", "#515252", "#000000", "#5c3921", "#9c6926", "#ffb470"
];
lazy_static! {
    pub static ref ACCOUNT_MANAGER_URL:String = var("ACCOUNT_MANAGER_URL").unwrap();
    pub static ref BOARD:Board = Board::from_save().unwrap();
}

/* Initialize */
fn main() -> () {
    let thr = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {

        dotenv().unwrap();

        /* Load ENV variables and pre-warn about non-existence */
        var("ACCOUNT_MANAGER_URL").unwrap();

        /* Http-routes */
        let routes = &[
            Route::ControlledStack(auth::origin_control, "data", &[
                Route::Get("board", get_board)
            ])
        ];

        /* Start server */
        Server::new()
            .address("127.0.0.1")
            .routes(routes)
            .port(8081)
            .start()
            .unwrap();
    }).unwrap();

    thr.join().unwrap();
}

/* Getters */
fn get_board(stream: &mut Stream) -> () {
    stream.respond(
        200,
        Respond::new()
            .json(
                &format!(
                    "{{\"pixels\":{:?}}}",
                    BOARD
                        .get_pixels()
                        .map(|e| if let Some(a) = e { a.into() } else { 255 } )
                )
            )
    );
}

/* Setters */
fn set_pixel(stream: &mut Stream) -> () {
    let pixel:Pixel = match stream.headers.get("pixel") {
        Some(e) => {
            let mut is_valid = true;
            let elems = e
                .split(";")
                .collect::<Vec<&str>>()
                .iter()
                .map(|e| match e.parse::<u16>() {
                    Ok(e) => e,
                    Err(_) => { is_valid = false; 0 }
                })
                .collect::<Vec<u16>>();

            if is_valid {
                let x = elems[0];
                let y = elems[1];
                let color = elems[2];
                Pixel::new(x, y, Color::from(color as u8))
            }else {
                return stream.respond(400, Respond::new().text("Invalid pixel"));
            }
        },
        None => return stream.respond_status(400)
    };
}