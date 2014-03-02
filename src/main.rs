#[feature(globs)];

extern crate ncurses;

use self::world::World;

pub mod entity;
pub mod world;

#[main]
fn main () {
    initialize_ncurses();
    let world = initialize_game();

    loop {
        display(&world);

        let input = get_input();
        let quit: bool = game_turn(&world, input);

        if quit {
            finalize_game(&world);
            finalize_ncurses();
            break;
        }
    }
}

// Lot of global state from C. :(
fn initialize_ncurses () {
    use ncurses::*;

    initscr();
    clear();

    noecho();
    cbreak();
    curs_set(CURSOR_INVISIBLE);
}

fn finalize_ncurses () {
    use ncurses::*;

    endwin();
}

fn initialize_game () -> World {
    let player = entity::player::new();
    world::new(player)
}

fn finalize_game (_world: &World) {
    
}

fn get_input () -> i32 {
    ncurses::getch()
}

fn display (world: &World) {
    use ncurses::*;

    let map = &world.levels[0].map;

    for row in range(0, map.len()) {
        mvprintw(row as i32, 0, map[row].map(|row| -> ~str {
            row.to_str()
        }).concat());
    }
}

fn game_turn (_world: &World, _input: i32) -> bool {
    true
}