//! Handles for the Kenney UI Pack assets and small styling helpers.

use bevy::prelude::*;
use bevy::sprite::{BorderRect, SliceScaleMode, TextureSlicer};

#[derive(Resource, Clone)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub font_narrow: Handle<Font>,
    pub blue_depth: Handle<Image>,
    pub blue_flat: Handle<Image>,
    pub grey_depth: Handle<Image>,
    pub grey_flat: Handle<Image>,
    pub grey_border: Handle<Image>,
    pub divider: Handle<Image>,
    pub star: Handle<Image>,
    pub orb: Handle<Image>,
    pub square: Handle<Image>,
    pub play: Handle<Image>,
    pub repeat: Handle<Image>,
    pub check: Handle<Image>,
    pub cross: Handle<Image>,
}

impl UiAssets {
    pub fn load(asset_server: &AssetServer) -> Self {
        let image = |path: &'static str| asset_server.load(path);
        Self {
            font: asset_server.load("ui/fonts/KenneyFuture.ttf"),
            font_narrow: asset_server.load("ui/fonts/KenneyFutureNarrow.ttf"),
            blue_depth: image("ui/buttons/blue_depth.png"),
            blue_flat: image("ui/buttons/blue_flat.png"),
            grey_depth: image("ui/buttons/grey_depth.png"),
            grey_flat: image("ui/buttons/grey_flat.png"),
            grey_border: image("ui/buttons/grey_border.png"),
            divider: image("ui/icons/divider.png"),
            star: image("ui/icons/star.png"),
            orb: image("ui/icons/orb.png"),
            square: image("ui/icons/square.png"),
            play: image("ui/icons/play.png"),
            repeat: image("ui/icons/repeat.png"),
            check: image("ui/icons/check.png"),
            cross: image("ui/icons/cross.png"),
        }
    }

    pub fn title(&self, value: impl Into<String>, size: f32, color: Color) -> impl Bundle {
        (
            Text::new(value),
            TextFont {
                font: self.font.clone().into(),
                font_size: size.into(),
                ..default()
            },
            TextColor(color),
        )
    }

    pub fn label(&self, value: impl Into<String>, size: f32, color: Color) -> impl Bundle {
        (
            Text::new(value),
            TextFont {
                font: self.font_narrow.clone().into(),
                font_size: size.into(),
                ..default()
            },
            TextColor(color),
        )
    }

    /// Nine-sliced border/button image, tinted.
    pub fn sliced(&self, image: &Handle<Image>, border: f32, color: Color) -> ImageNode {
        ImageNode {
            image: image.clone(),
            color,
            image_mode: NodeImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(border),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            }),
            ..default()
        }
    }

    pub fn icon(&self, image: &Handle<Image>, size: f32, color: Color) -> impl Bundle {
        (
            ImageNode {
                image: image.clone(),
                color,
                ..default()
            },
            Node {
                width: px(size),
                height: px(size),
                ..default()
            },
        )
    }
}

pub fn setup_ui_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets::load(&asset_server));
}
