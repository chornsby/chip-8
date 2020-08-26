use bevy::ecs::{Res, ResMut};
use bevy::input::keyboard::KeyCode;
use bevy::input::Input;

/// Stores the current pressed state of the Chip-8 hex keys
#[derive(Default)]
pub struct KeyboardState {
    pressed: [bool; 16],
}

/// Reads keyboard input from Bevy resources into our own state
///
/// We map QWERTY keyboard keys to the relevant hexadecimal keys using the
/// following layout:
///
/// PC Keyboard -->  Chip-8 Keyboard
///
/// 1  2  3  4       1  2  3  C
/// Q  W  E  R  -->  4  5  6  D
/// A  S  D  F  -->  7  8  9  E
/// Z  X  C  V       A  0  B  F
pub fn read_input_system(mut keyboard: ResMut<KeyboardState>, input: Res<Input<KeyCode>>) {
    keyboard.pressed[0] = input.pressed(KeyCode::X);
    keyboard.pressed[1] = input.pressed(KeyCode::Key1);
    keyboard.pressed[2] = input.pressed(KeyCode::Key2);
    keyboard.pressed[3] = input.pressed(KeyCode::Key3);
    keyboard.pressed[4] = input.pressed(KeyCode::Q);
    keyboard.pressed[5] = input.pressed(KeyCode::W);
    keyboard.pressed[6] = input.pressed(KeyCode::E);
    keyboard.pressed[7] = input.pressed(KeyCode::A);
    keyboard.pressed[8] = input.pressed(KeyCode::S);
    keyboard.pressed[9] = input.pressed(KeyCode::D);
    keyboard.pressed[10] = input.pressed(KeyCode::Z);
    keyboard.pressed[11] = input.pressed(KeyCode::C);
    keyboard.pressed[12] = input.pressed(KeyCode::Key4);
    keyboard.pressed[13] = input.pressed(KeyCode::R);
    keyboard.pressed[14] = input.pressed(KeyCode::F);
    keyboard.pressed[15] = input.pressed(KeyCode::V);
}
