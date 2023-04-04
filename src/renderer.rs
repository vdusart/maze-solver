use sdl2::rect::{Rect, Point};
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::maze::{Maze, Cell};

pub struct Renderer {
    canvas: WindowCanvas,
    maze: Maze,
    padding: i32,
    cell_width: i32,
    cell_height: i32
}

impl Renderer {
    
    pub fn new(window: Window, maze: Maze) -> Result<Renderer, String> {
        let size = window.size();
        let canvas = window.into_canvas().build().unwrap();
        let padding: i32 = 10;
        let win_width = size.0 as i32;
        let win_height = size.1 as i32;
        let cell_width = (win_width - 2 * padding) / maze.width as i32;
        let cell_height = (win_height - 2 * padding) / maze.height as i32;
        Ok(Renderer { canvas, maze, padding, cell_width, cell_height })
    }

    fn draw_cell(&mut self, i: u16) {
        let cell = &self.maze.grid[usize::from(i)];
        let x = i32::from(cell.x) * self.cell_width + self.padding;
        let y = i32::from(cell.y) * self.cell_height + self.padding;
        let cell_w = self.cell_width;
        let cell_h = self.cell_height;
        
        if cell.walls[0] { // top wall
            self.canvas.draw_line(Point::new(x, y), Point::new(x + cell_w, y)).unwrap();
        }

        if cell.walls[1] { // right wall
            self.canvas.draw_line(Point::new(x + cell_w, y), Point::new(x + cell_w, y + cell_h)).unwrap();
        }

        if cell.walls[2] { // bottom wall
            self.canvas.draw_line(Point::new(x, y + cell_h), Point::new(x + cell_w, y + cell_h)).unwrap();
        }

        if cell.walls[3] { // left wall
            self.canvas.draw_line(Point::new(x, y), Point::new(x, y + cell_h)).unwrap();
        }
    }

    fn draw_maze(&mut self) -> Result<(), String> {
        let n: u16 = self.maze.width as u16 * self.maze.height as u16;
        self.canvas.set_draw_color(Color::BLACK);
        for i in 0..n {
            self.draw_cell(i)
        }
        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();

        self.draw_maze()?;

        self.canvas.present();
        Ok(())
    }
}