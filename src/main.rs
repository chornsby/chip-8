use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod display;
mod emulator;
mod keyboard;

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

    'is_running: loop {
        // Input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'is_running,
                _ => {}
            }
        }

        // Update
        emulator.tick(&mut display, &keyboard);

        // Render
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);
        for (y, &row) in display.pixels.iter().enumerate() {
            for (x, &is_lit) in row.iter().enumerate() {
                if is_lit {
                    canvas.fill_rect(Rect::new(x as i32 * 10, y as i32 * 10, 10, 10))?;
                }
            }
        }

        canvas.present();
    }

    Ok(())
}
