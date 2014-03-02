use std::cell::RefCell;

use entity::Entity;
use entity::player::Player;

pub struct World {
    levels: ~[Level],
    player: Player
}

impl World {
    pub fn new (player: Player) -> World {
        use self::Tile::Tile;
        use make_vec = std::vec::from_fn;

        fn exterior () -> ~[Tile] {
            make_vec(80, |_n| -> Tile {
                Tile{passable: false, entity: RefCell::new(None)}
            })
        }

        fn interior () -> ~[Tile] {
            make_vec(80, |n| -> Tile {
                match n {
                    0 => {Tile{passable: false, entity: RefCell::new(None)}},
                    79 => {Tile{passable: false, entity: RefCell::new(None)}},
                    _ => {Tile{passable: true, entity: RefCell::new(None)}}
                }
            })
        }

        let map: ~[~[Tile]] = make_vec(24, |n| -> ~[Tile] {
            match n {
                0 => { exterior() }
                23 => { exterior() }
                _ => { interior() }
            }
        });

        map[12][39].entity.set(Some(~player as ~Entity));

        World{levels: ~[Level{map: map}], player: player}
    }
}

pub struct Level {
    map: ~[~[Tile::Tile]]
}

mod Tile {
    use std::fmt;
    use std::cell::RefCell;
    use entity::Entity;

    pub struct Tile {
        passable: bool,
        entity: RefCell<Option<~Entity:>>
    }

    impl fmt::Show for Tile {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f.buf, "{}", to_str(self))
        }
    }

    fn to_str (tile: &Tile) -> ~str {
        match tile.entity.borrow().get() {
            &None => {
                if tile.passable {~"."} else {~"#"}
            },

            _ => {
                ~"@"
            }
        }
    }
}