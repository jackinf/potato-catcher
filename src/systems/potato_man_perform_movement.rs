use crate::components::potato_man::PotatoMan;
use crate::components::potato_man_direction::PotatoManDirection;
use crate::POTATO_MAN_SPEED;
use bevy::math::Vec3;
use bevy::prelude::{Query, Res, Time, Transform, With};

pub fn potato_man_perform_movement(
    time: Res<Time>,
    mut q_potato_man: Query<(&mut Transform, &PotatoManDirection), With<PotatoMan>>,
) {
    let (mut transform, direction) = q_potato_man.single_mut();
    let dir = Vec3::new(direction.x, 0., 0.);

    transform.translation += dir * POTATO_MAN_SPEED * time.delta_seconds();
}
