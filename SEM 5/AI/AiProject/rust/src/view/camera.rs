//! Isometric camera framing, zoom and distance fog.

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::light::ShadowFilteringMethod;
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;

const VERTICAL_FOV_DEGREES: f32 = 40.0;
pub const WINDOW_WIDTH: u32 = 1600;
pub const WINDOW_HEIGHT: u32 = 900;

const BASE_OFFSET: Vec3 = Vec3::new(7.0, 22.0, 24.0);
const MIN_DISTANCE: f32 = 24.0;
const MAX_DISTANCE: f32 = 44.0;

/// Smoothed zoom state for the follow camera.
#[derive(Resource)]
pub struct CameraRig {
    pub distance: f32,
    pub target_distance: f32,
}

/// Point the camera orbits around: the middle of the raised board.
pub fn focus_point() -> Vec3 {
    Vec3::new(0.0, -2.0, 0.0)
}

pub fn setup_camera(mut commands: Commands) {
    let distance = BASE_OFFSET.length();
    commands.insert_resource(CameraRig {
        distance,
        target_distance: distance,
    });

    let focus = focus_point();
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: VERTICAL_FOV_DEGREES.to_radians(),
            near: 0.1,
            far: 300.0,
            ..default()
        }),
        DistanceFog {
            color: Color::srgb_u8(34, 52, 64),
            falloff: FogFalloff::Linear {
                start: 46.0,
                end: 96.0,
            },
            ..default()
        },
        ShadowFilteringMethod::Gaussian,
        Transform::from_translation(focus + BASE_OFFSET).looking_at(focus, Vec3::Y),
    ));
}

/// Scroll wheel and +/- keys move the camera along its view axis.
pub fn zoom_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<MouseWheel>,
    mut rig: ResMut<CameraRig>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    let mut scroll = 0.0;
    for event in wheel.read() {
        scroll += match event.unit {
            MouseScrollUnit::Line => event.y * 1.4,
            MouseScrollUnit::Pixel => event.y / 24.0,
        };
    }
    let keyboard = time.delta_secs();
    if keys.pressed(KeyCode::Equal) {
        scroll += keyboard * 2.0;
    }
    if keys.pressed(KeyCode::Minus) {
        scroll -= keyboard * 2.0;
    }
    if scroll == 0.0 && (rig.distance - rig.target_distance).abs() < 0.001 {
        return;
    }

    rig.target_distance = (rig.target_distance - scroll * 1.6).clamp(MIN_DISTANCE, MAX_DISTANCE);
    let blend = 1.0 - (-12.0 * time.delta_secs()).exp();
    rig.distance += (rig.target_distance - rig.distance) * blend;
    if (rig.distance - rig.target_distance).abs() < 0.001 {
        rig.distance = rig.target_distance;
    }

    let focus = focus_point();
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    *transform = Transform::from_translation(focus + BASE_OFFSET.normalize() * rig.distance)
        .looking_at(focus, Vec3::Y);
}
