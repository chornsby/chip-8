use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::convert::{TryFrom, TryInto};
use std::time::{Duration, Instant};

mod display;
mod emulator;
mod instruction;
mod keyboard;
mod memory;

impl TryFrom<Keycode> for keyboard::Key {
    type Error = ();

    /// Maps between modern keyboard keys and Chip-8 hex keys
    ///
    /// ```
    /// 1 2 3 4     1 2 3 C
    /// q w e r --> 4 5 6 D
    /// a s d f --> 7 8 9 E
    /// z x c v     A 0 B F
    /// ```
    fn try_from(value: Keycode) -> Result<Self, Self::Error> {
        match value {
            Keycode::Num1 => Ok(keyboard::Key::Num1),
            Keycode::Num2 => Ok(keyboard::Key::Num2),
            Keycode::Num3 => Ok(keyboard::Key::Num3),
            Keycode::Num4 => Ok(keyboard::Key::C),
            Keycode::Q => Ok(keyboard::Key::Num4),
            Keycode::W => Ok(keyboard::Key::Num5),
            Keycode::E => Ok(keyboard::Key::Num6),
            Keycode::R => Ok(keyboard::Key::D),
            Keycode::A => Ok(keyboard::Key::Num7),
            Keycode::S => Ok(keyboard::Key::Num8),
            Keycode::D => Ok(keyboard::Key::Num9),
            Keycode::F => Ok(keyboard::Key::E),
            Keycode::Z => Ok(keyboard::Key::A),
            Keycode::X => Ok(keyboard::Key::Num0),
            Keycode::C => Ok(keyboard::Key::B),
            Keycode::V => Ok(keyboard::Key::F),
            _ => Err(()),
        }
    }
}

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
    let mut keyboard = keyboard::Keyboard::default();

    let target_frame_time = Duration::from_secs(1) / 60;

    'is_running: loop {
        let frame_start = Instant::now();

        // Input
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        if let Ok(key) = keycode.try_into() {
                            keyboard.press(&key);
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        if let Ok(key) = keycode.try_into() {
                            keyboard.release(&key);
                        }
                    }
                }
                Event::Quit { .. } => break 'is_running,
                _ => {}
            }
        }

        // Update at 480Hz
        for _ in 0..8 {
            emulator.tick(&mut display, &keyboard)?;
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
