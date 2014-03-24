pub mod player {
    use position::Position;
    
    #[deriving(Clone)]
    pub struct Player {
        pos: Position
    }

    impl Player {
        pub fn new() -> Player {
            Player {
                pos: Position {
                    floor: 0,
                    row: 12,
                    col: 40
                }
            }
        }
    }
}