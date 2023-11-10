use crate::constants::{WINDOW_DEFAULT_HEIGHT, WINDOW_DEFAULT_WIDTH};
use bevy::prelude::*;

pub mod assets;
pub mod constants;
pub mod utils;

fn main() {
    utils::logging::setup_logger(log::LevelFilter::Debug);

    App::new().run();
}
