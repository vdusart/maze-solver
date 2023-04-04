pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<bool>>
}

impl Maze {

    pub fn new(width: usize, height: usize) -> Result<Maze, String> {
        let grid = vec![vec![false; width]; height];

        Ok(Maze { width, height, grid })
    }

    pub fn get_square(&mut self, x: usize, y: usize) -> Result<bool, String> {
        if x >= self.width || y >= self.height {
            return Err(String::from("'get_square': out of bounds"));
        }
        let square_value = self.grid
            .get(x).unwrap()
            .get(y).unwrap();
        Ok(*square_value)
    }
}