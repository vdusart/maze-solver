pub struct Maze {
    pub width: u8,
    pub height: u8,
    pub grid: Vec<Cell>
}

impl Maze {

    pub fn new(width: u8, height: u8) -> Result<Maze, String> {
        let mut grid: Vec<Cell> = vec![];

        for x in 0..width {
            for y in 0..height {
                grid.push(Cell::new(x, y))
            }
        }

        Ok(Maze { width, height, grid })
    }
}

pub struct Cell {
    pub x: u8,
    pub y: u8,
    pub walls: [bool; 4]
}

impl Cell {
    pub fn new(x: u8, y: u8) -> Cell {
        let walls = [true; 4];
        Cell { x, y, walls }
    }
}