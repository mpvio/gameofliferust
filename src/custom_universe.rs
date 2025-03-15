use crate::cell::Cell;

#[derive()]
pub struct CustomUniverse {
    pub width: usize,
    pub height: usize,
    grid: Vec<Vec<Cell>>,
}

impl CustomUniverse {
    pub fn new(width: usize, height: usize) -> Self {
        CustomUniverse {
            width,
            height,
            grid: vec![vec![Cell::Dead; width]; height],
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.grid[y][x] = cell;
    }

    fn count_live_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    if let Cell::Alive = self.grid[ny as usize][nx as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn next_generation(&mut self) {
        let mut new_grid = vec![vec![Cell::Dead; self.width]; self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let live_neighbors = self.count_live_neighbors(x, y);
                new_grid[y][x] = match (self.grid[y][x], live_neighbors) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };
            }
        }
        self.grid = new_grid;
    }

    pub fn display(&self) {
        for row in &self.grid {
            for &cell in row {
                let symbol = match cell {
                    Cell::Alive => '◼',
                    Cell::Dead => '◻',
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
}