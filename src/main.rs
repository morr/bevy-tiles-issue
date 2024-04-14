use bevy::window::PresentMode;

use bevy_tiles_zoom::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Automatic,
                        mode: bevy::window::WindowMode::Windowed,
                        present_mode: PresentMode::AutoNoVsync,
                        // present_mode: PresentMode::AutoVsync,
                        resolution: (WW as f32, WH as f32).into(),
                        // title: "Test App".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((camera::CameraPlugin, map::MapPlugin, input::InputPlugin))
        .add_systems(FixedUpdate, bevy::window::close_on_esc)
        .run();
}
