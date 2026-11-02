//! Mirrors simulated towers into glTF body + weapon pairs.

use bevy::prelude::*;
use bevy::world_serialization::WorldAssetRoot;

use crate::app::GameRes;
use crate::game::Cell;
use crate::view::assets::{GameAssets, SharedMeshes, VisualMaterials};
use crate::view::geometry::{FLOOR_TOP, WEAPON_Y, aim_yaw, cell_world};

#[derive(Component)]
pub struct TowerVisual;

#[derive(Component)]
pub struct TowerWeapon {
    pub cell: Cell,
}

/// Rebuild tower models whenever the tower list changes.
#[allow(clippy::too_many_arguments)]
pub fn sync_towers(
    mut commands: Commands,
    game: Res<GameRes>,
    assets: Res<GameAssets>,
    meshes: Res<SharedMeshes>,
    materials: Res<VisualMaterials>,
    mut last_version: Local<u64>,
    visuals: Query<Entity, With<TowerVisual>>,
) {
    if *last_version == game.towers_version {
        return;
    }
    *last_version = game.towers_version;

    for entity in &visuals {
        commands.entity(entity).despawn();
    }

    for tower in &game.towers {
        let body = assets.tower_body(tower.kind, tower.level);
        let weapon = assets.tower_weapon(tower.kind);
        let cell = tower.cell;
        let level_scale = 1.0 + 0.06 * f32::from(tower.level.saturating_sub(1));

        commands
            .spawn((
                WorldAssetRoot(body),
                Transform::from_translation(cell_world(cell, FLOOR_TOP))
                    .with_scale(Vec3::splat(level_scale)),
                TowerVisual,
            ))
            .with_children(|parent| {
                parent.spawn((
                    WorldAssetRoot(weapon),
                    Transform::from_xyz(0.0, WEAPON_Y, 0.0),
                    TowerWeapon { cell },
                ));
                parent.spawn((
                    Mesh3d(meshes.ring.clone()),
                    MeshMaterial3d(materials.projectiles[tower.kind.index()].clone()),
                    Transform::from_xyz(0.0, 0.025, 0.0)
                        .with_scale(Vec3::splat(0.42 + 0.06 * f32::from(tower.level))),
                ));
                if tower.level > 1 {
                    parent.spawn((
                        Mesh3d(meshes.ring.clone()),
                        MeshMaterial3d(materials.projectiles[tower.kind.index()].clone()),
                        Transform::from_xyz(0.0, 0.08, 0.0)
                            .with_scale(Vec3::splat(0.3 + 0.07 * f32::from(tower.level))),
                    ));
                }
            });
    }
}

/// Point every weapon at its tower's current target.
pub fn aim_towers(game: Res<GameRes>, mut weapons: Query<(&TowerWeapon, &mut Transform)>) {
    for (weapon, mut transform) in &mut weapons {
        let Some(tower) = game.towers.iter().find(|tower| tower.cell == weapon.cell) else {
            continue;
        };
        transform.rotation = Quat::from_rotation_y(aim_yaw(tower.aim));
    }
}
