#![allow(clippy::empty_docs)]

mod godot_api;
mod servers;

use godot::prelude::*;

struct ProjectileServer2D;

#[gdextension]
unsafe impl ExtensionLibrary for ProjectileServer2D {
    fn min_level() -> InitLevel {
        InitLevel::Servers
    }
}
