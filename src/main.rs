use std::{
    convert::{TryFrom, TryInto},
    time::{Duration, Instant},
};

use sdl2::{
    audio::{AudioCallback, AudioSpecDesired, AudioStatus},
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
};

mod display;
mod emulator;
mod instruction;
mod keyboard;
mod memory;

const SCALE: usize = 20;
const TARGET_FRAME_TIME: Duration = Duration::from_millis(16);

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

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
    let audio_subsystem = sdl_context.audio()?;
    let video_subsystem = sdl_context.video()?;

    let audio_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let audio_device = audio_subsystem.open_playback(None, &audio_spec, |spec| SquareWave {
        phase_inc: 440.0 / spec.freq as f32,
        phase: 0.0,
        volume: 0.5,
    })?;

    let window = video_subsystem
        .window(
            "Chip-8",
            (display::WIDTH * SCALE) as u32,
            (display::HEIGHT * SCALE) as u32,
        )
        .position_centered()
        .build()
        .expect("Could not create a window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;

    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let rom = std::fs::read(path).expect("Unable to read rom");

    let mut emulator = emulator::Emulator::new(&rom);
    let mut display = display::Display::default();
    let mut keyboard = keyboard::Keyboard::default();

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

        emulator.decrement_timers();

        // Update at 500Hz
        for _ in 0..8 {
            emulator.tick(&mut display, &keyboard)?;
        }

        // Sound
        if emulator.is_sound_playing() && audio_device.status() != AudioStatus::Playing {
            audio_device.resume();
        } else if !emulator.is_sound_playing() && audio_device.status() != AudioStatus::Paused {
            audio_device.pause();
        }

        // Render at 62.5Hz
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        for x in 0..display::WIDTH {
            for y in 0..display::HEIGHT {
                if display.get_pixel(x, y) {
                    canvas.fill_rect(Rect::new(
                        (x * SCALE) as i32,
                        (y * SCALE) as i32,
                        SCALE as u32,
                        SCALE as u32,
                    ))?;
                }
            }
        }

        canvas.present();

        // Framerate
        let elapsed = frame_start.elapsed();

        if elapsed < TARGET_FRAME_TIME {
            std::thread::sleep(TARGET_FRAME_TIME - elapsed);
        }
    }

    Ok(())
}
