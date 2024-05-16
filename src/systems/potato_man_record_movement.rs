use crate::components::potato_man::PotatoMan;
use crate::components::potato_man_direction::PotatoManDirection;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::{EventReader, KeyCode, Query, With};

pub fn potato_man_record_movement(
    mut q_potato_man: Query<&mut PotatoManDirection, With<PotatoMan>>,
    mut key_button_events: EventReader<KeyboardInput>,
) {
    let mut direction = q_potato_man.single_mut();

    for key_button_event in key_button_events.read() {
        direction.x = 0.0;
        if let ButtonState::Pressed = key_button_event.state {
            if let KeyCode::ArrowLeft | KeyCode::KeyA = key_button_event.key_code {
                direction.x = -1.0;
            }

            if let KeyCode::ArrowRight | KeyCode::KeyD = key_button_event.key_code {
                direction.x = 1.0;
            }
        }
    }
}
