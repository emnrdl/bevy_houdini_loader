use crate::types::JsonScene;
use std::{fs, path::Path};
use serde_json::Deserializer;
use serde_path_to_error as path;

pub fn load_json_scene(json_path: &str) -> anyhow::Result<JsonScene> {
    let txt = fs::read_to_string(json_path)?;
    let mut de = Deserializer::from_str(&txt);
    match path::deserialize::<_, JsonScene>(&mut de) {
        Ok(scene) => Ok(scene),
        Err(err) => {
            // Örn: $.lights[0].spot.cone_angle_deg gibi tam yol
            eprintln!("JSON path error at {}: {}", err.path(), err);
            Err(err.into())
        }
    }
}

/// Convert absolute ".../assets/file.gltf" -> "file.gltf"
pub fn to_asset_path(p: &str) -> String {
    let unified = p.replace('\\', "/");
    if let Some(idx) = unified.rfind("/assets/") {
        return unified[idx + "/assets/".len()..].to_string();
    }
    unified
}

/// Dev sırasında eksik dosyayı uyar
pub fn dev_warn_if_missing_asset(rel: &str) {
    let on_disk = format!("assets/{}", rel);
    if !Path::new(&on_disk).exists() {
        // tracing makroları:
        bevy::log::warn!("Missing file on disk: {}", on_disk);
    }
}
