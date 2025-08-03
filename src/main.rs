use avian2d::prelude::*;
use bevy::{prelude::*, window::PresentMode};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Stop yourself".into(),
                    present_mode: PresentMode::AutoNoVsync,
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            PhysicsPlugins::default().with_length_unit(20.),
            bevy_framepace::FramepacePlugin,
            PhysicsDebugPlugin::default(),
        ))
        .add_plugins((
            gmtk::SetupPlugin,
            gmtk::player::PlayerPlugin,
            gmtk::environment::EnvironmentPlugin,
            gmtk::camera::CameraPlugin,
            gmtk::modes::ModesManagement,
            gmtk::obstacles::ObstaclePlugin,
            gmtk::menu::MenuPlugin,
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 1000.))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
