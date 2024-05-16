//! Renders a 2D scene containing a single, moving sprite.

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use rand::Rng;

const POTATO_MAN_SPEED: f32 = 500.0;
const POTATO_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_potato)
        .add_systems(FixedUpdate, (potato_man_movement, potato_falling))
        .run();
}

#[derive(Component)]
struct PotatoMan;

#[derive(Component)]
struct Potato;

#[derive(Resource)]
struct SpawnTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn potato_man_movement(
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

fn spawn_potato(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
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

fn potato_falling(
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
