use std::io::{self, Write};

const DEFAULT_SIZE: usize = 20;
// parse grid size from user input
pub fn get_grid_size() -> (usize, usize) {
    let width = read_dimension("Enter the width of the universe (minimum 20): ");
    let height = read_dimension("Enter the height of the universe (minimum 20): ");

    (width, height)
}

// validate inputs
pub fn read_dimension(prompt: &str) -> usize {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(value) if value >= DEFAULT_SIZE => return value,
            Ok(_) => println!("Too small. Size set to {DEFAULT_SIZE}."),
            Err(_) => println!("Invalid. Size set to {DEFAULT_SIZE}."),
        }
        return DEFAULT_SIZE; // Default value
    }
}