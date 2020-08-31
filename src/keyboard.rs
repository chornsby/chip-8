/// Stores the current pressed state of the Chip-8 hex keys
#[derive(Default)]
pub struct Keyboard {
    pressed: [bool; 16],
}
