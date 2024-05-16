use crate::components::potato_man::PotatoMan;
use crate::components::potato_man_direction::PotatoManDirection;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::{EventReader, KeyCode, Local, Query, With};
use std::collections::HashSet;

pub fn potato_man_record_movement(
    mut q_potato_man: Query<&mut PotatoManDirection, With<PotatoMan>>,
    mut key_button_events: EventReader<KeyboardInput>,
    mut pressed_keys: Local<HashSet<KeyCode>>,
) {
    let mut direction = q_potato_man.single_mut();

    for key_button_event in key_button_events.read() {
        match key_button_event.state {
            ButtonState::Pressed => {
                pressed_keys.insert(key_button_event.key_code);
            }
            ButtonState::Released => {
                pressed_keys.remove(&key_button_event.key_code);
            }
        }
    }

    direction.x = 0.0;
    if pressed_keys.contains(&KeyCode::KeyA) || pressed_keys.contains(&KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if pressed_keys.contains(&KeyCode::KeyD) || pressed_keys.contains(&KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
}
