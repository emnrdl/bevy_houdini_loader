//! Minimal Bevy plugin to spawn glTF Scene0s from a Houdini manifest JSON.
//! Manifest path is under `assets/` (default: "output.json").
//!
//! Usage:
//!   
//!  fn main() {
//!      App::new()
//!          .add_plugins(DefaultPlugins)
//!          .insert_resource(HoudiniImportSettings {
//!              json_path: "assets/output.json".into(),
//!          })
//!          .add_plugins(HoudiniImportPlugin)
//!          .run();
//!  }

use bevy::prelude::*;

pub mod io;
pub mod types;
mod systems;

#[derive(Resource, Clone)]
pub struct HoudiniImportSettings {
    pub json_path: String, // e.g. "assets/output.json"
}

impl Default for HoudiniImportSettings {
    fn default() -> Self {
        Self { json_path: "assets/output.json".into() }
    }
}

pub struct HoudiniImportPlugin;

impl Plugin for HoudiniImportPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoudiniImportSettings>()
           .add_systems(Startup, systems::import_entities_from_houdini);
    }
}

pub mod prelude {
    pub use super::{HoudiniImportPlugin, HoudiniImportSettings};
}