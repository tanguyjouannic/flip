use bevy::prelude::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);
    }
}

#[derive(Component)]
struct GameMarker;

fn setup() {}

fn cleanup(entities: Query<Entity, With<GameMarker>>, mut commands: Commands) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}