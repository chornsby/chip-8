mod command;

use command::Command;

struct Emulator {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [usize; 16],
    delay: u8,
    sound: u8,
    stack_pointer: usize,
}

impl Emulator {
    fn new() -> Self {
        Self {
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            delay: 0,
            sound: 0,
            stack_pointer: 0,
        }
    }
}

struct Program(Vec<Command>);

fn main() {
    let program = Program(vec![]);
    let mut emulator = Emulator::new();
}
