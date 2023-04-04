pub struct Maze {
    pub width: u8,
    pub height: u8,
    pub grid: Vec<Vec<bool>>
}

impl Maze {

    pub fn new(width: u8, height: u8) -> Result<Maze, String> {
        let grid = vec![vec![false; width.into()]; height.into()];

        Ok(Maze { width, height, grid })
    }

    pub fn get_cell(&mut self, x: u8, y: u8) -> Result<bool, String> {
        if x >= self.width || y >= self.height {
            return Err(String::from("'get_cell': out of bounds"));
        }
        let cell_value = self.grid[usize::from(x)][usize::from(y)];
        Ok(cell_value)
    }
}