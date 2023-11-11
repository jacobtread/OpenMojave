use crate::constants::{WINDOW_DEFAULT_HEIGHT, WINDOW_DEFAULT_WIDTH};
use assets::bsa::BsaPlugin;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

pub mod assets;
pub mod constants;
pub mod utils;

fn main() {
    let config = utils::config::load_config();

    App::new()
        .insert_resource(config)
        .add_plugins(
            DefaultPlugins
                .build()
                // Add bsa asset loading where previous asset plugin was
                .add_before::<AssetPlugin, BsaPlugin>(BsaPlugin)
                // Custom window settings
                .set(WindowPlugin {
                    ..Default::default()
                })
                // Update logging
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "wgpu=error,naga=warn,open_mojave=debug,bevy_app=warn,bevy_render=warn"
                        .to_string(),
                }),
        )
        .run();
}
