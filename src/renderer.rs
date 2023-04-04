use sdl2::rect::{Point, Rect};
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::maze::{Maze, Cell};

pub struct Renderer {
    canvas: WindowCanvas,
    padding_x: i32,
    padding_y: i32,
    cell_width: i32,
    cell_height: i32
}

impl Renderer {
    
    pub fn new(window: Window, maze_width: i32, maze_height: i32) -> Result<Renderer, String> {
        let goal_padding: i32 = 10;
        let size = window.size();
        let canvas = window.into_canvas().build().unwrap();
        let win_width = size.0 as i32;
        let win_height = size.1 as i32;
        let cell_width = (win_width - 2 * goal_padding) / maze_width;
        let cell_height = (win_height - 2 * goal_padding) / maze_height;
        let padding_x = (win_width - cell_width * maze_width) / 2;
        let padding_y = (win_height - cell_height * maze_height) / 2;
        Ok(Renderer { canvas, padding_x, padding_y, cell_width, cell_height })
    }

    fn draw_borders(&mut self, cell: &Cell) {
        let x = i32::from(cell.x) * self.cell_width + self.padding_x;
        let y = i32::from(cell.y) * self.cell_height + self.padding_y;
        let cell_w = self.cell_width;
        let cell_h = self.cell_height;

        self.canvas.set_draw_color(Color::BLACK);

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

    fn draw_cell(&mut self, cell: &Cell, is_current_cell: bool) {
        let x = i32::from(cell.x) * self.cell_width + self.padding_x;
        let y = i32::from(cell.y) * self.cell_height + self.padding_y;
        
        if cell.visited != 0 {
            self.canvas.set_draw_color(Color::RGB(128,0,128));
            self.canvas.fill_rect(Rect::new(
                i32::from(x),
                i32::from(y),
                self.cell_width as u32,
                self.cell_height as u32,
            )).unwrap();
        }
        if is_current_cell {
            self.canvas.set_draw_color(Color::RGB(0, 128,128));
            self.canvas.fill_rect(Rect::new(
                i32::from(x),
                i32::from(y),
                self.cell_width as u32,
                self.cell_height as u32,
            )).unwrap();
        }
        self.draw_borders(cell);
    }

    fn draw_cell_path(&mut self, cell: &Cell) {
        let x = i32::from(cell.x) * self.cell_width + self.padding_x;
        let y = i32::from(cell.y) * self.cell_height + self.padding_y;
        self.canvas.set_draw_color(Color::RED);
            self.canvas.fill_rect(Rect::new(
                i32::from(x),
                i32::from(y),
                self.cell_width as u32,
                self.cell_height as u32,
            )).unwrap();
        self.draw_borders(cell);
    }

    fn draw_maze(&mut self, maze: &Maze) -> Result<(), String> {
        let n: u16 = maze.width as u16 * maze.height as u16;
        self.canvas.set_draw_color(Color::BLACK);
        for i in 0..n {
            let cell = &maze.grid[usize::from(i)];
            self.draw_cell(cell, (maze.is_generating || maze.is_solving) && i == maze.current_cell_index as u16);
        }
        if !maze.is_generating {
            for i in 0..maze.stack.len() {
                let index = maze.stack[usize::from(i)];
                self.draw_cell_path(&maze.grid[index])
            }
        }
        Ok(())
    }

    pub fn draw(&mut self, maze: &Maze) -> Result<(), String> {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();

        self.draw_maze(maze)?;

        self.canvas.present();
        Ok(())
    }
}