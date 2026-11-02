//! UFO models and floating health bars for every live enemy.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy::world_serialization::WorldAssetRoot;

use crate::app::GameRes;
use crate::game::EnemyKind;
use crate::view::assets::{GameAssets, SharedMeshes, VisualMaterials};
use crate::view::geometry::{ENEMY_Y, HEALTH_BAR_Y, position_world};

const BAR_WIDTH: f32 = 0.8;

#[derive(Component)]
pub struct EnemyVisual {
    pub id: u64,
}

#[derive(Component)]
pub struct HealthBar {
    pub id: u64,
}

fn model_index(kind: EnemyKind) -> usize {
    match kind {
        EnemyKind::Grunt => 0,
        EnemyKind::Runner => 1,
        EnemyKind::Tank => 2,
        EnemyKind::Boss => 3,
    }
}

fn model_scale(kind: EnemyKind) -> f32 {
    match kind {
        EnemyKind::Grunt => 1.0,
        EnemyKind::Runner => 0.85,
        EnemyKind::Tank => 1.1,
        EnemyKind::Boss => 1.3,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn sync_enemies(
    mut commands: Commands,
    game: Res<GameRes>,
    assets: Res<GameAssets>,
    meshes: Res<SharedMeshes>,
    materials: Res<VisualMaterials>,
    time: Res<Time>,
    mut visuals: Query<(Entity, &EnemyVisual, &mut Transform)>,
    mut bars: Query<
        (
            Entity,
            &HealthBar,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<EnemyVisual>,
    >,
) {
    let mut visual_entities: HashMap<u64, Entity> = visuals
        .iter()
        .map(|(entity, visual, _)| (visual.id, entity))
        .collect();
    let mut bar_entities: HashMap<u64, Entity> = bars
        .iter()
        .map(|(entity, bar, _, _)| (bar.id, entity))
        .collect();

    let mut alive = HashSet::new();
    let now = time.elapsed_secs();

    for enemy in &game.enemies {
        alive.insert(enemy.id);
        let bob = (now * 4.0 + enemy.id as f32).sin() * 0.03;
        let rotation = Quat::from_rotation_y(enemy.travelled * 2.0 + enemy.id as f32);
        let translation = position_world(enemy.pos, ENEMY_Y + bob);

        if let Some(&entity) = visual_entities.get(&enemy.id) {
            if let Ok((_, _, mut transform)) = visuals.get_mut(entity) {
                transform.translation = translation;
                transform.rotation = rotation;
            }
        } else {
            let model = assets.enemies[model_index(enemy.kind)].clone();
            let entity = commands
                .spawn((
                    WorldAssetRoot(model),
                    Transform::from_translation(translation)
                        .with_rotation(rotation)
                        .with_scale(Vec3::splat(model_scale(enemy.kind))),
                    EnemyVisual { id: enemy.id },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Mesh3d(meshes.ring.clone()),
                        MeshMaterial3d(materials.enemy_markers[model_index(enemy.kind)].clone()),
                        Transform::from_xyz(0.0, -0.48, 0.0).with_scale(Vec3::splat(0.34)),
                    ));
                })
                .id();
            visual_entities.insert(enemy.id, entity);
        }

        let ratio = (enemy.hp / enemy.max_hp.max(1) as f32).clamp(0.0, 1.0);
        let mut bar_world = position_world(enemy.pos, HEALTH_BAR_Y);
        bar_world.x += -BAR_WIDTH * 0.5 + BAR_WIDTH * 0.5 * ratio;
        let color_index = (ratio * 10.0).round() as usize;

        if let Some(&entity) = bar_entities.get(&enemy.id) {
            if let Ok((_, _, mut transform, mut material)) = bars.get_mut(entity) {
                transform.translation = bar_world;
                transform.scale = Vec3::new((BAR_WIDTH * ratio).max(0.001), 0.06, 0.08);
                *material = MeshMaterial3d(materials.health[color_index].clone());
            }
        } else {
            let entity = commands
                .spawn((
                    Mesh3d(meshes.cube.clone()),
                    MeshMaterial3d(materials.health[color_index].clone()),
                    Transform::from_translation(bar_world).with_scale(Vec3::new(
                        (BAR_WIDTH * ratio).max(0.001),
                        0.06,
                        0.08,
                    )),
                    HealthBar { id: enemy.id },
                ))
                .id();
            bar_entities.insert(enemy.id, entity);
        }
    }

    for (entity, visual, _) in &visuals {
        if !alive.contains(&visual.id) {
            commands.entity(entity).despawn();
        }
    }
    for (entity, bar, _, _) in &bars {
        if !alive.contains(&bar.id) {
            commands.entity(entity).despawn();
        }
    }
}
