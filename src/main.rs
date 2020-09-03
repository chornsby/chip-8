use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

mod display;
mod emulator;
mod keyboard;
mod memory;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Chip-8", 640, 320)
        .position_centered()
        .build()
        .expect("Could not create a window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;

    let rom = std::fs::read("roms/BLINKY").expect("Unable to read rom");
    let mut emulator = emulator::Emulator::new(&rom);
    let mut display = display::Display::default();
    let keyboard = keyboard::Keyboard::default();

    let target_frame_time = Duration::from_secs(1) / 60;

    'is_running: loop {
        let frame_start = Instant::now();

        // Input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'is_running,
                _ => {}
            }
        }

        // Update at 480Hz
        for _ in 0..8 {
            emulator.tick(&mut display, &keyboard);
        }
        emulator.decrement_timers();

        // Render at 60Hz
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        for x in 0..display::WIDTH {
            for y in 0..display::HEIGHT {
                if display.get_pixel(x, y) {
                    canvas.fill_rect(Rect::new(x as i32 * 10, y as i32 * 10, 10, 10))?;
                }
            }
        }

        canvas.present();

        // Framerate
        let frame_end = Instant::now();
        let frame_time = frame_end - frame_start;

        if frame_time < target_frame_time {
            std::thread::sleep(target_frame_time - frame_time);
        }
    }

    Ok(())
}
