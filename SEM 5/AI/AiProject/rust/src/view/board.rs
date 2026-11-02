//! Raised maze terrain, path preview, core, spawn gates and scenery.

use std::collections::HashSet;

use bevy::prelude::*;
use bevy::world_serialization::WorldAssetRoot;

use crate::app::GameRes;
use crate::game::{COLS, Cell, ROWS};
use crate::view::assets::{GameAssets, SharedMeshes, VisualMaterials, hash};
use crate::view::geometry::{FLOOR_TOP, GROUND_Y, HOVER_Y, MARKER_Y, PATH_Y, WALL_TOP, cell_world};
use crate::view::input::HoverCell;

#[derive(Component)]
pub struct BoardTile;

#[derive(Component)]
pub struct BoardMarker;

#[derive(Component)]
pub struct PathDot;

#[derive(Component)]
pub struct GateMarker {
    pub index: usize,
}

#[derive(Component)]
pub struct CoreOrb;

#[derive(Component)]
pub struct HoverTile;

#[derive(Component)]
pub struct RangeRing;

/// Rebuild all tiles, scenery and markers when a new level is generated.
#[allow(clippy::too_many_arguments)]
pub fn rebuild_board(
    mut commands: Commands,
    game: Res<GameRes>,
    assets: Res<GameAssets>,
    meshes: Res<SharedMeshes>,
    materials: Res<VisualMaterials>,
    mut last_version: Local<u64>,
    tiles: Query<Entity, With<BoardTile>>,
    markers: Query<Entity, With<BoardMarker>>,
) {
    if *last_version == game.level_version {
        return;
    }
    *last_version = game.level_version;

    for entity in &tiles {
        commands.entity(entity).despawn();
    }
    for entity in &markers {
        commands.entity(entity).despawn();
    }

    spawn_island(&mut commands, &assets, &meshes, &materials);

    for cell in game.grid.cells() {
        if game.grid.is_floor(cell) {
            spawn_floor_block(&mut commands, &meshes, &materials, cell);
            if !game.spawns.contains(&cell) && cell != game.core && hash(cell, 5) % 100 < 14 {
                spawn_detail(&mut commands, &assets, cell, 0, 0.32, FLOOR_TOP);
            }
        } else {
            spawn_wall_block(&mut commands, &game, &assets, &meshes, &materials, cell);
        }
    }

    spawn_core(&mut commands, &game, &assets, &meshes, &materials);
    spawn_gates(&mut commands, &game, &assets, &meshes, &materials);
    spawn_scenery(&mut commands, &assets);
}

fn spawn_island(
    commands: &mut Commands,
    assets: &GameAssets,
    meshes: &SharedMeshes,
    materials: &VisualMaterials,
) {
    // A two-cell grass apron around the maze hides the board edge.
    for y in -2..ROWS + 2 {
        for x in -2..COLS + 2 {
            let cell = Cell::new(x, y);
            if cell.in_bounds(COLS, ROWS) {
                continue;
            }
            let snow = x < 0 && y < 3;
            let material = if snow {
                materials.snow.clone()
            } else {
                materials.grass_dark.clone()
            };
            commands.spawn((
                Mesh3d(meshes.cube.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(cell_world(cell, GROUND_Y + 0.02))
                    .with_scale(Vec3::new(0.98, 0.08, 0.98)),
                BoardTile,
            ));
            if snow && hash(cell, 15) % 100 < 35 {
                spawn_detail(commands, assets, cell, 0, 0.35, GROUND_Y + 0.06);
            } else if hash(cell, 17) % 100 < 18 {
                spawn_detail(commands, assets, cell, 0, 0.3, GROUND_Y + 0.06);
            }
        }
    }

    // Forest ring, snow patch and a small river pond.
    for y in -9..ROWS + 9 {
        for x in -9..COLS + 9 {
            let cell = Cell::new(x, y);
            if (-2..COLS + 2).contains(&x) && (-2..ROWS + 2).contains(&y) {
                continue;
            }
            if x < 0 && y < 3 {
                if hash(cell, 21) % 100 < 24 {
                    spawn_detail(commands, assets, cell, 0, 0.55, GROUND_Y + 0.05);
                }
                continue;
            }
            if x > COLS && (3..10).contains(&y) {
                commands.spawn((
                    Mesh3d(meshes.cube.clone()),
                    MeshMaterial3d(materials.water.clone()),
                    Transform::from_translation(cell_world(cell, GROUND_Y + 0.03))
                        .with_scale(Vec3::new(0.99, 0.04, 0.99)),
                    BoardTile,
                ));
                if hash(cell, 27) % 100 < 12 {
                    spawn_detail(commands, assets, cell, 0, 0.4, GROUND_Y + 0.05);
                }
                continue;
            }
            let roll = hash(cell, 29) % 100;
            if roll < 7 {
                spawn_detail(commands, assets, cell, 0, 0.7, GROUND_Y + 0.04);
            } else if roll < 10 {
                spawn_large_detail(commands, assets, cell, 0.85, GROUND_Y + 0.04);
            } else if roll < 11 {
                let structure =
                    assets.structures[hash(cell, 37) as usize % assets.structures.len()].clone();
                commands.spawn((
                    WorldAssetRoot(structure),
                    Transform::from_translation(cell_world(cell, GROUND_Y + 0.04)).with_rotation(
                        Quat::from_rotation_y((hash(cell, 39) % 628) as f32 / 100.0),
                    ),
                    BoardTile,
                ));
            }
        }
    }
}

fn spawn_floor_block(
    commands: &mut Commands,
    meshes: &SharedMeshes,
    materials: &VisualMaterials,
    cell: Cell,
) {
    commands.spawn((
        Mesh3d(meshes.cube.clone()),
        MeshMaterial3d(materials.dirt.clone()),
        Transform::from_translation(cell_world(cell, FLOOR_TOP * 0.5))
            .with_scale(Vec3::new(0.99, FLOOR_TOP, 0.99)),
        BoardTile,
    ));
    commands.spawn((
        Mesh3d(meshes.cube.clone()),
        MeshMaterial3d(materials.grass.clone()),
        Transform::from_translation(cell_world(cell, FLOOR_TOP + 0.02))
            .with_scale(Vec3::new(0.99, 0.04, 0.99)),
        BoardTile,
    ));
}

fn spawn_wall_block(
    commands: &mut Commands,
    game: &GameRes,
    assets: &GameAssets,
    meshes: &SharedMeshes,
    materials: &VisualMaterials,
    cell: Cell,
) {
    commands.spawn((
        Mesh3d(meshes.cube.clone()),
        MeshMaterial3d(materials.dirt.clone()),
        Transform::from_translation(cell_world(cell, WALL_TOP * 0.5))
            .with_scale(Vec3::new(0.99, WALL_TOP, 0.99)),
        BoardTile,
    ));
    commands.spawn((
        Mesh3d(meshes.cube.clone()),
        MeshMaterial3d(materials.grass_dark.clone()),
        Transform::from_translation(cell_world(cell, WALL_TOP + 0.02))
            .with_scale(Vec3::new(0.99, 0.04, 0.99)),
        BoardTile,
    ));
    commands.spawn((
        WorldAssetRoot(assets.walls[GameAssets::wall_for(cell)].clone()),
        Transform::from_translation(cell_world(cell, WALL_TOP + 0.02)),
        BoardTile,
    ));
    if hash(cell, 11) % 100 < 55 {
        spawn_detail(commands, assets, cell, 0, 0.42, WALL_TOP + 0.04);
    }
    if game.spawns.contains(&cell) {
        // Gates sit in the wall line but still need a visible opening.
        commands.spawn((
            Mesh3d(meshes.cube.clone()),
            MeshMaterial3d(materials.path_floor.clone()),
            Transform::from_translation(cell_world(cell, WALL_TOP + 0.05))
                .with_scale(Vec3::new(0.7, 0.03, 0.7)),
            BoardTile,
        ));
    }
}

fn spawn_core(
    commands: &mut Commands,
    game: &GameRes,
    assets: &GameAssets,
    meshes: &SharedMeshes,
    materials: &VisualMaterials,
) {
    let core = game.core;
    commands.spawn((
        WorldAssetRoot(assets.core_base.clone()),
        Transform::from_translation(cell_world(core, FLOOR_TOP)).with_scale(Vec3::splat(1.45)),
        BoardMarker,
    ));
    commands.spawn((
        WorldAssetRoot(assets.core_crystals.clone()),
        Transform::from_translation(cell_world(core, FLOOR_TOP + 0.05))
            .with_scale(Vec3::splat(1.1)),
        BoardMarker,
    ));
    commands.spawn((
        Mesh3d(meshes.ring.clone()),
        MeshMaterial3d(materials.core.clone()),
        Transform::from_translation(cell_world(core, FLOOR_TOP + 0.06))
            .with_scale(Vec3::splat(1.3)),
        BoardMarker,
    ));
    commands.spawn((
        Mesh3d(meshes.sphere.clone()),
        MeshMaterial3d(materials.core.clone()),
        Transform::from_translation(cell_world(core, FLOOR_TOP + 2.0)).with_scale(Vec3::splat(7.0)),
        CoreOrb,
        BoardMarker,
    ));
    for (x, z) in [(-0.78, -0.78), (0.78, -0.78), (-0.78, 0.78), (0.78, 0.78)] {
        commands.spawn((
            Mesh3d(meshes.cylinder.clone()),
            MeshMaterial3d(materials.stone.clone()),
            Transform::from_translation(cell_world(core, FLOOR_TOP + 0.55) + Vec3::new(x, 0.0, z))
                .with_scale(Vec3::new(0.16, 1.05, 0.16)),
            BoardMarker,
        ));
    }
}

fn spawn_gates(
    commands: &mut Commands,
    game: &GameRes,
    assets: &GameAssets,
    meshes: &SharedMeshes,
    materials: &VisualMaterials,
) {
    for (index, &cell) in game.spawns.iter().enumerate() {
        commands.spawn((
            WorldAssetRoot(assets.spawn_tile.clone()),
            Transform::from_translation(cell_world(cell, FLOOR_TOP + 0.03)),
            BoardMarker,
        ));
        commands.spawn((
            Mesh3d(meshes.ring.clone()),
            MeshMaterial3d(materials.gate_idle.clone()),
            Transform::from_translation(cell_world(cell, MARKER_Y)).with_scale(Vec3::splat(0.62)),
            GateMarker { index },
            BoardMarker,
        ));
        commands.spawn((
            Mesh3d(meshes.cylinder.clone()),
            MeshMaterial3d(materials.gate_idle.clone()),
            Transform::from_translation(cell_world(cell, FLOOR_TOP + 1.35))
                .with_scale(Vec3::new(0.34, 2.6, 0.34)),
            GateMarker { index },
            BoardMarker,
        ));
    }
}

fn spawn_scenery(commands: &mut Commands, assets: &GameAssets) {
    for (x, y, scale) in [
        (-3, 2, 1.2),
        (-4, 12, 1.0),
        (COLS + 2, 1, 1.1),
        (COLS + 3, ROWS - 2, 1.25),
        (5, -4, 1.0),
        (COLS - 4, ROWS + 3, 1.15),
    ] {
        let cell = Cell::new(x, y);
        let structure =
            assets.structures[hash(cell, 41) as usize % assets.structures.len()].clone();
        commands.spawn((
            WorldAssetRoot(structure),
            Transform::from_translation(cell_world(cell, GROUND_Y + 0.04))
                .with_rotation(Quat::from_rotation_y((hash(cell, 43) % 628) as f32 / 100.0))
                .with_scale(Vec3::splat(scale)),
            BoardTile,
        ));
    }
}

fn spawn_detail(
    commands: &mut Commands,
    assets: &GameAssets,
    cell: Cell,
    index: u32,
    scale: f32,
    y: f32,
) {
    let seed = hash(cell, 20 + index);
    let model = assets.details[(seed as usize) % assets.details.len()].clone();
    let dx = ((hash(cell, 40 + index) % 60) as f32 / 100.0) - 0.3;
    let dz = ((hash(cell, 60 + index) % 60) as f32 / 100.0) - 0.3;
    let rotation = (hash(cell, 80 + index) % 628) as f32 / 100.0;
    let size = scale * (0.8 + (hash(cell, 100 + index) % 45) as f32 / 100.0);
    commands.spawn((
        WorldAssetRoot(model),
        Transform::from_translation(cell_world(cell, y) + Vec3::new(dx, 0.0, dz))
            .with_rotation(Quat::from_rotation_y(rotation))
            .with_scale(Vec3::splat(size)),
        BoardTile,
    ));
}

fn spawn_large_detail(
    commands: &mut Commands,
    assets: &GameAssets,
    cell: Cell,
    scale: f32,
    y: f32,
) {
    let seed = hash(cell, 51);
    let model = assets.large_details[(seed as usize) % assets.large_details.len()].clone();
    let rotation = (hash(cell, 53) % 628) as f32 / 100.0;
    commands.spawn((
        WorldAssetRoot(model),
        Transform::from_translation(cell_world(cell, y))
            .with_rotation(Quat::from_rotation_y(rotation))
            .with_scale(Vec3::splat(scale)),
        BoardTile,
    ));
}

/// Rebuild the dotted A* preview only when the route visibility changes.
pub fn rebuild_path_preview(
    mut commands: Commands,
    game: Res<GameRes>,
    meshes: Res<SharedMeshes>,
    materials: Res<VisualMaterials>,
    mut last: Local<(u64, bool, u32, bool)>,
    dots: Query<Entity, With<PathDot>>,
) {
    let key = (
        game.towers_version,
        game.show_path,
        game.wave,
        game.wave_active,
    );
    if *last == key {
        return;
    }
    *last = key;

    for entity in &dots {
        commands.entity(entity).despawn();
    }

    if !game.show_path {
        return;
    }

    let mut seen = HashSet::new();
    for path in &game.preview_paths {
        for &cell in path {
            if !seen.insert(cell) {
                continue;
            }
            if cell != game.core && !game.spawns.contains(&cell) {
                commands.spawn((
                    Mesh3d(meshes.cube.clone()),
                    MeshMaterial3d(materials.path_floor.clone()),
                    Transform::from_translation(cell_world(cell, FLOOR_TOP + 0.045))
                        .with_scale(Vec3::new(0.86, 0.025, 0.86)),
                    PathDot,
                ));
            }
            commands.spawn((
                Mesh3d(meshes.cube.clone()),
                MeshMaterial3d(materials.path.clone()),
                Transform::from_translation(cell_world(cell, PATH_Y))
                    .with_scale(Vec3::new(0.14, 0.03, 0.14)),
                PathDot,
            ));
        }
    }
}

pub fn update_gates(
    game: Res<GameRes>,
    materials: Res<VisualMaterials>,
    mut gates: Query<(&GateMarker, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    for (gate, mut material) in &mut gates {
        let active = game
            .active_spawns
            .iter()
            .any(|cell| *cell == game.spawns[gate.index]);
        *material = MeshMaterial3d(if active {
            materials.gate_active.clone()
        } else {
            materials.gate_idle.clone()
        });
    }
}

pub fn animate_core(time: Res<Time>, mut orb: Query<&mut Transform, With<CoreOrb>>) {
    let pulse = 4.6 + (time.elapsed_secs() * 2.2).sin() * 0.35;
    for mut transform in &mut orb {
        transform.scale = Vec3::splat(pulse);
        transform.rotate_local_y(time.delta_secs() * 0.8);
    }
}

pub fn update_hover_tile(
    game: Res<GameRes>,
    hover: Res<HoverCell>,
    materials: Res<VisualMaterials>,
    mut tile: Query<
        (
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
            &mut Visibility,
        ),
        With<HoverTile>,
    >,
) {
    let Ok((mut transform, mut material, mut visibility)) = tile.single_mut() else {
        return;
    };

    let Some(cell) = hover.0.filter(|cell| game.grid.in_bounds(*cell)) else {
        *visibility = Visibility::Hidden;
        return;
    };

    let valid = !game.game_over && !game.victory && game.placement_error(cell).is_ok();
    transform.translation = cell_world(cell, HOVER_Y);
    transform.scale = Vec3::new(0.94, 0.02, 0.94);
    *material = MeshMaterial3d(if valid {
        materials.hover_ok.clone()
    } else {
        materials.hover_bad.clone()
    });
    *visibility = Visibility::Visible;
}

/// Ring showing the selected tower's range, or the build range while placing.
pub fn update_range_ring(
    game: Res<GameRes>,
    hover: Res<HoverCell>,
    mut ring: Query<(&mut Transform, &mut Visibility), With<RangeRing>>,
) {
    let Ok((mut transform, mut visibility)) = ring.single_mut() else {
        return;
    };

    let selected_range = game
        .selected_tower
        .and_then(|cell| game.towers.iter().find(|tower| tower.cell == cell))
        .map_or(game.selected.stats().range, |tower| tower.range);

    let cell = hover.0.filter(|cell| {
        !game.game_over
            && !game.victory
            && game.grid.in_bounds(*cell)
            && (game.placement_error(*cell).is_ok() || game.has_tower(*cell))
    });

    let Some(cell) = cell else {
        *visibility = Visibility::Hidden;
        return;
    };

    transform.translation = cell_world(cell, HOVER_Y + 0.01);
    transform.scale = Vec3::splat(selected_range);
    *visibility = Visibility::Visible;
}
