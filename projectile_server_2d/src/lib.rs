#![allow(clippy::empty_docs)]

mod constants;
mod godot_api;
mod servers;

use godot::prelude::*;

struct ProjectileServer2D;

#[gdextension]
unsafe impl ExtensionLibrary for ProjectileServer2D {
    fn min_level() -> InitLevel {
        InitLevel::Servers
    }

    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                // servers::register_scene();
            }
            InitLevel::Servers => {
                // servers::register_server();
            }
            _ => (),
        }
    }

    fn on_level_deinit(_level: InitLevel) {}
}
