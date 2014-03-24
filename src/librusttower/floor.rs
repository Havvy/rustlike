use feature::Wall;
use tile::Tile;

pub struct Floor {
    priv plan: Vec<Vec<Tile>>,
    rows: uint,
    cols: uint
}

impl Floor {
    pub fn new () -> Floor {
        // TODO: Invert col and row here.
        let cols = 80;
        let rows = 24;

        fn exterior (cols: uint) -> Vec<Tile> {
            Vec::from_fn(cols, |_n| -> Tile {
                let tile = Tile::new();
                tile.set_feature(~Wall);
                tile
            })
        }

        fn interior (cols: uint) -> Vec<Tile> {
            Vec::from_fn(cols, |n| -> Tile {
                match n {
                    0 => { Tile::new() },
                    79 => { 
                        let tile = Tile::new();
                        tile.set_feature(~Wall);
                        tile
                    },
                    _ => { Tile::new() }
                }
            })
        }

        Floor {
            plan: Vec::from_fn(rows, |n| -> Vec<Tile> {
                match n {
                    0 => { exterior(cols) }
                    23 => { exterior(cols) }
                    _ => { interior(cols) }
                }
            }),

            rows: rows,
            cols: cols
        }
    }

    pub fn get<'a> (&'a self, col: uint, row: uint) -> &'a Tile {
        self.plan.get(row).get(col)
    }
}