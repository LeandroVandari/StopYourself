use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Stop yourself".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            PhysicsPlugins::default(),
        ))
        .add_plugins(gmtk::player::PlayerPlugin)
        .insert_resource(Gravity(Vec2::NEG_Y * 100.))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
