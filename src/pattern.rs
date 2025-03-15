use std::io;
use rand::Rng;

use crate::{custom_universe::CustomUniverse, cell::Cell};

// Enum to represent the available patterns
pub enum Pattern {
    Glider,
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    GliderGun,
    Random,
}

// pattern selection, add to enum before adding to other functions
pub fn select_pattern() -> Pattern {
    let mut input = String::new();

    println!("Select a starting pattern:");
    println!("1. Glider");
    println!("2. Blinker");
    println!("3. Toad");
    println!("4. Beacon");
    println!("5. Pulsar");
    println!("6. Glider Gun");
    println!("7. Random");

    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => Pattern::Glider,
        "2" => Pattern::Blinker,
        "3" => Pattern::Toad,
        "4" => Pattern::Beacon,
        "5" => Pattern::Pulsar,
        "6" => Pattern::GliderGun,
        "7" => Pattern::Random,
        _ => {
            println!("Invalid choice. Using Random.");
            Pattern::Random
        }
    }
}

// Apply pattern to universe
pub fn apply_pattern(universe: &mut CustomUniverse, pattern: Pattern) {
    match pattern {
        Pattern::Glider => {
            // Glider pattern
            universe.set_cell(1, 0, Cell::Alive);
            universe.set_cell(2, 1, Cell::Alive);
            universe.set_cell(0, 2, Cell::Alive);
            universe.set_cell(1, 2, Cell::Alive);
            universe.set_cell(2, 2, Cell::Alive);
        }
        Pattern::Blinker => {
            // Blinker pattern
            universe.set_cell(1, 1, Cell::Alive);
            universe.set_cell(1, 2, Cell::Alive);
            universe.set_cell(1, 3, Cell::Alive);
        }
        Pattern::Toad => {
            // Toad pattern
            universe.set_cell(2, 2, Cell::Alive);
            universe.set_cell(3, 2, Cell::Alive);
            universe.set_cell(4, 2, Cell::Alive);
            universe.set_cell(1, 3, Cell::Alive);
            universe.set_cell(2, 3, Cell::Alive);
            universe.set_cell(3, 3, Cell::Alive);
        }
        Pattern::Beacon => {
            // Beacon pattern
            universe.set_cell(1, 1, Cell::Alive);
            universe.set_cell(2, 1, Cell::Alive);
            universe.set_cell(1, 2, Cell::Alive);
            universe.set_cell(4, 3, Cell::Alive);
            universe.set_cell(3, 4, Cell::Alive);
            universe.set_cell(4, 4, Cell::Alive);
        }
        Pattern::Pulsar => {
            // Pulsar pattern
            let offsets = [(2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0)];
            for &(dx, dy) in &offsets {
                for i in 0..6 {
                    universe.set_cell(dx + i, dy + 2, Cell::Alive);
                    universe.set_cell(dx + i, dy + 7, Cell::Alive);
                    universe.set_cell(dx + 2, dy + i, Cell::Alive);
                    universe.set_cell(dx + 7, dy + i, Cell::Alive);
                }
            }
        }
        Pattern::GliderGun => {
            // Glider Gun pattern
            let gun_cells = [
                (1, 5), (2, 5), (1, 6), (2, 6),
                (11, 5), (11, 6), (11, 7),
                (12, 4), (12, 8),
                (13, 3), (13, 9),
                (14, 3), (14, 9),
                (15, 6),
                (16, 4), (16, 8),
                (17, 5), (17, 6), (17, 7),
                (18, 6),
                (21, 3), (21, 4), (21, 5),
                (22, 3), (22, 4), (22, 5),
                (23, 2), (23, 6),
                (25, 1), (25, 2), (25, 6), (25, 7),
                (35, 3), (35, 4), (36, 3), (36, 4),
            ];
            for &(x, y) in &gun_cells {
                universe.set_cell(x, y, Cell::Alive);
            }
        }
        Pattern::Random => {
            // Random pattern
            let mut rng = rand::thread_rng();
            for y in 0..universe.height {
                for x in 0..universe.width {
                    if rng.gen_bool(0.5) {
                        universe.set_cell(x, y, Cell::Alive);
                    }
                }
            }
        }
    }
}