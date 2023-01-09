//! Run: cargo test --test '*'

/* Global allowances */
#![allow(
    dead_code,
    unused_imports
)]

#[cfg(test)]
mod pixel_tests {
    /* Imports */
    use std::path::Path;
    use the_art_of_cooperation::pixel::{ Color, Pixel, PIXELS_HISTORY_PATH };

    #[test]
    pub fn encode_decode() -> () {
        let px = Pixel::new(32, 41, Color::LightPink);
        let decoded = Pixel::decode(&px.encode().unwrap()).unwrap();

        /* X */
        assert_eq!(decoded.coordinate().0 == &32, true);

        /* Y */
        assert_eq!(decoded.coordinate().1 == &41, true);

        /* Color */
        assert_eq!(matches!(decoded.color(), Color::LightPink), true);
    }

    #[test]
    pub fn get_coordinate() -> () {
        let px = Pixel::new(32, 41, Color::LightPink);

        /* X */
        assert_eq!(px.coordinate().0 == &32, true);

        /* Y */
        assert_eq!(px.coordinate().1 == &41, true);
    }

    #[test]
    pub fn get_color() -> () {
        let px = Pixel::new(32, 41, Color::LightPink);
        assert_eq!(matches!(px.color(), Color::LightPink), true);
    }

    #[test]
    pub fn history_file_availability() -> () {
        assert_eq!(Path::new(PIXELS_HISTORY_PATH).is_file(), true, "Path to pixel history file does not exist!")
    }
}