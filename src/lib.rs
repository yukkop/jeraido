#![allow(clippy::module_inception)]

mod actor;
mod component;
mod controls;
mod level;
mod lobby;
mod settings;
mod sound;
mod ui;
mod util;
mod world;

#[cfg(all(debug_assertions, feature = "devtools"))]
pub mod editor;
pub mod core;

pub const ASSET_DIR: &str = "asset";

#[cfg(feature = "devtools")]
lazy_static::lazy_static! {
    /// If the application is running in debug mode
    pub static ref DEBUG: bool = {
        let debug = std::env::var("DEBUG").is_ok();
        println!("DEVTOOLS: DEBUG = {}", debug);
        debug
    };
}

