//! Application wiring: window, resources and system schedule.

use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode, WindowResolution};

use crate::game::Game;
use crate::ui::{self, theme};
use crate::view::{self, input::HoverCell};

/// Bevy wrapper around the pure simulation.
#[derive(Resource, bevy::prelude::Deref, bevy::prelude::DerefMut)]
pub struct GameRes(pub Game);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AI Tower Defense".into(),
                resolution: WindowResolution::new(
                    view::camera::WINDOW_WIDTH,
                    view::camera::WINDOW_HEIGHT,
                ),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(theme::BG))
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 350.0,
            affects_lightmapped_meshes: false,
        })
        .insert_resource(GameRes(Game::new()))
        .init_resource::<HoverCell>()
        .add_systems(
            Startup,
            (
                view::assets::setup_visuals,
                view::camera::setup_camera,
                view::daynight::setup_sun,
                ui::hud::setup_hud,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                view::input::keyboard,
                view::input::board_mouse,
                ui::update::handle_buttons,
                advance,
                view::camera::zoom_camera,
                view::daynight::update_day_night,
                view::board::rebuild_board,
                view::board::rebuild_path_preview,
                view::board::update_gates,
                view::board::animate_core,
                view::board::update_hover_tile,
                view::board::update_range_ring,
                view::towers::sync_towers,
                view::towers::aim_towers,
                view::enemies::sync_enemies,
                view::projectiles::sync_projectiles,
            )
                .chain(),
        )
        .add_systems(
            PostUpdate,
            (
                ui::update::refresh_cards,
                ui::update::refresh_text,
                ui::update::refresh_core_bar,
                ui::update::refresh_context_actions,
                ui::update::refresh_overlay,
            )
                .chain(),
        );
    }
}

fn advance(time: Res<Time>, mut game: ResMut<GameRes>) {
    game.update(time.delta_secs());
}

pub fn run() {
    App::new().add_plugins(GamePlugin).run();
}
