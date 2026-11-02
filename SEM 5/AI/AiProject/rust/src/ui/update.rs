//! Button handling plus per-frame HUD refresh.

use bevy::prelude::*;

use crate::app::GameRes;
use crate::game::{MAX_TOWER_LEVEL, TOTAL_WAVES};
use crate::ui::hud::{
    ButtonAction, ContextActions, ContextName, ContextStats, CoreBarFill, CoreText, EnemiesText,
    GoldText, MessageText, OverlayRoot, OverlaySubtitle, OverlayTitle, PathLabel, SpeedLabel,
    StartLabel, UpgradeLabel, WaveText,
};
use crate::ui::theme;

pub fn handle_buttons(
    interactions: Query<(&Interaction, &ButtonAction), Changed<Interaction>>,
    mut game: ResMut<GameRes>,
) {
    for (interaction, action) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            ButtonAction::Select(kind) => game.select(*kind),
            ButtonAction::StartWave => game.start_wave(),
            ButtonAction::Regenerate => game.regenerate(),
            ButtonAction::Speed => game.toggle_speed(),
            ButtonAction::Path => game.toggle_path(),
            ButtonAction::Upgrade => game.upgrade_selected(),
            ButtonAction::Sell => game.sell_selected(),
        }
    }
}

pub fn refresh_cards(
    game: Res<GameRes>,
    mut buttons: Query<(&ButtonAction, &Interaction, &mut ImageNode), With<Button>>,
) {
    for (action, interaction, mut image) in &mut buttons {
        let hovered = !matches!(interaction, Interaction::None);
        let pressed = *interaction == Interaction::Pressed;

        image.color = match action {
            ButtonAction::Select(kind) => {
                let selected = game.selected == *kind;
                if selected {
                    if hovered {
                        Color::srgb(0.42, 0.7, 0.98)
                    } else {
                        theme::CARD_TINT_SELECTED
                    }
                } else if hovered {
                    theme::CARD_TINT_HOVER
                } else {
                    theme::CARD_TINT
                }
            }
            ButtonAction::StartWave => {
                if pressed {
                    Color::srgb(0.72, 0.82, 0.96)
                } else {
                    Color::WHITE
                }
            }
            ButtonAction::Regenerate | ButtonAction::Speed | ButtonAction::Path => {
                if pressed {
                    theme::BUTTON_TINT_PRESSED
                } else if hovered {
                    theme::BUTTON_TINT_HOVER
                } else {
                    theme::BUTTON_TINT
                }
            }
            ButtonAction::Upgrade | ButtonAction::Sell => {
                if pressed {
                    theme::BUTTON_TINT_PRESSED
                } else if hovered {
                    theme::BUTTON_TINT_HOVER
                } else {
                    theme::BUTTON_TINT
                }
            }
        };
    }
}

#[allow(clippy::type_complexity)]
pub fn refresh_text(
    game: Res<GameRes>,
    mut texts: Query<(
        &mut Text,
        &mut TextColor,
        Option<&GoldText>,
        Option<&CoreText>,
        Option<&WaveText>,
        Option<&EnemiesText>,
        Option<&MessageText>,
        Option<&StartLabel>,
        Option<&SpeedLabel>,
        Option<&PathLabel>,
        Option<&ContextName>,
        Option<&ContextStats>,
        Option<&UpgradeLabel>,
        Option<&OverlayTitle>,
        Option<&OverlaySubtitle>,
    )>,
) {
    for (
        mut text,
        mut color,
        gold,
        core,
        wave,
        enemies,
        message,
        start,
        speed,
        path,
        context_name,
        context_stats,
        upgrade,
        overlay_title,
        overlay_subtitle,
    ) in &mut texts
    {
        if gold.is_some() {
            text.0 = game.gold.to_string();
        } else if core.is_some() {
            text.0 = format!("{} / {}", game.core_hp, game.core_max_hp);
            color.0 = if game.core_hp as f32 / game.core_max_hp.max(1) as f32 > 0.5 {
                theme::GOOD
            } else if game.core_hp as f32 / game.core_max_hp.max(1) as f32 > 0.25 {
                theme::GOLD
            } else {
                theme::BAD
            };
        } else if wave.is_some() {
            text.0 = format!("{} / {}", game.wave, TOTAL_WAVES);
            color.0 = if game.wave_active {
                theme::GOOD
            } else {
                theme::ACCENT
            };
        } else if enemies.is_some() {
            let alive = game
                .enemies
                .iter()
                .filter(|enemy| enemy.alive && !enemy.leaked)
                .count();
            text.0 = alive.to_string();
        } else if message.is_some() {
            if game.message_timer > 0.0 {
                text.0 = game.message.clone();
                color.0 = theme::tone_color(game.message_tone);
            } else {
                text.0.clear();
            }
        } else if start.is_some() {
            text.0 = if game.wave_active {
                format!("WAVE {} RUNNING", game.wave)
            } else if game.victory {
                "ALL WAVES CLEARED".to_string()
            } else {
                "SEND WAVE  [Space]".to_string()
            };
            color.0 = if game.wave_active {
                theme::MUTED
            } else {
                theme::TEXT
            };
        } else if speed.is_some() {
            text.0 = format!("{}x", game.speed as i32);
        } else if path.is_some() {
            text.0 = if game.show_path {
                "PATH ON".to_string()
            } else {
                "PATH OFF".to_string()
            };
        } else if context_name.is_some() {
            if let Some(tower) = selected_tower(&game) {
                text.0 = format!(
                    "{}  ·  LV {}",
                    tower.kind.stats().name.to_uppercase(),
                    tower.level
                );
                color.0 = theme::tower_color(tower.kind);
            } else {
                text.0 = "BUILD A TOWER".to_string();
                color.0 = theme::ACCENT;
            }
        } else if context_stats.is_some() {
            if let Some(tower) = selected_tower(&game) {
                text.0 = format!(
                    "DMG {:.0}  RATE {:.2}/s  RANGE {:.1}\nINVESTED {}g  ·  SELL {}g",
                    tower.damage,
                    tower.fire_rate,
                    tower.range,
                    tower.invested,
                    tower.sell_value()
                );
            } else {
                text.0 = "Pick a card or press 1-6.\nClick a grass tile to build.".to_string();
            }
            color.0 = theme::MUTED;
        } else if upgrade.is_some() {
            if let Some(tower) = selected_tower(&game) {
                text.0 = match tower.upgrade_cost() {
                    Some(cost) => format!("UPGRADE  {}g", cost),
                    None => format!("MAX LEVEL {}", MAX_TOWER_LEVEL),
                };
            } else {
                text.0 = "UPGRADE".to_string();
            }
        } else if overlay_title.is_some() {
            text.0 = if game.victory { "VICTORY" } else { "DEFEAT" }.to_string();
            color.0 = if game.victory {
                theme::GOOD
            } else {
                theme::BAD
            };
        } else if overlay_subtitle.is_some() {
            text.0 = if game.victory {
                format!(
                    "All {} waves cleared with {} core integrity",
                    TOTAL_WAVES, game.core_hp
                )
            } else {
                format!("The core fell during wave {}", game.wave.max(1))
            };
        }
    }
}

fn selected_tower(game: &GameRes) -> Option<&crate::game::Tower> {
    game.selected_tower
        .and_then(|cell| game.towers.iter().find(|tower| tower.cell == cell))
}

pub fn refresh_core_bar(
    game: Res<GameRes>,
    mut bar: Query<(&mut Node, &mut BackgroundColor), With<CoreBarFill>>,
) {
    let ratio = (game.core_hp as f32 / game.core_max_hp.max(1) as f32).clamp(0.0, 1.0);
    let color = if ratio > 0.5 {
        theme::GOOD
    } else if ratio > 0.25 {
        theme::GOLD
    } else {
        theme::BAD
    };
    for (mut node, mut background) in &mut bar {
        node.width = percent(ratio * 100.0);
        background.0 = color;
    }
}

pub fn refresh_context_actions(
    game: Res<GameRes>,
    mut actions: Query<&mut Node, With<ContextActions>>,
) {
    for mut node in &mut actions {
        node.display = if game.selected_tower.is_some() {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn refresh_overlay(game: Res<GameRes>, mut overlay: Query<&mut Node, With<OverlayRoot>>) {
    let Ok(mut node) = overlay.single_mut() else {
        return;
    };
    node.display = if game.game_over || game.victory {
        Display::Flex
    } else {
        Display::None
    };
}
