use rand::Rng;

pub struct Maze {
    pub width: u8,
    pub height: u8,
    pub grid: Vec<Cell>,
    pub stack: Vec<usize>,
    pub current_cell_index: usize,
    pub is_generating: bool,
    pub is_solving: bool
}

impl Maze {

    fn get_neighbors(&mut self, index: usize) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();
        let w = usize::from(self.width);
        let h = usize::from(self.height);

        if index > w {
            neighbors.push(index - w); // add top neighbor
        }

        if index % w != w - 1 {
            neighbors.push(index + 1); // add right neighbor
        }

        if index < w * (h - 1) {
            neighbors.push(index + w); // add bottom neighbor
        }

        if index % w != 0 {
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

    fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        let current = &mut self.grid[self.current_cell_index];
        current.visited = 1;
        let neighbors = self.get_neighbors(self.current_cell_index);
        let unvisited_neighbors: Vec<usize> = neighbors
            .into_iter()
            .filter(|n| self.grid[*n].visited == 0)
            .collect();

        if unvisited_neighbors.len() > 0 {
            let next_cell_index = unvisited_neighbors[rng.gen_range(0..unvisited_neighbors.len())];

            self.stack.push(self.current_cell_index);

            self.remove_walls(self.current_cell_index, next_cell_index);

            self.grid[next_cell_index].visited = 1;
            self.current_cell_index = next_cell_index;
        } else if self.stack.len() > 0{
            let next_cell_index = self.stack.pop().unwrap();
            self.current_cell_index = next_cell_index;
        } else {
            self.is_generating = false;
            self.is_solving = true;
        }
    }

    fn is_possible_neighbor(&mut self, n: usize) -> bool {
        let mov = n as i32 - self.current_cell_index as i32;
        let is_wall = match mov {
            -1 => self.grid[self.current_cell_index].walls[3],
            1 => self.grid[self.current_cell_index].walls[1],
            _ if mov == self.width as i32 => self.grid[self.current_cell_index].walls[2],
            _ if mov == -1 * self.width as i32 => self.grid[self.current_cell_index].walls[0],
            _ => {
                panic!("Impossible neighbor choice");
            }
        };
        !is_wall && self.grid[n].visited == 1
    }

    fn solve(&mut self) {
        let mut rng = rand::thread_rng();
        let neighbors = self.get_neighbors(self.current_cell_index);
        let possible_neighbors: Vec<usize> = neighbors
            .into_iter()
            .filter(|n| self.is_possible_neighbor(*n))
            .collect();

        self.grid[self.current_cell_index].visited = 2;
        if possible_neighbors.len() > 0 {
            let next_cell_index = possible_neighbors[rng.gen_range(0..possible_neighbors.len())];

            self.stack.push(self.current_cell_index);

            self.grid[next_cell_index].visited = 2;
            self.current_cell_index = next_cell_index;
        } else if self.stack.len() > 0{
            let next_cell_index = self.stack.pop().unwrap();
            self.current_cell_index = next_cell_index;
        }
        
    }

    pub fn update(&mut self, speed_up: bool) {
        if self.is_generating {
            loop {
                self.generate();
                if !speed_up || !self.is_generating {
                    break;
                }
            }
        } else if self.is_solving {
            loop {
                self.solve();
                if self.current_cell_index as u16 == self.width as u16 * self.height as u16 - 1 {
                    self.is_solving = false;
                    self.stack.push(self.current_cell_index);
                }
                if !speed_up || !self.is_solving {
                    break;
                } 
            }
        }
    }

    pub fn new(width: u8, height: u8) -> Result<Maze, String> {
        let mut grid: Vec<Cell> = vec![];
        let stack: Vec<usize> = vec![];

        for y in 0..height {
            for x in 0..width {
                grid.push(Cell::new(x, y))
            }
        }

        Ok(Maze { width, height, grid, stack, current_cell_index: 0, is_generating: true, is_solving: false })
    }
}

pub struct Cell {
    pub x: u8,
    pub y: u8,
    pub walls: [bool; 4],
    pub visited: u8
}

impl Cell {
    pub fn new(x: u8, y: u8) -> Cell {
        let walls = [true; 4];
        Cell { x, y, walls, visited: 0 }
    }
}