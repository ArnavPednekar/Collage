//! Kenney model handles plus meshes and materials shared by all visuals.

use bevy::color::Alpha;
use bevy::material::AlphaMode;
use bevy::prelude::*;
use bevy::world_serialization::WorldAsset;

use crate::game::{COLS, ROWS, TowerKind};
use crate::ui::theme;
use crate::view::board::{HoverTile, RangeRing};
use crate::view::geometry::{GROUND_Y, HOVER_Y};

const MODELS: &str = "kenney/models";

fn model(asset_server: &AssetServer, name: &str) -> Handle<WorldAsset> {
    asset_server.load(format!("{MODELS}/{name}.glb#Scene0"))
}

/// glTF scenes and materials for the Kenney Tower Defense Kit.
#[derive(Resource, Clone)]
pub struct GameAssets {
    pub tile: Handle<WorldAsset>,
    pub dirt_tile: Handle<WorldAsset>,
    pub snow_tile: Handle<WorldAsset>,
    pub spawn_tile: Handle<WorldAsset>,
    pub goal_tile: Handle<WorldAsset>,
    /// Wall-top props: rocks, trees, crystals, hills and snow variants.
    pub walls: Vec<Handle<WorldAsset>>,
    /// Small props sprinkled around the island.
    pub details: Vec<Handle<WorldAsset>>,
    /// Larger props for the forest ring.
    pub large_details: Vec<Handle<WorldAsset>>,
    /// All twelve build-stage tower bodies.
    pub tower_bodies: Vec<Handle<WorldAsset>>,
    /// Turret, ballista, cannon and catapult.
    pub tower_weapons: Vec<Handle<WorldAsset>>,
    pub core_base: Handle<WorldAsset>,
    pub core_crystals: Handle<WorldAsset>,
    pub structures: Vec<Handle<WorldAsset>>,
    pub river_tiles: Vec<Handle<WorldAsset>>,
    pub enemies: [Handle<WorldAsset>; 4],
}

impl GameAssets {
    pub fn load(asset_server: &AssetServer) -> Self {
        let names = |prefix: &str, suffix: &[&'static str]| {
            suffix
                .iter()
                .map(|name| model(asset_server, &format!("{prefix}{name}")))
                .collect()
        };
        let builds = [
            "tower-round-build-a",
            "tower-round-build-b",
            "tower-round-build-c",
            "tower-round-build-d",
            "tower-round-build-e",
            "tower-round-build-f",
            "tower-square-build-a",
            "tower-square-build-b",
            "tower-square-build-c",
            "tower-square-build-d",
            "tower-square-build-e",
            "tower-square-build-f",
        ];
        Self {
            tile: model(asset_server, "tile"),
            dirt_tile: model(asset_server, "tile-dirt"),
            snow_tile: model(asset_server, "snow-tile"),
            spawn_tile: model(asset_server, "tile-spawn-round"),
            goal_tile: model(asset_server, "tile-end-round"),
            walls: names(
                "tile-",
                &[
                    "rock",
                    "tree",
                    "tree-double",
                    "tree-quad",
                    "crystal",
                    "hill",
                    "bump",
                ],
            ),
            details: names(
                "detail-",
                &[
                    "rocks",
                    "rocks-large",
                    "crystal",
                    "crystal-large",
                    "tree",
                    "tree-large",
                    "dirt",
                    "dirt-large",
                ],
            ),
            large_details: {
                let mut all = names(
                    "tile-",
                    &[
                        "rock",
                        "tree",
                        "tree-double",
                        "tree-quad",
                        "crystal",
                        "hill",
                    ],
                );
                all.extend(names(
                    "snow-tile-",
                    &[
                        "rock",
                        "tree",
                        "tree-double",
                        "tree-quad",
                        "crystal",
                        "hill",
                    ],
                ));
                all
            },
            tower_bodies: builds
                .iter()
                .map(|name| model(asset_server, name))
                .collect(),
            tower_weapons: names("weapon-", &["turret", "ballista", "cannon", "catapult"]),
            core_base: model(asset_server, "tower-round-build-f"),
            core_crystals: model(asset_server, "tower-round-crystals"),
            structures: names(
                "",
                &[
                    "wood-structure",
                    "wood-structure-high",
                    "wood-structure-part",
                    "wood-structure-high-part",
                    "snow-wood-structure",
                    "snow-wood-structure-high",
                ],
            ),
            river_tiles: names(
                "tile-",
                &[
                    "river-straight",
                    "river-corner",
                    "river-bridge",
                    "river-transition",
                    "river-slope",
                ],
            ),
            enemies: [
                model(asset_server, "enemy-ufo-a"),
                model(asset_server, "enemy-ufo-b"),
                model(asset_server, "enemy-ufo-c"),
                model(asset_server, "enemy-ufo-d"),
            ],
        }
    }

    /// Build-stage body for a tower kind and level.
    pub fn tower_body(&self, kind: TowerKind, level: u8) -> Handle<WorldAsset> {
        let offset = usize::from(level.saturating_sub(1).min(1));
        let index = (kind.index() * 2 + offset).min(self.tower_bodies.len() - 1);
        self.tower_bodies[index].clone()
    }

    pub fn tower_weapon(&self, kind: TowerKind) -> Handle<WorldAsset> {
        let index = match kind {
            TowerKind::Basic | TowerKind::Rapid => 0,
            TowerKind::Sniper => 1,
            TowerKind::Cannon => 2,
            TowerKind::Frost | TowerKind::Tesla => 3,
        };
        self.tower_weapons[index].clone()
    }

    pub fn wall_for(cell: crate::game::Cell) -> usize {
        (hash(cell, 31) as usize) % 7
    }
}

/// Primitive meshes reused by markers, bars and projectiles.
#[derive(Resource, Clone)]
pub struct SharedMeshes {
    pub cube: Handle<Mesh>,
    pub ring: Handle<Mesh>,
    pub range_ring: Handle<Mesh>,
    pub sphere: Handle<Mesh>,
    pub cylinder: Handle<Mesh>,
}

/// Materials shared by terrain, overlays and tiny props.
#[derive(Resource, Clone)]
pub struct VisualMaterials {
    pub hover_ok: Handle<StandardMaterial>,
    pub hover_bad: Handle<StandardMaterial>,
    pub path: Handle<StandardMaterial>,
    pub range: Handle<StandardMaterial>,
    pub gate_active: Handle<StandardMaterial>,
    pub gate_idle: Handle<StandardMaterial>,
    pub grass: Handle<StandardMaterial>,
    pub grass_dark: Handle<StandardMaterial>,
    pub dirt: Handle<StandardMaterial>,
    pub path_floor: Handle<StandardMaterial>,
    pub snow: Handle<StandardMaterial>,
    pub water: Handle<StandardMaterial>,
    pub wood: Handle<StandardMaterial>,
    pub stone: Handle<StandardMaterial>,
    pub core: Handle<StandardMaterial>,
    pub projectiles: [Handle<StandardMaterial>; 6],
    pub enemy_markers: [Handle<StandardMaterial>; 4],
    pub health: Vec<Handle<StandardMaterial>>,
}

/// Stable hash used to pick decorations without any per-frame randomness.
pub fn hash(cell: crate::game::Cell, salt: u32) -> u32 {
    let mut value = (cell.x as u32).wrapping_mul(0x9E37_79B1)
        ^ (cell.y as u32).wrapping_mul(0x85EB_CA77)
        ^ salt.wrapping_mul(0xC2B2_AE3D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x2C1B_3C6D);
    value ^= value >> 12;
    value
}

pub fn setup_visuals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let unlit = |color: Color| StandardMaterial {
        base_color: color,
        unlit: true,
        ..default()
    };
    let transparent = |color: Color| StandardMaterial {
        base_color: color,
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    };
    let solid = |color: Color, roughness: f32| StandardMaterial {
        base_color: color,
        perceptual_roughness: roughness,
        ..default()
    };

    let health = (0..=10)
        .map(|step| {
            let ratio = step as f32 / 10.0;
            materials.add(unlit(theme::health_color(ratio)))
        })
        .collect();

    let mut projectile_materials: [Handle<StandardMaterial>; 6] =
        std::array::from_fn(|_| Handle::default());
    for kind in TowerKind::ALL {
        projectile_materials[kind.index()] = materials.add(unlit(theme::tower_color(kind)));
    }
    let enemy_markers: [Handle<StandardMaterial>; 4] = std::array::from_fn(|index| {
        let kind = crate::game::EnemyKind::ALL[index];
        materials.add(transparent(theme::enemy_color(kind).with_alpha(0.8)))
    });

    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let ring = meshes.add(Torus::new(0.86, 1.0));
    let range_ring = meshes.add(Torus::new(0.97, 1.0));
    let sphere = meshes.add(Sphere::new(0.12));
    let cylinder = meshes.add(Cylinder::new(0.5, 1.0));

    let hover_ok = materials.add(transparent(theme::GOOD.with_alpha(0.45)));
    let hover_bad = materials.add(transparent(theme::BAD.with_alpha(0.45)));
    let path = materials.add(transparent(theme::ACCENT.with_alpha(0.9)));
    let range = materials.add(transparent(theme::ACCENT.with_alpha(0.32)));
    let gate_active = materials.add(StandardMaterial {
        base_color: theme::ACCENT,
        emissive: LinearRgba::rgb(0.15, 0.8, 1.0),
        unlit: true,
        ..default()
    });
    let gate_idle = materials.add(StandardMaterial {
        base_color: Color::srgba(0.25, 0.35, 0.42, 0.55),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let grass = materials.add(solid(Color::srgb_u8(91, 157, 74), 1.0));
    let grass_dark = materials.add(solid(Color::srgb_u8(61, 118, 59), 1.0));
    let dirt = materials.add(solid(Color::srgb_u8(112, 79, 48), 1.0));
    let path_floor = materials.add(solid(Color::srgb_u8(176, 143, 92), 0.95));
    let snow = materials.add(solid(Color::srgb_u8(222, 236, 242), 0.9));
    let water = materials.add(StandardMaterial {
        base_color: Color::srgba(0.15, 0.46, 0.72, 0.88),
        perceptual_roughness: 0.25,
        metallic: 0.15,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let wood = materials.add(solid(Color::srgb_u8(133, 92, 56), 0.9));
    let stone = materials.add(solid(Color::srgb_u8(112, 120, 128), 0.85));
    let core = materials.add(StandardMaterial {
        base_color: Color::srgb_u8(80, 220, 255),
        emissive: LinearRgba::rgb(0.02, 0.22, 0.38),
        perceptual_roughness: 0.35,
        ..default()
    });

    commands.insert_resource(GameAssets::load(&asset_server));
    commands.insert_resource(SharedMeshes {
        cube: cube.clone(),
        ring,
        range_ring: range_ring.clone(),
        sphere,
        cylinder,
    });
    commands.insert_resource(VisualMaterials {
        hover_ok: hover_ok.clone(),
        hover_bad,
        path,
        range: range.clone(),
        gate_active,
        gate_idle,
        grass,
        grass_dark,
        dirt,
        path_floor,
        snow,
        water,
        wood,
        stone,
        core,
        projectiles: projectile_materials,
        enemy_markers,
        health,
    });

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(hover_ok),
        Transform::from_translation(Vec3::new(0.0, HOVER_Y, 0.0))
            .with_scale(Vec3::new(0.94, 0.02, 0.94)),
        Visibility::Hidden,
        HoverTile,
    ));

    commands.spawn((
        Mesh3d(range_ring),
        MeshMaterial3d(range),
        Transform::from_translation(Vec3::new(0.0, HOVER_Y, 0.0)),
        Visibility::Hidden,
        RangeRing,
    ));

    // Dark ground under the island so seams never show the clear colour.
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size((COLS + 24) as f32, (ROWS + 24) as f32),
            ),
        ),
        MeshMaterial3d(materials.add(solid(Color::srgb_u8(28, 55, 38), 0.98))),
        Transform::from_xyz(0.0, GROUND_Y - 0.04, 0.0),
        Ground,
    ));
}

#[derive(Component)]
pub struct Ground;
