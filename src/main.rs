#[feature(globs)];
#[feature(struct_variant)];

extern crate ncurses;

use self::world::World;
use std::task;

pub mod entity;
pub mod world;

#[main]
fn main () {
    initialize_ncurses();

    let result = task::try(proc () {
        let world = World::new();

        loop {
            display(&world);

            let input = get_input();
            ncurses::mvprintw(82i32, 0, input.to_str());

            let should_quit = game_turn(&world, input);

            if should_quit {
                break;
            }
        }
    });

    finalize_ncurses();

    match result {
        Err(_) => {
            println!("An error occured!");
        },

        _ => {}
    }
}

// Lot of global state from C. :(
fn initialize_ncurses () {
    use ncurses::*;

    initscr();
    clear();

    noecho();
    cbreak();
    keypad(stdscr, true);
    curs_set(CURSOR_INVISIBLE);
}

fn finalize_ncurses () {
    use ncurses::*;

    endwin();
}

fn get_input () -> i32 {
    ncurses::getch()
}

fn display (world: &World) {
    use ncurses::*;
    use std::vec_ng::Vec;

    let floorplan = &world.floors.get(0).plan;

    for row_ix in range(0, floorplan.len()) {
        let row = floorplan.get(row_ix).iter().map(|row: &world::Tile| -> ~str {
            row.to_str()
        }).collect::<Vec<~str>>().concat();

        mvprintw(row_ix as i32, 0, row);
    }
}

// Returns whether or not game should quit.
fn game_turn (world: &World, input: i32) -> bool {
    use ncurses::*;

    match input {
        KEY_UP => {
            world.try_move_player_relative(0, -1);
            false
        },

        KEY_DOWN => {
            world.try_move_player_relative(0, 1);
            false
        },

        KEY_LEFT => {
            world.try_move_player_relative(-1, 0);
            false
        },

        KEY_RIGHT => {
            world.try_move_player_relative(1, 0);
            false
        },

        // F1 | 'q' To quit the game.
        KEY_F1 | 113 => { true },

        _ => { false}
    }
}