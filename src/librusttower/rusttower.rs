#[crate_id = "rusttower#0.1"];
#[crate_type = "lib"];
#[feature(globs)];

extern crate sync;

use std::fmt;
use entity::player::Player;
use world::World;

pub mod world;
mod tile;
mod floor;
mod entity;
mod feature;
mod position;

#[deriving(Eq)]
pub enum Input {
    NoInput,
    QuitGame,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    ClimbUp,
    ClimbDown
}

pub fn newgame () -> (world::WorldReader, Sender<Input>) {
    let (reader, writer) = World::new();
    let (sender, receiver): (Sender<Input>, Receiver<Input>) = channel();

    spawn(proc () {
        loop {
            let input = receiver.recv();
            writer.game_turn(input);
            if input == QuitGame { break; }
        }
    });

    (reader, sender)
}