use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod renderer;
mod maze;

const WINDOW_WITH   : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

const MAZE_COLS     : u8  = 20;
const MAZE_ROWS     : u8  = 20;

fn main() {
    println!("Starting the maze solver...");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Maze Generator", WINDOW_WITH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut maze = maze::Maze::new(MAZE_COLS, MAZE_ROWS).unwrap();

    let mut renderer = renderer::Renderer::new(window, MAZE_COLS as i32, MAZE_ROWS as i32).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let speed_rate = 0;
    let mut current_frame = 0;
    let mut speed_up = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    maze = maze::Maze::new(MAZE_COLS, MAZE_ROWS).unwrap()
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    speed_up = true;
                },
                _ => {}
            }
        }
        current_frame += 1;
        if speed_up || current_frame > speed_rate {
            current_frame = 0;
            maze.update(speed_up);
            speed_up = false;
        }
        
        renderer.draw(&maze).unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
