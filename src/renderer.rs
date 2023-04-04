use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct Renderer { canvas: WindowCanvas }

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().unwrap();
        Ok(Renderer { canvas })
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.present();
        Ok(())
    }
}