use super::*;
use bevy_pancam::PanCam;
use bevy_pancam::PanCamPlugin;

#[derive(Component, Debug)]
pub struct FloorCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera2dBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                projection: OrthographicProjection {
                    // don't forget to set `near` and `far`
                    near: -1000.0,
                    far: 1000.0,
                    // initial zoom
                    scale: 1.25,
                    // ... any other settings you want to change ...
                    ..default()
                },
                ..default()
            },
            Name::new("main_camera"),
            FloorCamera,
        ))
        .insert(PanCam {
            enabled: true,
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
            max_scale: Some(20.0),
            max_x: None,
            max_y: None,
            min_scale: 0.1, // 0.5,
            min_x: None,
            min_y: None,
            zoom_to_cursor: true,
        });
}
