use avian2d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Setup, Self::spawn_player);
    }
}

impl PlayerPlugin {
    fn spawn_player(mut commands: Commands) {
        commands.spawn((
            RigidBody::Dynamic,
            Collider::rectangle(20., 100.),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }
}
