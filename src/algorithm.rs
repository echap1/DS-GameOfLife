use crate::grid::Grid;

pub trait Algorithm {
    fn update(&mut self, grid: &mut Grid);
}

pub struct GameOfLife {
    temp_grid: Grid
}

impl GameOfLife {
    pub fn new(temp_grid: Grid) -> Self {
        Self {
            temp_grid
        }
    }

    fn num_neighbors(x: u32, y: u32, grid: &Grid) -> u8 {
        let mut n: u8 = 0;

        if x != 0 && grid[(x - 1, y)].on { n += 1 }
        if x != 0 && y != 0 && grid[(x - 1, y - 1)].on { n += 1 };
        if x != 0 && y + 1 != grid.height() && grid[(x - 1, y + 1)].on { n += 1 };

        if y != 0 && grid[(x, y - 1)].on { n += 1 }
        if y != 0 && x + 1 != grid.width() && grid[(x + 1, y - 1)].on { n += 1 };

        if x + 1 != grid.width() && grid[(x + 1, y)].on { n += 1 }
        if x + 1 != grid.width() && y + 1 != grid.height() && grid[(x + 1, y + 1)].on { n += 1 }

        if y + 1 != grid.height() && grid[(x, y + 1)].on { n += 1 }

        n
    }
}

impl Algorithm for GameOfLife {
    fn update(&mut self, grid: &mut Grid) {
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let n = Self::num_neighbors(x, y, grid);
                // print!("{}", n);

                match n {
                    2 if grid[(x, y)].on => self.temp_grid[(x, y)].on = true,
                    3 => self.temp_grid[(x, y)].on = true,
                    _ => self.temp_grid[(x, y)].on = false
                }
            }
            // println!();
        }

        for x in 0..grid.width() {
            for y in 0..grid.height() {
                grid[(x, y)].on = self.temp_grid[(x, y)].on;
            }
        }
    }
}