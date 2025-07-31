use bevy::prelude::*;
use avian2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Setup, Self::spawn_player)
    }
}

impl PlayerPlugin {
    fn spawn_player(mut commands: Commands) {
        commands.spawn((
            RigidBody::Dynamic
        ));
    }
}