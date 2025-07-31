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
            PhysicsPlugins::default().with_length_unit(20.),
        ))
        .add_plugins((gmtk::SetupPlugin, gmtk::player::PlayerPlugin,gmtk::environment::EnvironmentPlugin, gmtk::camera::CameraPlugin))
        .insert_resource(Gravity(Vec2::NEG_Y * 1000.))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
