//! Run: cargo test --test '*'

/* Global allowances */
#![allow(
    dead_code,
    unused_imports
)]

#[cfg(test)]
mod color_tests {
    /* Imports */
    use the_art_of_cooperation::pixel::Color;

    #[test]
    pub fn to_u8() -> () {
        let color = Color::DarkPink; // 20
        assert_eq!(color as u8 == 20, true);
    }

    #[test]
    pub fn from_u8() -> () {
        let color = Color::from(20); // `DarkPink`
        assert_eq!(matches!(color, Color::DarkPink), true);
    }
}