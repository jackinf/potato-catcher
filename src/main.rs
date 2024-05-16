pub mod components;
pub mod resources;
pub mod systems;

use crate::resources::potato_spawn_timer::PotatoSpawnTimer;
use crate::resources::score::Score;
use crate::systems::check_if_potato_caught_by_potato_man::check_if_potato_caught_by_potato_man;
use crate::systems::potato_falling::potato_falling;
use crate::systems::potato_man_perform_movement::potato_man_perform_movement;
use crate::systems::potato_man_record_movement::potato_man_record_movement;
use crate::systems::setup::setup;
use crate::systems::spawn_potato::spawn_potato;
use bevy::prelude::*;

const POTATO_MAN_SPEED: f32 = 800.0;
const POTATO_SPEED: f32 = 300.0;

fn main() {
    App::new()
        .insert_resource(PotatoSpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(Score(0))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_potato)
        .add_systems(Update, potato_man_record_movement)
        .add_systems(FixedUpdate, potato_man_perform_movement)
        .add_systems(FixedUpdate, potato_falling)
        .add_systems(FixedUpdate, check_if_potato_caught_by_potato_man)
        .run();
}
