//! Top status bar, bottom build bar and the end-of-run overlay.

use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::game::TowerKind;
use crate::ui::assets::UiAssets;
use crate::ui::{BOTTOM_BAR_HEIGHT, TOP_BAR_HEIGHT, theme};

#[derive(Component, Clone, Copy)]
pub enum ButtonAction {
    Select(TowerKind),
    StartWave,
    Regenerate,
    Speed,
    Path,
    Upgrade,
    Sell,
}

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct CoreText;

#[derive(Component)]
pub struct CoreBarFill;

#[derive(Component)]
pub struct WaveText;

#[derive(Component)]
pub struct EnemiesText;

#[derive(Component)]
pub struct MessageText;

#[derive(Component)]
pub struct StartLabel;

#[derive(Component)]
pub struct SpeedLabel;

#[derive(Component)]
pub struct PathLabel;

#[derive(Component)]
pub struct ContextName;

#[derive(Component)]
pub struct ContextStats;

#[derive(Component)]
pub struct ContextActions;

#[derive(Component)]
pub struct UpgradeLabel;

#[derive(Component)]
pub struct SellLabel;

#[derive(Component)]
pub struct OverlayRoot;

#[derive(Component)]
pub struct OverlayTitle;

#[derive(Component)]
pub struct OverlaySubtitle;

fn bar_button(action: ButtonAction) -> impl Bundle {
    (
        Button,
        action,
        Interaction::None,
        FocusPolicy::Block,
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: px(6.0),
            padding: UiRect::new(px(10.0), px(10.0), px(8.0), px(8.0)),
            ..default()
        },
    )
}

fn card_button(action: ButtonAction) -> impl Bundle {
    (
        Button,
        action,
        Interaction::None,
        FocusPolicy::Block,
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            row_gap: px(3.0),
            padding: UiRect::all(px(8.0)),
            width: px(142.0),
            height: px(136.0),
            ..default()
        },
    )
}

fn stat_box(ui: &UiAssets) -> impl Bundle {
    (
        ui.sliced(&ui.grey_flat, 14.0, theme::STAT_TINT),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            row_gap: px(2.0),
            padding: UiRect::new(px(12.0), px(12.0), px(6.0), px(6.0)),
            min_width: px(96.0),
            ..default()
        },
    )
}

pub fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui = UiAssets::load(&asset_server);
    commands.insert_resource(ui.clone());
    let ui = &ui;

    // Top status bar -------------------------------------------------------
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(0.0),
                top: px(0.0),
                width: percent(100.0),
                height: px(TOP_BAR_HEIGHT),
                display: Display::Flex,
                align_items: AlignItems::Center,
                column_gap: px(10.0),
                padding: UiRect::new(px(16.0), px(16.0), px(8.0), px(8.0)),
                ..default()
            },
            ui.sliced(&ui.grey_depth, 18.0, theme::PANEL_TINT),
        ))
        .with_children(|bar| {
            bar.spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: px(240.0),
                row_gap: px(1.0),
                ..default()
            })
            .with_children(|brand| {
                brand.spawn(ui.title("AI TOWER DEFENSE", 20.0, theme::TEXT));
                brand.spawn(ui.label("PROCEDURAL MAZES  ·  A*  ·  RUST/BEVY", 9.0, theme::MUTED));
            });

            bar.spawn((
                Node {
                    width: px(2.0),
                    height: px(46.0),
                    ..default()
                },
                BackgroundColor(theme::PANEL_EDGE),
            ));

            // Gold.
            bar.spawn(stat_box(ui)).with_children(|box_| {
                box_.spawn(ui.label("GOLD", theme::STAT_LABEL_SIZE, theme::MUTED));
                box_.spawn(Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    column_gap: px(6.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn(ui.icon(&ui.star, 17.0, theme::GOLD));
                    row.spawn((
                        ui.title("--", theme::STAT_VALUE_SIZE, theme::TEXT),
                        GoldText,
                    ));
                });
            });

            // Core integrity + bar.
            bar.spawn((
                ui.sliced(&ui.grey_flat, 14.0, theme::STAT_TINT),
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    row_gap: px(3.0),
                    padding: UiRect::new(px(12.0), px(12.0), px(6.0), px(6.0)),
                    width: px(250.0),
                    ..default()
                },
            ))
            .with_children(|box_| {
                box_.spawn(ui.label("CORE INTEGRITY", theme::STAT_LABEL_SIZE, theme::MUTED));
                box_.spawn(Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    column_gap: px(7.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn(ui.icon(&ui.orb, 17.0, theme::GOOD));
                    row.spawn((ui.title("1000", 20.0, theme::TEXT), CoreText));
                    row.spawn(Node {
                        flex_grow: 1.0,
                        height: px(10.0),
                        padding: UiRect::all(px(1.0)),
                        ..default()
                    })
                    .insert(BackgroundColor(Color::srgb_u8(18, 22, 32)))
                    .with_children(|track| {
                        track.spawn((
                            Node {
                                width: percent(100.0),
                                height: percent(100.0),
                                ..default()
                            },
                            BackgroundColor(theme::GOOD),
                            CoreBarFill,
                        ));
                    });
                });
            });

            // Wave.
            bar.spawn(stat_box(ui)).with_children(|box_| {
                box_.spawn(ui.label("WAVE", theme::STAT_LABEL_SIZE, theme::MUTED));
                box_.spawn((ui.title("0 / 15", 20.0, theme::ACCENT), WaveText));
            });

            // Enemies.
            bar.spawn(stat_box(ui)).with_children(|box_| {
                box_.spawn(ui.label("ENEMIES", theme::STAT_LABEL_SIZE, theme::MUTED));
                box_.spawn((ui.title("0", 20.0, theme::TEXT), EnemiesText));
            });

            bar.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });

            bar.spawn((
                bar_button(ButtonAction::Speed),
                ui.sliced(&ui.grey_depth, 18.0, theme::BUTTON_TINT),
            ))
            .with_children(|button| {
                button.spawn((ui.title("1x", theme::BUTTON_SIZE, theme::TEXT), SpeedLabel));
            });

            bar.spawn((
                bar_button(ButtonAction::Path),
                ui.sliced(&ui.grey_depth, 18.0, theme::BUTTON_TINT),
            ))
            .with_children(|button| {
                button.spawn((ui.title("PATH", theme::BUTTON_SIZE, theme::TEXT), PathLabel));
            });

            bar.spawn((
                bar_button(ButtonAction::Regenerate),
                ui.sliced(&ui.grey_depth, 18.0, theme::BUTTON_TINT),
            ))
            .with_children(|button| {
                button.spawn(ui.icon(&ui.repeat, 15.0, theme::MUTED));
                button.spawn(ui.title("NEW MAZE", theme::BUTTON_SIZE, theme::TEXT));
            });
        });

    // Bottom build bar ----------------------------------------------------
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(0.0),
                bottom: px(0.0),
                width: percent(100.0),
                height: px(BOTTOM_BAR_HEIGHT),
                display: Display::Flex,
                align_items: AlignItems::Center,
                column_gap: px(8.0),
                padding: UiRect::new(px(14.0), px(14.0), px(9.0), px(9.0)),
                ..default()
            },
            ui.sliced(&ui.grey_depth, 18.0, theme::PANEL_TINT),
        ))
        .with_children(|bar| {
            for kind in TowerKind::ALL {
                let stats = kind.stats();
                bar.spawn((
                    card_button(ButtonAction::Select(kind)),
                    ui.sliced(&ui.grey_flat, 14.0, theme::CARD_TINT),
                ))
                .with_children(|card| {
                    card.spawn(Node {
                        display: Display::Flex,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        column_gap: px(4.0),
                        ..default()
                    })
                    .with_children(|top| {
                        top.spawn(Node {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            column_gap: px(5.0),
                            ..default()
                        })
                        .with_children(|name| {
                            name.spawn(ui.icon(&ui.square, 10.0, theme::tower_color(kind)));
                            name.spawn(ui.title(
                                stats.name.to_uppercase(),
                                theme::CARD_NAME_SIZE,
                                theme::tower_color(kind),
                            ));
                        });
                        top.spawn(ui.title(
                            format!("{}g", stats.cost),
                            theme::CARD_COST_SIZE,
                            theme::GOLD,
                        ));
                    });

                    card.spawn(ui.label(
                        format!("{:.0} dmg  ·  {:.2}/s", stats.damage, stats.fire_rate),
                        theme::CARD_STATS_SIZE,
                        Color::srgb_u8(210, 220, 235),
                    ));
                    card.spawn(ui.label(
                        if stats.splash > 0.0 {
                            format!("{:.1} range  ·  splash", stats.range)
                        } else if stats.slow < 1.0 {
                            format!("{:.1} range  ·  slows", stats.range)
                        } else if stats.chain > 0 {
                            format!("{:.1} range  ·  chains {}", stats.range, stats.chain)
                        } else {
                            format!("{:.1} range", stats.range)
                        },
                        theme::CARD_STATS_SIZE,
                        theme::MUTED,
                    ));

                    card.spawn((
                        ui.title(format!("[{}]", kind.index() + 1), 12.0, theme::MUTED),
                        Node {
                            align_self: AlignSelf::FlexEnd,
                            ..default()
                        },
                    ));
                });
            }

            // Selected tower panel.
            bar.spawn((
                ui.sliced(&ui.grey_flat, 14.0, theme::CARD_TINT),
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    row_gap: px(4.0),
                    padding: UiRect::all(px(9.0)),
                    width: px(250.0),
                    height: px(136.0),
                    ..default()
                },
            ))
            .with_children(|panel| {
                panel.spawn((ui.title("BUILD A TOWER", 15.0, theme::ACCENT), ContextName));
                panel.spawn((ui.label("", 10.0, theme::MUTED), ContextStats));
                panel
                    .spawn((
                        Node {
                            display: Display::Flex,
                            column_gap: px(6.0),
                            ..default()
                        },
                        ContextActions,
                    ))
                    .with_children(|actions| {
                        actions
                            .spawn((
                                bar_button(ButtonAction::Upgrade),
                                ui.sliced(&ui.blue_depth, 18.0, Color::WHITE),
                            ))
                            .with_children(|button| {
                                button
                                    .spawn((ui.title("UPGRADE", 11.0, theme::TEXT), UpgradeLabel));
                            });
                        actions
                            .spawn((
                                bar_button(ButtonAction::Sell),
                                ui.sliced(&ui.grey_depth, 18.0, theme::BUTTON_TINT),
                            ))
                            .with_children(|button| {
                                button.spawn((ui.title("SELL", 11.0, theme::TEXT), SellLabel));
                            });
                    });
            });

            // Wave controls and feedback.
            bar.spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                justify_content: JustifyContent::Center,
                row_gap: px(5.0),
                padding: UiRect::new(px(10.0), px(0.0), px(4.0), px(4.0)),
                ..default()
            })
            .with_children(|panel| {
                panel.spawn((
                    ui.title("SEND WAVE  [Space]", 17.0, theme::TEXT),
                    StartLabel,
                ));
                panel
                    .spawn((
                        bar_button(ButtonAction::StartWave),
                        ui.sliced(&ui.blue_depth, 18.0, Color::WHITE),
                    ))
                    .with_children(|button| {
                        button.spawn(ui.icon(&ui.play, 14.0, theme::TEXT));
                        button.spawn(ui.title("SEND WAVE", theme::BUTTON_SIZE, theme::TEXT));
                    });
                panel.spawn((
                    ui.label("", theme::MESSAGE_SIZE, theme::TEXT),
                    MessageText,
                    Node {
                        min_height: px(18.0),
                        ..default()
                    },
                ));
                panel.spawn(ui.label(
                    "LMB build · RMB sell · U upgrade · X sell · F speed · P path · R maze",
                    9.0,
                    theme::MUTED,
                ));
            });
        });

    // End-of-run overlay ---------------------------------------------------
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(0.0),
                top: px(0.0),
                width: percent(100.0),
                height: percent(100.0),
                display: Display::None,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.01, 0.02, 0.04, 0.72)),
            OverlayRoot,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    ui.sliced(&ui.grey_depth, 22.0, theme::PANEL_TINT),
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: px(10.0),
                        padding: UiRect::all(px(42.0)),
                        ..default()
                    },
                ))
                .with_children(|card| {
                    card.spawn((
                        ui.title("VICTORY", theme::OVERLAY_TITLE_SIZE, theme::GOOD),
                        OverlayTitle,
                    ));
                    card.spawn((
                        ui.label("", theme::OVERLAY_SUBTITLE_SIZE, theme::TEXT),
                        OverlaySubtitle,
                    ));
                    card.spawn(ui.label(
                        "Press R for a new maze",
                        theme::OVERLAY_HINT_SIZE,
                        theme::MUTED,
                    ));
                });
        });
}
