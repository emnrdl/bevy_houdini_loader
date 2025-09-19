use bevy::prelude::*;
use bevy::scene::SceneRoot;

use crate::io::{dev_warn_if_missing_asset, load_json_scene, to_asset_path};
use crate::HoudiniImportSettings;



#[derive(Component)]
pub struct FromHoudini;


pub fn import_entities_from_houdini(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<HoudiniImportSettings>,
) {
    let Ok(parsed) = load_json_scene(&settings.json_path) else {
        bevy::log::warn!("Failed to load or parse JSON: {}", &settings.json_path);
        return;
    };

    let root = commands
        .spawn((
            Name::new("HoudiniRoot"),
            FromHoudini,
            Transform::default(),
            Visibility::Visible,
        ))
        .id();

    for e in parsed.entities {
        let rel = to_asset_path(&e.output_path);
        dev_warn_if_missing_asset(&rel);
        
        let handle: Handle<Scene> = asset_server.load(format!("{rel}#Scene0"));

        let child = commands
            .spawn((Name::new(e.name), SceneRoot(handle)))
            .id();

        commands.entity(root).add_child(child);
    }

    for cam in parsed.cams{
        let q_wxyz = cam.transform.rotation_quat_wxyz;
        let rot = Quat::from_xyzw(q_wxyz[0], q_wxyz[1], q_wxyz[2], q_wxyz[3]);

        let transform = Transform {
            translation: Vec3::from(cam.transform.translation),
            rotation: rot,
            scale: Vec3::ONE,
        };

        let cam_entity = commands.spawn((
            Name::new(cam.name.clone()),
            transform,
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection {
                fov: cam.perspective.as_ref().map_or(1.0, |p| p.fov_y_rad),
                near: cam.clipping.near,
                far: cam.clipping.far,
                ..Default::default()
            }),
            FromHoudini,
        )).id();
        commands.entity(root).add_child(cam_entity);
        
    }

    for light in parsed.lights{
        let q_wxyz = light.transform.rotation_quat_wxyz;
        let rot = Quat::from_xyzw(q_wxyz[0], q_wxyz[1], q_wxyz[2], q_wxyz[3]);

        let transform = Transform {
            translation: Vec3::from(light.transform.translation),
            rotation: rot,
            scale: Vec3::ONE,
        };


        if light.kind == "Point"{
            let light_entity = commands.spawn((
                            PointLight {
                                intensity: light.intensity_photometric*10000.0, // photometric to bevy factor
                                color: Color::srgb(light.color[0], light.color[1], light.color[2]),
                                range: 25.0,
                                shadows_enabled: true,
                                ..default()
                            },
                            Name::new(light.name),
                            transform,
                            FromHoudini,
                        )).id();
            commands.entity(root).add_child(light_entity);

        } else if light.kind == "Spot"{
            let light_entity = commands.spawn((
                            SpotLight {
                                intensity: light.intensity_photometric*10000.0, // photometric to bevy factor
                                color: Color::srgb(light.color[0], light.color[1], light.color[2]),
                                range: 25.0,
                                radius: light.spot.cone_angle_deg.to_radians(),
                                shadows_enabled: true,
                                ..default()
                            },
                            Name::new(light.name),
                            transform,
                            FromHoudini,
                        )).id();
            commands.entity(root).add_child(light_entity);
        } else {
            bevy::log::warn!("Unknown light kind: {}", light.kind);
            continue;
        }
    }
}