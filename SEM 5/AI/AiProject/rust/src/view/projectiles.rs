//! Small glowing spheres for in-flight projectiles.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::app::GameRes;
use crate::view::assets::{SharedMeshes, VisualMaterials};
use crate::view::geometry::{PROJECTILE_Y, position_world};

#[derive(Component)]
pub struct ProjectileVisual {
    pub id: u64,
}

fn scale_for(kind: crate::game::TowerKind) -> f32 {
    match kind {
        crate::game::TowerKind::Basic => 1.0,
        crate::game::TowerKind::Sniper => 1.3,
        crate::game::TowerKind::Rapid => 0.8,
        crate::game::TowerKind::Cannon => 1.45,
        crate::game::TowerKind::Frost => 1.1,
        crate::game::TowerKind::Tesla => 0.95,
    }
}

pub fn sync_projectiles(
    mut commands: Commands,
    game: Res<GameRes>,
    meshes: Res<SharedMeshes>,
    materials: Res<VisualMaterials>,
    mut visuals: Query<(Entity, &ProjectileVisual, &mut Transform)>,
) {
    let mut existing: HashMap<u64, Entity> = visuals
        .iter()
        .map(|(entity, visual, _)| (visual.id, entity))
        .collect();

    for projectile in &game.projectiles {
        let translation = position_world(projectile.pos, PROJECTILE_Y);
        if let Some(&entity) = existing.get(&projectile.id) {
            if let Ok((_, _, mut transform)) = visuals.get_mut(entity) {
                transform.translation = translation;
            }
        } else {
            let entity = commands
                .spawn((
                    Mesh3d(meshes.sphere.clone()),
                    MeshMaterial3d(materials.projectiles[projectile.kind.index()].clone()),
                    Transform::from_translation(translation)
                        .with_scale(Vec3::splat(scale_for(projectile.kind))),
                    ProjectileVisual { id: projectile.id },
                ))
                .id();
            existing.insert(projectile.id, entity);
        }
    }

    for (entity, visual, _) in &visuals {
        if !game
            .projectiles
            .iter()
            .any(|projectile| projectile.id == visual.id)
        {
            commands.entity(entity).despawn();
        }
    }
}
