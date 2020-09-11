use array2d::Array2D;

pub struct Cell {
    pub on: bool
}

impl Cell {
    pub fn new(on: bool) -> Self {
        Self {
            on
        }
    }

    pub fn default() -> Cell {
        Cell::new(false)
    }
}

impl Clone for Cell {
    fn clone(&self) -> Cell {
        Cell::new(self.on)
    }
}

pub struct Grid {
    pub cells: Array2D<Cell>
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            cells: Array2D::filled_with(Cell::default(), width as usize, height as usize)
        }
    }

    pub fn width(&self) -> u32 {
        return self.cells.num_rows() as u32;
    }

    pub fn height(&self) -> u32 {
        return self.cells.num_columns() as u32;
    }
}

impl std::ops::Index<(u32, u32)> for Grid {
    type Output = Cell;

    fn index(&self, index: (u32, u32)) -> &Cell {
        let idx = (index.0 as usize, index.1 as usize);
        &self.cells[idx]
    }
}

impl std::ops::IndexMut<(u32, u32)> for Grid {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Cell {
        let idx = (index.0 as usize, index.1 as usize);
        &mut self.cells[idx]
    }
}
