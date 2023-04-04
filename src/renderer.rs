use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::maze::Maze;

pub struct Renderer {
    canvas: WindowCanvas,
    maze: Maze
}

impl Renderer {
    
    pub fn new(window: Window, maze: Maze) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().unwrap();
        Ok(Renderer { canvas, maze })
    }

    fn draw_maze(&mut self) -> Result<(), String> {
        let w = self.maze.width;
        let h = self.maze.height;
        for x in 0..w {
            for y in 0..h {
                println!("Square[{0}, {1}] = {2}", x, y, self.maze.get_square(x, y).unwrap());
            }
        }
        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.draw_maze()?;

        self.canvas.present();
        Ok(())
    }
}