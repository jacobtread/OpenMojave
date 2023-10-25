use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

use config::{GameConfigPlugin, GameConfiguration};
use loaders::bsa::BsaAssetPlugin;

mod config;
mod esm;
mod loaders;

fn main() {
    App::new()
        .add_plugins((GameConfigPlugin, BsaAssetPlugin))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "OpenNV".into(),
                        resolution: (960., 540.).into(),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    asset_folder: "DataPacked".to_string(),
                    ..Default::default()
                }),
        )
        .add_systems(Startup, setup_menu)
        .run();
}

const MENU_BACKGROUND_IMAGE: &str = "textures/interface/main/main_background.dds";
const MENU_TITLE_IMAGE: &str = "textures/interface/main/main_title.dds";
const MENU_AUDIO: &str = "MainTitle.wav";

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GameConfiguration>,
) {
    let background_path = format!(
        "/textures/interface/main/{}.dds",
        config.loading.sMainMenuBackground
    );

    let font_image: Handle<Image> = asset_server.load("textures/fonts/glow_monofonto_medium.fnt");

    let background_image: Handle<Image> = asset_server.load(MENU_BACKGROUND_IMAGE);

    let title_image: Handle<Image> = asset_server.load(MENU_TITLE_IMAGE);
    let menu_sound: Handle<AudioSource> = asset_server.load(MENU_AUDIO);

    // 2D camera for the menu
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: UiImage {
                        texture: background_image,
                        ..Default::default()
                    },
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        aspect_ratio: Some(16.0 / 9.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Vw(2.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: title_image,
                            ..Default::default()
                        },
                        style: Style {
                            // left: Val::Vw(5.),
                            width: Val::Vw(35.),
                            height: Val::Vw(35.0 / 4.0),
                            aspect_ratio: Some(4.0),

                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: font_image,
                            ..Default::default()
                        },
                        style: Style {
                            // left: Val::Vw(5.),
                            width: Val::Px(256.),
                            height: Val::Px(256.),
                            aspect_ratio: Some(1.0),

                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });

    // Menu background audio
    commands.spawn(AudioBundle {
        source: menu_sound,
        settings: PlaybackSettings::LOOP,
    });
}
