#[crate_id = "rusttower#0.1"];
#[crate_type = "lib"];

extern crate sync;

use std::fmt;
use std::vec_ng::Vec;
use entity::player::Player;
use world::World;

pub mod world;
pub mod entity;

pub enum Input {
    NoInput,
    QuitGame,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown
}

impl Input {
    fn is_quit_game (&self) -> bool {
        match *self {
            QuitGame => { true },
            _ => { false }
        }
    }
}

pub struct SendTile {
    passable: bool,
    entity: Option<Player>
}

impl SendTile {
    fn from_tile (tile: &world::Tile) -> SendTile {
        let player: Option<Player> = match tile.entity.get() {
            None => { None },
            Some(player) => {
                Some(*(player.borrow().get().clone()))
            }
        };

        SendTile {
            passable: tile.passable,
            entity: player
        }
    }
}

impl fmt::Show for SendTile {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self.entity {
            None => {
                if self.passable {~"."} else {~"#"}
            },

            _ => {
                ~"@"
            }
        };

        write!(f.buf, "{}", out)
    }
}

pub struct GameState {
    floorplan: Vec<Vec<SendTile>>,
    //messages: Vec<~str>,
    end_of_game: bool
}

impl GameState {
    fn new (world: &World) -> GameState {
        let player = world.player.borrow();
        let floorplan = &world.floors.get(player.pos.floor).plan;

        let sendable_floorplan = floorplan.iter().map(|tiles: &Vec<world::Tile>| {
            tiles.iter().map(|tile: &world::Tile| {
                SendTile::from_tile(tile)
            }).collect()
        }).collect();
        
        GameState {
            floorplan: sendable_floorplan,
            //messages: Vec::new(),
            end_of_game: false
        }
    }

    fn quit (world: &World) -> GameState {
        let player = world.player.borrow();
        let floorplan = &world.floors.get(player.pos.floor).plan;

        let sendable_floorplan = floorplan.iter().map(|tiles: &Vec<world::Tile>| {
            tiles.iter().map(|tile: &world::Tile| {
                SendTile::from_tile(tile)
            }).collect()
        }).collect();
        
        GameState {
            floorplan: sendable_floorplan,
            //messages: Vec::new(),
            end_of_game: true
        }
    }
}

pub fn newgame () -> sync::DuplexStream<Input, GameState> {
    let (input_sender, gamestate_sender) = sync::duplex::<Input, GameState>();

    spawn(proc () {
        let world = World::new();
        let mut input: Input = NoInput;

        loop {
            if input.is_quit_game() {
                break;
            }
            let gamestate = game_turn(&world, input);
            gamestate_sender.send(gamestate);
            input = gamestate_sender.recv();
        }

        let gamestate = game_turn(&world, input);
        gamestate_sender.send(gamestate);
    });

    input_sender
}


fn game_turn (world: &World, input: Input) -> GameState {
    match input {
        NoInput => {
            GameState::new(world)
        },

        MoveDown => {
            world.try_move_player_relative(0, 1);
            GameState::new(world)
        },

        MoveUp => {
            world.try_move_player_relative(0, -1);
            GameState::new(world)
        },

        MoveLeft => {
            world.try_move_player_relative(-1, 0);
            GameState::new(world)
        },

        MoveRight => {
            world.try_move_player_relative(1, 0);
            GameState::new(world)
        },

        QuitGame => { 
            GameState::quit(world)
        }
    }
}