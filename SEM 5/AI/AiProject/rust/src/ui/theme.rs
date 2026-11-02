//! Shared palette and text sizes, ported from the 2D prototype.

use bevy::prelude::*;

use crate::game::{EnemyKind, MessageTone, TowerKind};

pub const BG: Color = Color::srgb_u8(14, 16, 24);
pub const PANEL: Color = Color::srgb_u8(22, 25, 37);
pub const PANEL_LIGHT: Color = Color::srgb_u8(31, 35, 51);
pub const PANEL_EDGE: Color = Color::srgb_u8(48, 54, 76);
pub const TEXT: Color = Color::srgb_u8(222, 228, 242);
pub const MUTED: Color = Color::srgb_u8(128, 138, 162);
pub const ACCENT: Color = Color::srgb_u8(80, 200, 255);
pub const PATH: Color = Color::srgb_u8(62, 150, 205);
pub const GOOD: Color = Color::srgb_u8(95, 220, 145);
pub const BAD: Color = Color::srgb_u8(240, 95, 105);
pub const GOLD: Color = Color::srgb_u8(250, 205, 90);
pub const SELECTED_FILL: Color = Color::srgb_u8(40, 52, 74);
pub const START_FILL: Color = Color::srgb_u8(28, 62, 82);

/// Multipliers applied to the white/grey Kenney UI button textures.
pub const PANEL_TINT: Color = Color::srgb(0.18, 0.22, 0.31);
pub const STAT_TINT: Color = Color::srgb(0.24, 0.28, 0.38);
pub const CARD_TINT: Color = Color::srgb(0.30, 0.34, 0.44);
pub const CARD_TINT_HOVER: Color = Color::srgb(0.42, 0.47, 0.59);
pub const CARD_TINT_SELECTED: Color = Color::srgb(0.22, 0.44, 0.66);
pub const BUTTON_TINT: Color = Color::srgb(0.42, 0.47, 0.58);
pub const BUTTON_TINT_HOVER: Color = Color::srgb(0.58, 0.64, 0.76);
pub const BUTTON_TINT_PRESSED: Color = Color::srgb(0.28, 0.32, 0.42);

pub const TITLE_SIZE: f32 = 22.0;
pub const SUBTITLE_SIZE: f32 = 12.0;
pub const STAT_LABEL_SIZE: f32 = 10.0;
pub const STAT_VALUE_SIZE: f32 = 20.0;
pub const SECTION_SIZE: f32 = 11.0;
pub const CARD_NAME_SIZE: f32 = 16.0;
pub const CARD_COST_SIZE: f32 = 15.0;
pub const CARD_STATS_SIZE: f32 = 10.0;
pub const BUTTON_SIZE: f32 = 14.0;
pub const SMALL_SIZE: f32 = 12.0;
pub const MESSAGE_SIZE: f32 = 13.0;
pub const OVERLAY_TITLE_SIZE: f32 = 54.0;
pub const OVERLAY_SUBTITLE_SIZE: f32 = 18.0;
pub const OVERLAY_HINT_SIZE: f32 = 15.0;

pub fn tone_color(tone: MessageTone) -> Color {
    match tone {
        MessageTone::Info => TEXT,
        MessageTone::Accent => ACCENT,
        MessageTone::Good => GOOD,
        MessageTone::Bad => BAD,
        MessageTone::Gold => GOLD,
        MessageTone::Muted => MUTED,
    }
}

pub fn tower_color(kind: TowerKind) -> Color {
    match kind {
        TowerKind::Basic => Color::srgb_u8(80, 180, 255),
        TowerKind::Sniper => Color::srgb_u8(190, 120, 255),
        TowerKind::Rapid => Color::srgb_u8(255, 160, 70),
        TowerKind::Cannon => Color::srgb_u8(235, 100, 60),
        TowerKind::Frost => Color::srgb_u8(90, 220, 255),
        TowerKind::Tesla => Color::srgb_u8(250, 220, 80),
    }
}

pub fn enemy_color(kind: EnemyKind) -> Color {
    match kind {
        EnemyKind::Grunt => Color::srgb_u8(235, 90, 90),
        EnemyKind::Runner => Color::srgb_u8(240, 140, 220),
        EnemyKind::Tank => Color::srgb_u8(200, 70, 60),
        EnemyKind::Boss => Color::srgb_u8(255, 60, 120),
    }
}

/// Green at full health, red at zero.
pub fn health_color(ratio: f32) -> Color {
    let ratio = ratio.clamp(0.0, 1.0);
    let lerp = |from: u8, to: u8| (from as f32 + (to as f32 - from as f32) * ratio).round() as u8;
    Color::srgb_u8(lerp(240, 95), lerp(95, 220), lerp(105, 145))
}
