//! Day/night cycle: the sun orbits the island, changing shadows and colours.

use std::f32::consts::{PI, TAU};

use bevy::light::CascadeShadowConfigBuilder;
use bevy::prelude::*;

use crate::view::camera::focus_point;

/// Seconds for one full day/night rotation.
const CYCLE_SECONDS: f32 = 150.0;

#[derive(Component)]
pub struct Sun;

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

pub fn setup_sun(mut commands: Commands) {
    let focus = focus_point();
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.96, 0.88),
            illuminance: 16_000.0,
            shadow_maps_enabled: true,
            shadow_depth_bias: 0.03,
            shadow_normal_bias: 2.2,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.1,
            maximum_distance: 70.0,
            first_cascade_far_bound: 18.0,
            overlap_proportion: 0.2,
        }
        .build(),
        Transform::from_translation(focus + Vec3::new(18.0, 30.0, 14.0)).looking_at(focus, Vec3::Y),
        Sun,
    ));
}

/// Advance the cycle using real time, so it keeps moving between waves.
pub fn update_day_night(
    time: Res<Time>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut sun: Query<(&mut DirectionalLight, &mut Transform), With<Sun>>,
) {
    let angle = (time.elapsed_secs() / CYCLE_SECONDS) * TAU + 0.55;
    let elevation = angle.sin();
    let daylight = ((elevation + 0.18) / 1.18).clamp(0.0, 1.0);
    let horizon = (1.0 - elevation.abs() / 0.4).clamp(0.0, 1.0);

    let day = [1.0, 0.96, 0.88];
    let dusk = [1.0, 0.55, 0.3];
    let night = [0.42, 0.58, 1.0];
    let light_rgb = if elevation >= 0.0 {
        mix(day, dusk, horizon * 0.85)
    } else {
        mix(night, dusk, horizon * 0.55)
    };
    let light_color = Color::srgb(light_rgb[0], light_rgb[1], light_rgb[2]);
    let illuminance = if elevation >= 0.0 {
        1_000.0 + 16_000.0 * elevation.sqrt()
    } else {
        1_800.0 + 1_500.0 * -elevation
    };

    // Keep the light above the horizon even at "night" so it acts as moonlight.
    let above = elevation.abs().max(0.16);
    let azimuth = if elevation >= 0.0 { angle } else { angle + PI };
    let focus = focus_point();
    let position = focus
        + Vec3::new(
            azimuth.cos() * 30.0,
            12.0 + 24.0 * above,
            14.0 + azimuth.sin() * 12.0,
        );

    if let Ok((mut light, mut transform)) = sun.single_mut() {
        light.color = light_color;
        light.illuminance = illuminance;
        *transform = Transform::from_translation(position).looking_at(focus, Vec3::Y);
    }

    ambient.brightness = 140.0 + 80.0 * daylight;
    ambient.color = Color::srgb(
        0.55 + 0.45 * daylight,
        0.62 + 0.33 * daylight,
        0.9 - 0.1 * daylight,
    );

    let sky = mix([8.0, 12.0, 28.0], [18.0, 36.0, 58.0], daylight);
    let sky = mix(sky, [52.0, 26.0, 34.0], horizon * 0.6);
    clear.0 = Color::srgb_u8(sky[0] as u8, sky[1] as u8, sky[2] as u8);
}
