use std::cell::RefCell;
use std::rc::Rc;
use std::vec_ng::Vec;

use entity::player::Player;

pub struct World {
    floors: Vec<Floor>,
    player: Rc<RefCell<Player>>
}

#[deriving(Eq, Show)]
pub struct Position {
    floor: uint,
    row: uint,
    col: uint
}

impl Position {
    pub fn add(&self, floor: int, col: int, row: int) -> Position {
        let new_floor = self.floor as int + floor;
        let new_col = self.col as int + col;
        let new_row = self.row as int + row;

        assert!(new_floor >= 0 && new_floor < 18);
        assert!(new_col >= 0 && new_col < 80);
        assert!(new_row >= 0 && new_row < 24);

        Position {
            floor: new_floor as uint,
            col: new_col as uint,
            row: new_row as uint
        }
    }
}

impl World {
    pub fn new () -> World {
        let player = Rc::new(RefCell::new(Player::new()));

        let floors = Vec::from_fn(18, |_n: uint| -> Floor { Floor::new() });

        floors.get(0).plan.get(12).get(39).entity.set(Some(player.clone()));

        World{floors: floors, player: player}
    }

    pub fn borrow_tile<'a> (&'a self, pos: Position) -> &'a Tile {
        self.floors.get(pos.floor).get(pos.col, pos.row)
    }

    pub fn try_move_player_absolute (&self, pos: Position) -> bool {
        if !self.can_place_entity(self.player.borrow().get(), pos) {
            return false
        }

        self.move_player_absolute(pos);
        true
    }

    pub fn try_move_player_relative (&self, cols: int, rows: int) -> bool {
        let pos: Position;

        {
            pos = self.player.borrow().pos;
        }

        self.try_move_player_absolute(pos.add(0, cols, rows))
    }

    pub fn can_place_entity (&self, _entity: &Player, pos: Position) -> bool {
        self.floors.get(pos.floor).get(pos.col, pos.row).passable
    }

    pub fn move_player_absolute (&self, to_pos: Position) {
        assert!(self.can_place_entity(self.player.borrow().get(), to_pos));

        let from_pos = self.player.borrow().pos;

        let player = ::entity::player::Player {
            pos: to_pos
        };

        self.player.set(player);

        self.borrow_tile(from_pos).entity.set(None);
        self.borrow_tile(to_pos).entity.set(Some(self.player.clone()));
    }

    pub fn move_player_relative (&self, cols: int, rows: int) {
        let old_pos: Position;
        {
            let player = self.player.borrow();
            old_pos = player.pos;
        }

        self.move_player_absolute(old_pos.add(0, cols, rows));
    }
}

#[deriving(Clone)]
pub struct Tile {
    passable: bool,
    entity: RefCell<Option<Rc<RefCell<Player>>>>
}

impl Tile {
    // TODO: Flip this around.
    pub fn new (passable: bool) -> Tile {
        use std::cell::RefCell;

        Tile {
            passable: passable,
            entity: RefCell::new(None)
        }
    }

    pub fn is_passable (&self) -> bool {
        self.passable
    }
}

pub struct Floor {
    plan: Vec<Vec<Tile>>,
    rows: uint,
    cols: uint
}

impl Floor {
    fn new () -> Floor {
        // TODO: Invert col and row here.
        let cols = 80;
        let rows = 24;

        fn exterior (cols: uint) -> Vec<Tile> {
            Vec::from_fn(cols, |_n| -> Tile { Tile::new(false) })
        }

        fn interior (cols: uint) -> Vec<Tile> {
            Vec::from_fn(cols, |n| -> Tile {
                match n {
                    0 => { Tile::new(false) },
                    79 => { Tile::new(false) },
                    _ => { Tile::new(true) }
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

    fn get<'a> (&'a self, col: uint, row: uint) -> &'a Tile {
        self.plan.get(row).get(col)
    }
}

#[test]
fn test_move_player_absolute () {
    let world = World::new();

    world.move_player_absolute(Position {floor: 0, col: 1, row: 2});
    assert_eq!(world.player.borrow().pos, Position{floor: 0, row: 2, col: 1});
}

#[test]
fn test_move_player_relative () {
    let world = World::new();

    world.move_player_absolute(Position {floor: 0, col: 5, row: 5});
    world.move_player_relative(1, -1);

    let player = world.player.borrow();

    assert_eq!(player.pos, Position{floor: 0, row: 4, col: 6});
}

#[test]
fn test_can_place_entity () {
    let world = World::new();
    let player = world.player.borrow();

    assert!(world.can_place_entity(player.get(), Position{floor: 0, col: 1, row: 1}));
    assert!(!world.can_place_entity(player.get(), Position{floor: 0, col: 0, row: 0}));
}

#[test]
fn test_try_move_player_relative () {
    let world = World::new();

    world.move_player_absolute(Position {floor: 0, col: 5, row: 5});
    world.try_move_player_relative(1, -1);

    let player = world.player.borrow();
    assert_eq!(player.pos, Position{floor: 0, row: 4, col: 6});
}