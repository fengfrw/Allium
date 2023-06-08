#![allow(unused)]
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use embedded_graphics::prelude::Size;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALLIUM_CONFIG_DIR: PathBuf = PathBuf::from(
        &env::var("ALLIUM_CONFIG_DIR").unwrap_or_else(|_| "/mnt/SDCARD/.allium".to_string())
    );
    pub static ref ALLIUM_GAMES_DIR: PathBuf = PathBuf::from(
        &env::var("ALLIUM_ROMS_DIR").unwrap_or_else(|_| "/mnt/SDCARD/Roms".to_string())
    );
    pub static ref ALLIUMD_STATE: PathBuf = ALLIUM_CONFIG_DIR.join(Path::new("state/alliumd.json"));
    pub static ref ALLIUM_LAUNCHER_STATE: PathBuf =
        ALLIUM_CONFIG_DIR.join(Path::new("state/allium-launcher.json"));
    pub static ref ALLIUM_GAME_INFO: PathBuf =
        ALLIUM_CONFIG_DIR.join(Path::new("state/current_game"));
    pub static ref ALLIUM_LAUNCHER: PathBuf = PathBuf::from(
        &env::var("ALLIUM_LAUNCHER")
            .unwrap_or_else(|_| "/mnt/SDCARD/.allium/allium-launcher".to_string())
    );
    pub static ref ALLIUM_MENU: PathBuf = PathBuf::from(
        &env::var("ALLIUM_MENU").unwrap_or_else(|_| "/mnt/SDCARD/.allium/allium-menu".to_string())
    );
    pub static ref ALLIUM_RETROARCH: PathBuf = PathBuf::from(
        &env::var("ALLIUM_RETROARCH")
            .unwrap_or_else(|_| "/mnt/SDCARD/.allium/cores/retroarch/launch.sh".to_string())
    );
}
pub const RETROARCH_UDP_SOCKET: &str = "127.0.0.1:55355";

pub const BUTTON_DIAMETER: u32 = 34;
pub const SELECTION_HEIGHT: u32 = 34;
pub const SELECTION_MARGIN: u32 = 8;
pub const IMAGE_SIZE: Size = Size::new(250, 376);
pub const LISTING_SIZE: i32 = 10;
pub const LISTING_JUMP_SIZE: i32 = 5;
pub const BATTERY_UPDATE_INTERVAL: Duration = Duration::from_secs(5);
