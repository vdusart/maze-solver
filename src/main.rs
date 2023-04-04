use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod renderer;

fn main() {
    println!("Starting the maze solver");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Maze Generator", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = renderer::Renderer::new(window).unwrap();


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("ESCAPE");
                },
                _ => {}
            }
        }
        
        renderer.draw().unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
