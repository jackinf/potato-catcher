use crate::components::potato::Potato;
use crate::PotatoSpawnTimer;
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Query, Res, ResMut, SpriteBundle, Time, Transform, Window};
use rand::Rng;

pub fn spawn_potato(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PotatoSpawnTimer>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // Get the primary window
        let window = windows.single();

        // Generate a random x position
        let mut rng = rand::thread_rng();
        let x_position = rng.gen_range(0.0..window.width()) - (window.width() / 2.0);

        // Spawn the potato at the top of the window
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/potato.png"),
                transform: Transform {
                    translation: Vec3::new(x_position, window.height() / 2.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Potato,
        ));

        // Reset the timer with a new random duration
        let duration = rng.gen_range(1.0..5.0);
        timer
            .0
            .set_duration(std::time::Duration::from_secs_f32(duration));
        timer.0.reset();
    }
}
