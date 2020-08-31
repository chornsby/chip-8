mod display;
mod emulator;
mod keyboard;

fn main() {
    let rom = std::fs::read("roms/BLINKY").expect("Unable to read rom");
    let mut emulator = emulator::Emulator::new(&rom);
    let mut display = display::Display::default();
    let keyboard = keyboard::Keyboard::default();

    loop {
        emulator.tick(&mut display, &keyboard);
    }
}
