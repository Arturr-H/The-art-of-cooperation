#[cfg(test)]
mod board_tests {
    /* Imports */
    use the_art_of_cooperation::{ board::{ Board, SIZE, PATH_TO_SAVE }, pixel::Color };
    use std::{ thread, path::Path };

    /* Constants */
    const STACK_SIZE:usize = 10 * 1024 * 1024;

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
        board.set(41, 12, Color::LightBrown);

        /* Get tile */
        assert_eq!(matches!(board.get(41, 12).unwrap().color(), Color::LightBrown), true)
    }

    #[test]
    pub fn get_empty() -> () {
        let board = Board::new();

        /* Get empty tile */
        assert_eq!(board.get(20, 10).is_none(), true)
    }

    #[test]
    pub fn save_and_retrieve() -> () {
        /* We need to build a thread with bigger stack size than usual */
        let thr = thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(|| {
                let mut board = Board::new();
                let length = board.get_pixels().len().clone();

                /* Set some random pixels */
                board.set(32, 12, Color::DarkTeal);
                board.set(12, 14, Color::Black);
                
                /* Encode to bytes */
                let bytes = board.encode().unwrap();
                drop(board);
                
                /* Decode */
                let retrieved = Board::decode(&bytes).unwrap();
                
                /* Check */
                assert_eq!(retrieved.get_pixels().len() == length, true);
                assert_eq!(matches!(retrieved.get(32, 12).unwrap().color(), Color::DarkTeal), true);
                assert_eq!(matches!(retrieved.get(12, 14).unwrap().color(), Color::Black), true);
                assert_eq!(retrieved.get(1, 12).is_none(), true);
            })
            .unwrap();

        thr.join().unwrap();
    }

    #[test]
    pub fn save_file_availability() -> () {
        assert_eq!(Path::new(PATH_TO_SAVE).is_file(), true, "Path to board save file does not exist!")
    }
}