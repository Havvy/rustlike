#[feature(globs)];
#[feature(struct_variant)];

extern crate rusttower;
extern crate ncurses;

use rusttower::GameState;
use rusttower::SendTile;

#[main]
fn main () {
    initialize_ncurses();

    let channel = rusttower::newgame();

    loop {
        let gamestate = channel.recv();

        display(&gamestate);

        if gamestate.end_of_game {
            break
        }

        channel.send(get_input());
    }

    ncurses::getch();
    finalize_ncurses();
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
    ncurses::endwin();
}

fn display (state: &GameState) {
    use ncurses::*;
    use std::vec_ng::Vec;

    let floorplan = &state.floorplan;

    for row_ix in range(0, floorplan.len()) {
        let row: ~str = floorplan.get(row_ix).iter().map(|tile: &SendTile| -> ~str {
            tile.to_str()
        }).collect::<Vec<~str>>().concat();

        mvprintw(row_ix as i32, 0, row);
    }
}

fn get_input () -> rusttower::Input {
    key_to_input(ncurses::getch())
}

fn key_to_input (key: i32) -> rusttower::Input {
    use ncurses::*;

    match key {
        KEY_UP => { rusttower::MoveUp },
        KEY_DOWN => { rusttower::MoveDown },
        KEY_LEFT => { rusttower::MoveLeft },
        KEY_RIGHT => { rusttower::MoveRight },
        // F1 | 'q' To quit the game.
        KEY_F1 | 113 => { rusttower::QuitGame },
        _ => { rusttower::NoInput }
    }
}