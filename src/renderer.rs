use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::maze::Maze;

pub struct Renderer {
    canvas: WindowCanvas,
    maze: Maze,
    cell_width: u32,
    cell_height: u32
}

impl Renderer {
    
    pub fn new(window: Window, maze: Maze) -> Result<Renderer, String> {
        let size = window.size();
        let canvas = window.into_canvas().build().unwrap();
        let cell_width = size.0 / maze.width as u32;
        let cell_height = size.1 / maze.height as u32;
        Ok(Renderer { canvas, maze, cell_width, cell_height })
    }

    fn draw_square(&mut self, x: u8, y: u8) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255 / self.maze.width * x, 255 / self.maze.height * y, 50));
        self.canvas.fill_rect(Rect::new(
            i32::from(x) * self.cell_width as i32,
            i32::from(y) * self.cell_height as i32,
            self.cell_width,
            self.cell_height,
        ))?;

        Ok(())
    }

    fn draw_maze(&mut self) -> Result<(), String> {
        let w = self.maze.width;
        let h = self.maze.height;
        for x in 0..w {
            for y in 0..h {
                self.draw_square(x, y)?;
            }
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