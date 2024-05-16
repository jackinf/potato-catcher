use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct PotatoSpawnTimer(pub Timer);
