//! Run: cargo test --test '*'

/* Global allowances */
#![allow(
    dead_code,
    unused_imports
)]

#[cfg(test)]
mod pixel_tests {
    /* Imports */
    use the_art_of_cooperation::pixel::{ Color, Pixel };

    #[test]
    pub fn decode() -> () {
        let px = Pixel::new(32, 41, Color::LightPink);
        let decoded = Pixel::decode(&px.encode().unwrap()).unwrap();

        /* X */
        assert_eq!(decoded.coordinate().0 == &32, true);

        /* Y */
        assert_eq!(decoded.coordinate().1 == &41, true);

        /* Color */
        assert_eq!(matches!(decoded.color(), Color::LightPink), true);
    }
}