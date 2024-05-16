use crate::components::potato_man::PotatoMan;
use crate::POTATO_MAN_SPEED;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::math::Vec3;
use bevy::prelude::{EventReader, KeyCode, Query, Res, Time, Transform, With};

pub fn potato_man_movement(
    time: Res<Time>,
    mut q_potato_man: Query<&mut Transform, With<PotatoMan>>,
    mut key_button_events: EventReader<KeyboardInput>,
) {
    let mut transform = q_potato_man.single_mut();
    let mut direction = Vec3::ZERO;

    for key_button_event in key_button_events.read() {
        if let ButtonState::Pressed = key_button_event.state {
            if key_button_event.key_code == KeyCode::ArrowLeft {
                direction.x -= 1.0;
            }

            if key_button_event.key_code == KeyCode::ArrowRight {
                direction.x += 1.0;
            }
        }
    }

    transform.translation += direction * POTATO_MAN_SPEED * time.delta_seconds();
}
