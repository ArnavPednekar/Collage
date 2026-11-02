//! Keyboard shortcuts and mouse interaction with the board.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::app::GameRes;
use crate::game::{Cell, TowerKind};
use crate::ui::{BOTTOM_BAR_HEIGHT, TOP_BAR_HEIGHT};
use crate::view::geometry::{FLOOR_TOP, world_cell};

/// Cell currently under the cursor, if any. Read by the hover systems.
#[derive(Resource, Default)]
pub struct HoverCell(pub Option<Cell>);

pub fn keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<GameRes>,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if keys.just_pressed(KeyCode::Digit1) {
        game.select(TowerKind::Basic);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        game.select(TowerKind::Sniper);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        game.select(TowerKind::Rapid);
    }
    if keys.just_pressed(KeyCode::Digit4) {
        game.select(TowerKind::Cannon);
    }
    if keys.just_pressed(KeyCode::Digit5) {
        game.select(TowerKind::Frost);
    }
    if keys.just_pressed(KeyCode::Digit6) {
        game.select(TowerKind::Tesla);
    }
    if keys.just_pressed(KeyCode::KeyU) {
        game.upgrade_selected();
    }
    if keys.just_pressed(KeyCode::KeyX) {
        game.sell_selected();
    }
    if keys.just_pressed(KeyCode::Space) {
        game.start_wave();
    }
    if keys.just_pressed(KeyCode::KeyR) {
        game.regenerate();
    }
    if keys.just_pressed(KeyCode::KeyF) {
        game.toggle_speed();
    }
    if keys.just_pressed(KeyCode::KeyP) {
        game.toggle_path();
    }
}

pub fn board_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut game: ResMut<GameRes>,
    mut hover: ResMut<HoverCell>,
) {
    let (camera, camera_transform) = *camera;

    let Some(cursor) = window.cursor_position() else {
        hover.0 = None;
        return;
    };

    let over_board = cursor.y > TOP_BAR_HEIGHT
        && cursor.y < window.height() - BOTTOM_BAR_HEIGHT
        && cursor.x >= 0.0
        && cursor.x < window.width();

    let cell = over_board
        .then(|| {
            camera
                .viewport_to_world(camera_transform, cursor)
                .ok()
                .and_then(|ray| {
                    ray.plane_intersection_point(
                        Vec3::new(0.0, FLOOR_TOP, 0.0),
                        InfinitePlane3d::default(),
                    )
                })
                .and_then(world_cell)
        })
        .flatten();

    hover.0 = cell;

    if game.game_over || game.victory {
        return;
    }
    let Some(cell) = cell else {
        return;
    };

    if buttons.just_pressed(MouseButton::Left) {
        if game.has_tower(cell) {
            game.select_tower(cell);
        } else {
            game.try_place(cell);
        }
    } else if buttons.just_pressed(MouseButton::Right) {
        game.sell_tower(cell);
    }
}
