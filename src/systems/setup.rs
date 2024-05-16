use crate::components::potato_man::PotatoMan;
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{default, Camera2dBundle, Commands, Res, SpriteBundle, Transform};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/potato_man.png"),
            transform: Transform::default()
                .with_scale(Vec3::splat(0.2))
                .with_translation(Vec3::new(0., -300., 10.)),
            ..default()
        },
        PotatoMan,
    ));
}
