use entity::player::Player;

pub struct World {
    levels: ~[Level],
    player: ~Player
}

pub struct Level {
    map: ~[~[Tile::Tile]]
}

pub fn new (player: ~Player) -> World {
    use self::Tile::Tile;

    use make_vec = std::vec::from_elem;

    let exterior = make_vec(80, Tile{passable: false});

    let mut interior= make_vec(80, Tile{passable: true});
    interior[0] = Tile{passable: false};
    interior[79] = Tile{passable: false};

    let mut map = make_vec(24, interior);
    map[0] = exterior.clone();
    map[23] = exterior.clone();

    World{levels: ~[Level{map: map}], player: player}
}

mod Tile {
    use std::fmt;
    use entity::Entity;

    #[deriving(Clone)]
    pub struct Tile {
        passable: bool,
        entity: ~Entity
    }

    impl fmt::Show for Tile {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f.buf, "{}", to_str(self))
        }
    }

    fn to_str (tile: &Tile) -> ~str {
        if tile.passable {~"."} else {~"#"}
    }
}