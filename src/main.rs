use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::window::WindowMode;

mod keyboard;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_resource(WindowDescriptor {
            width: 640,
            height: 320,
            title: "Chip-8 Emulator".to_string(),
            vsync: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<keyboard::KeyboardState>()
        .add_system(keyboard::read_input_system.system())
        .add_default_plugins()
        .run();
}
