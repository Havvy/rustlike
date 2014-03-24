use entity::player::Player;
use world::World;
use position::Position;
use tile::Tile;

// Methods that features can overwrite.
// A feature by default cannot be climbed but is passable.
pub trait Feature {
    fn climb_up (&self, _w: &World, _p: &Player) {
        ()
    }

    fn climb_down (&self, _w: &World, _p: &Player) {
        ()
    }

    fn is_walkable (&self) -> bool {
        true
    }

    fn is_passable (&self, _p: &Player) -> bool {
        self.is_walkable()
    }
}

// Feature 0: NoFeature
pub struct EmptySpace;
impl Feature for EmptySpace {}

// Feature 1: Walls
pub struct Wall;

impl Feature for Wall {
    fn is_walkable (&self) -> bool {
        false
    }
}

// Feature 2: Stairs
#[deriving(Eq, Clone)]
pub enum StairsDirection {
    StairsUp,
    StairsDown
}

#[deriving(Clone)]
pub struct Stairs {
    priv destination: Position,
    priv direction: StairsDirection
}

impl Stairs {
    pub fn new (tile: &Tile, destination: Position, direction: StairsDirection) {
        let stairs = Stairs {
            destination: destination,
            direction: direction
        };

        tile.set_feature(~stairs as ~Feature);
    }
}

impl Feature for Stairs {
    fn climb_up (&self, world: &World, _player: &Player) {
        match self.direction {
            StairsUp => {
                world.move_player_absolute(self.destination);
            },

            _ => { }
        }
    }

    fn climb_down (&self, world: &World, _player: &Player) {
        match self.direction {
            StairsDown => {
                world.move_player_absolute(self.destination);
            },

            _ => { }
        }
    }
}