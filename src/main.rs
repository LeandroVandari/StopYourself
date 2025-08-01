use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Stop yourself".into(),
                    resolution: WindowResolution::new(1000., 600.),
                    present_mode: PresentMode::AutoNoVsync,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            PhysicsPlugins::default().with_length_unit(20.),
            bevy_framepace::FramepacePlugin,
        ))
        .add_plugins((
            gmtk::SetupPlugin,
            gmtk::player::PlayerPlugin,
            gmtk::environment::EnvironmentPlugin,
            gmtk::camera::CameraPlugin,
            gmtk::modes::ModesManagement,
            gmtk::obstacles::ObstaclePlugin,
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 1000.))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
