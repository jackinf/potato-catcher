use crate::components::potato::Potato;
use crate::components::potato_man::PotatoMan;
use bevy::prelude::{AssetServer, AudioBundle, Commands, default, Entity, Query, Res, Transform, Vec2, With, Without};

pub fn check_if_potato_caught_by_potato_man(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_potato_man: Query<&mut Transform, (With<PotatoMan>, Without<Potato>)>,
    q_potatoes: Query<(Entity, &Transform), (With<Potato>, Without<PotatoMan>)>,
) {
    let potato_man = q_potato_man.single();
    let potato_man_size = Vec2::new(130.0, 150.0);
    let potato_size = Vec2::new(60.0, 80.0);

    // check if potato_man intersects with any potato
    q_potatoes
        .iter()
        .filter_map(|(entity, potato_transform)| {
            if intersects(
                potato_man.translation.truncate(),
                potato_man_size,
                potato_transform.translation.truncate(),
                potato_size,
            ) {
                Some(entity)
            } else {
                None
            }
        })
        .for_each(|entity| {
            commands.entity(entity).despawn();

            let sound = if rand::random() { "sounds/potato.ogg" } else { "sounds/harvest.ogg" };

            commands.spawn(AudioBundle {
                source: asset_server.load(sound),
                ..default()
            });
        });
}

fn intersects(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
    let half_size1 = size1 / 2.0;
    let half_size2 = size2 / 2.0;

    pos1.x - half_size1.x < pos2.x + half_size2.x
        && pos1.x + half_size1.x > pos2.x - half_size2.x
        && pos1.y - half_size1.y < pos2.y + half_size2.y
        && pos1.y + half_size1.y > pos2.y - half_size2.y
}
