use std::cell::RefCell;
use std::rc::Rc;
use sync::{Arc, Mutex};

use entity::player::Player;
use feature::Feature;
use floor::Floor;
use position::Position;
use tile::Tile;

static COLS: uint = 80;
static ROWS: uint = 24;
static FLOORS: uint = 18;

pub struct World {
    floors: Vec<Floor>,
    player: Rc<RefCell<Player>>
}

struct WorldWriter {
    world: Arc<Mutex<*World>>
}

impl WorldWriter {
    pub fn game_turn (&self, input: ::Input) {
        let world = self.world.lock();
        match input {
            NoInput => {},

            MoveDown => {
                world.try_move_player_relative(0, 1);
            },

            MoveUp => {
                world.try_move_player_relative(0, -1);
            },

            MoveLeft => {
                world.try_move_player_relative(-1, 0);
            },

            MoveRight => {
                world.try_move_player_relative(1, 0);
            },

            ClimbUp => {
                world.climb_up();
            },

            ClimbDown => {
                world.climb_down();
            },

            QuitGame => {}
        }
    }
}

#[Deriving(Clone)]
pub struct WorldReader {
    priv world: Arc<Mutex<*World>>
}

impl World {
    fn generate () -> World {
        let player = Rc::new(RefCell::new(Player::new()));
        let floors = Vec::from_fn(FLOORS, |_n: uint| -> Floor { Floor::new() });
        floors.get(0).get(COLS / 2, ROWS / 2).entity.set(Some(player.clone()));
        let world = World{floors: floors, player: player};
        world.generate_stairs();
        return world;
    }

    pub fn new () -> (WorldReader, WorldWriter) {
        let world = unsafe {
            Arc::new(Mutex::new(std::cast::transmute(~World::generate())))
        };
        
        (WorldReader{world: world}, WorldWriter{world: world})
    }

    fn generate_stairs (&self) {
        use feature::{Stairs, StairsUp, StairsDown};

        for (ix, floor) in self.floors.iter().enumerate() {
            if ix + 1 != FLOORS {
                let pos = Position::new(ix + 1, 1, 1);
                Stairs::new(floor.get(1, 1), pos, StairsUp);
            }

            if ix != 0 {
                let pos = Position::new(ix - 1, COLS - 1, ROWS - 1);
                Stairs::new(floor.get(1, 1), pos, StairsDown);
            }
        }
    }

    pub fn borrow_tile<'a> (&'a self, pos: Position) -> &'a Tile {
        self.floors.get(pos.floor).get(pos.col, pos.row)
    }

    pub fn try_move_player_absolute (&self, pos: Position) -> bool {
        if !self.can_place_entity(&*self.player.borrow(), pos) {
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

    pub fn can_place_entity (&self, entity: &Player, pos: Position) -> bool {
        self.floors.get(pos.floor).get(pos.col, pos.row).is_passable(entity)
    }

    pub fn move_player_absolute (&self, to_pos: Position) {
        assert!(self.can_place_entity(&*self.player.borrow(), to_pos));

        let from_pos = self.player.borrow().pos;

        let player = ::entity::player::Player {
            pos: to_pos
        };

        self.player.set(player);

        self.borrow_tile(from_pos).remove_entity();
        self.borrow_tile(to_pos).entity.set(Some(self.player.clone()));
    }

    pub fn climb_up (&self) {
        let player = &*self.player.borrow();
        let tile = {
            self.borrow_tile(player.pos)
        };
        tile.feature.borrow().climb_up(self, player);
    }

    pub fn climb_down (&self) {
        let player = &*self.player.borrow();
        let tile = {
            self.borrow_tile(player.pos)
        };
        tile.feature.borrow().climb_down(self, player);
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