use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{
    default, Camera2dBundle, Commands, PositionType, Res, SpriteBundle, Style, TextBundle,
    TextStyle, Transform, Val,
};

use crate::components::potato_man::PotatoMan;
use crate::components::potato_man_direction::PotatoManDirection;
use crate::components::score_text::ScoreText;

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
        PotatoManDirection { x: 0. },
    ));

    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/AmericanCaptain.ttf"),
                font_size: 30.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));
}
