#[cfg(test)]
mod board_tests {
    /* Imports */
    use the_art_of_cooperation::{ board::{ Board, SIZE }, pixel::Color };

    #[test]
    pub fn initialization() -> () {
        let board = Board::new();

        /* Check that they're the same length */
        assert_eq!(board.get_pixels().len() == SIZE, true);
    }

    #[test]
    pub fn set_and_get() -> () {
        let mut board = Board::new();

        /* Set tile */
        board.set(41, 84, Color::LightBrown);

        /* Get tile */
        assert_eq!(matches!(board.get(41, 84).unwrap().color(), Color::LightBrown), true)
    }

    #[test]
    pub fn get_empty() -> () {
        let board = Board::new();

        /* Get empty tile */
        assert_eq!(board.get(20, 10).is_none(), true)
    }
}