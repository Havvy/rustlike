static COLS: uint = 80;
static ROWS: uint = 24;
static FLOORS: uint = 18;

#[deriving(Eq, Show, Clone)]
pub struct Position {
    floor: uint,
    row: uint,
    col: uint
}

impl Position {
    pub fn new(floor: uint, col: uint, row: uint) -> Position {
        Position {
            floor: floor,
            col: col,
            row: row
        }
    }

    pub fn add(&self, floor: int, col: int, row: int) -> Position {
        let new_floor = self.floor as int + floor;
        let new_col = self.col as int + col;
        let new_row = self.row as int + row;

        assert!(new_floor >= 0 && new_floor < FLOORS as int);
        assert!(new_col >= 0 && new_col < COLS as int);
        assert!(new_row >= 0 && new_row < ROWS as int);

        Position {
            floor: new_floor as uint,
            col: new_col as uint,
            row: new_row as uint
        }
    }
}