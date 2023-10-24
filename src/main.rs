use std::fs::{create_dir_all, File};
use std::io::{BufReader, Seek};
use std::path::Path;

use bevy::a11y::AccessibilityPlugin;
use bevy::asset::FileAssetIo;
use bevy::audio::Source;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::scene::ScenePlugin;
use bevy::time::TimePlugin;
use bevy::ui::widget::UiImageSize;
use bevy::utils::HashSet;
use bevy::window::{PresentMode, WindowTheme};
use bevy::winit::WinitPlugin;
use binrw::BinRead;
use bsa::v104::ReaderV104;
use bsa::{read, Reader};
use esm::RecordHeader;
use rayon::prelude::{ParallelBridge, ParallelIterator};
mod esm;

fn main() {
    App::new()
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
                    asset_folder: "Data".to_string(),
                    ..Default::default()
                }),
        )
        .add_systems(Startup, setup_menu)
        .run();
}

const MENU_BACKGROUND_IMAGE: &str = "Fallout - Textures2.bsa/main_background.dds";
const MENU_TITLE_IMAGE: &str = "Fallout - Textures2.bsa/main_title.dds";
const MENU_AUDIO: &str = "MainTitle.wav";

fn setup() {}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                });
        });

    // Menu background audio
    commands.spawn(AudioBundle {
        source: menu_sound,
        settings: PlaybackSettings::LOOP,
    });
}
