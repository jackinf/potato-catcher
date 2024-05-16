use crate::components::potato::Potato;
use crate::POTATO_SPEED;
use bevy::prelude::{Commands, Entity, Query, Res, Time, Transform, Window, With};

pub fn potato_falling(
    mut commands: Commands,
    time: Res<Time>,
    mut q_potatoes: Query<(Entity, &mut Transform), With<Potato>>,
    windows: Query<&Window>,
) {
    let dt = time.delta_seconds();

    for (entity, mut transform) in &mut q_potatoes.iter_mut() {
        let window = windows.single();
        let window_height = window.height();

        transform.translation.y -= POTATO_SPEED * dt;

        if transform.translation.y < -window_height / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}
