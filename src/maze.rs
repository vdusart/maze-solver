use rand::Rng;

pub struct Maze {
    pub width: u8,
    pub height: u8,
    pub grid: Vec<Cell>,
    stack: Vec<usize>,
    pub current_cell_index: usize
}

impl Maze {


    fn get_unvisited_neighbors(&mut self, index: usize) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();
        let w = usize::from(self.width);
        let h = usize::from(self.height);

        if index > w && !self.grid[index - w].visited {
            neighbors.push(index - w); // add top neighbor
        }

        if index % w != w - 1 && !self.grid[index + 1].visited {
            neighbors.push(index + 1); // add right neighbor
        }

        if index < w * (h - 1) && !self.grid[index + w].visited {
            neighbors.push(index + w); // add bottom neighbor
        }

        if index % w != 0 && !self.grid[index - 1].visited {
            neighbors.push(index - 1); // add left neighbor
        }

        neighbors
    }

    fn remove_walls(&mut self, current: usize, next: usize) {
        let mov = next as i32 - current as i32;
        match mov {
            -1 => {
                self.grid[current].walls[3] = false;
                self.grid[next].walls[1] = false;
            }
            1 => {
                self.grid[current].walls[1] = false;
                self.grid[next].walls[3] = false;
            }
            _ if mov == self.width as i32 => {
                self.grid[current].walls[2] = false;
                self.grid[next].walls[0] = false;
            }
            _ if mov == -1 * self.width as i32 => {
                self.grid[current].walls[0] = false;
                self.grid[next].walls[2] = false;
            }
            _ => {
                panic!("Impossible neighbor choice");
            }
        }
    }

    pub fn generate_maze(&mut self) {
        let mut rng = rand::thread_rng();

        let current = &mut self.grid[self.current_cell_index];
        current.visited = true;
        let neighbors = self.get_unvisited_neighbors(self.current_cell_index);

        if neighbors.len() > 0 {
            let next_cell_index = neighbors[rng.gen_range(0..neighbors.len())];

            self.stack.push(self.current_cell_index);

            self.remove_walls(self.current_cell_index, next_cell_index);

            self.grid[next_cell_index].visited = true;
            self.current_cell_index = next_cell_index;
        } else if self.stack.len() > 0{
            let next_cell_index = self.stack.pop().unwrap();
            self.current_cell_index = next_cell_index;
        }
    }

    pub fn new(width: u8, height: u8) -> Result<Maze, String> {
        let mut grid: Vec<Cell> = vec![];
        let stack: Vec<usize> = vec![];
        let current_cell_index = 0;

        for y in 0..height {
            for x in 0..width {
                grid.push(Cell::new(x, y))
            }
        }

        Ok(Maze { width, height, grid, stack, current_cell_index })
    }
}

pub struct Cell {
    pub x: u8,
    pub y: u8,
    pub walls: [bool; 4],
    pub visited: bool
}

impl Cell {
    pub fn new(x: u8, y: u8) -> Cell {
        let walls = [true; 4];
        let visited = false;
        Cell { x, y, walls, visited }
    }
}