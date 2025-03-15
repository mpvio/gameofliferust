pub mod custom_universe;
pub mod pattern;
pub mod cell;
pub mod width_height_funcs;


use crossterm::event::{self, Event, KeyCode};
use width_height_funcs::get_grid_size;
use std::{thread, time::Duration};
use pattern::{apply_pattern, select_pattern};
use custom_universe::CustomUniverse;

fn main() {
    custom_universe();
}

fn custom_universe() {
    // get user input for grid size
    let (width, height) = get_grid_size();

    // initialize the universe
    let mut universe = CustomUniverse::new(width, height);

    // select a starting pattern
    let pattern = select_pattern();
    apply_pattern(&mut universe, pattern);

    // run simulation
    loop {
        print!("\x1B[2J\x1B[H"); // Clear terminal
        universe.display();
        universe.next_generation();

        // Check for "q" key press to quit
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        thread::sleep(Duration::from_millis(200));
    }
}


