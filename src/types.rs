use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JsonScene {
    #[serde(default)]
    pub units: Option<String>,
    #[serde(default)]
    pub unit_scale: Option<f32>,
    pub entities: Vec<JsonEntity>,
    #[serde(default)]
    pub cams: Vec<JsonCamera>,
    #[serde(default)]
    pub lights: Vec<JsonLight>,
}

#[derive(Debug, Deserialize)]
pub struct JsonEntity {
    pub name: String,
    pub output_path: String, // absolute or relative path to .gltf
}

#[derive(Debug, Deserialize)]
pub struct JsonCamera {
    pub name: String,
    pub transform: JsonTransform,
    pub projection: String,
    pub clipping: JsonCameraClipping,

    #[serde(default)]
    pub resolution: Option<JsonResolution>,

    #[serde(default)]
    pub perspective: Option<JsonPerspective>,

    #[serde(default)]
    pub depth_of_field: Option<JsonDepthOfField>,

    #[serde(default)]
    pub shutter: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct JsonLight {
    pub name: String,
    pub transform: JsonTransform,
    #[serde(rename = "type")]
    pub kind: String, // "point", "spot"
    pub color: [f32; 3],
    pub intensity_photometric: f32,
    pub range: Option<f32>,       // for point and spot
    pub spot: SpotParms, // for spot
}

#[derive(Debug, Deserialize)]
pub struct SpotParms {
    pub cone_angle_deg: f32,
    pub cone_delta: f32,
    pub cone_rolloff: f32,
}

#[derive(Debug, Deserialize)]
pub struct JsonCameraClipping {
    pub near: f32,
    pub far: f32,
}

#[derive(Debug, Deserialize)]
pub struct JsonResolution {
    pub width: u32,
    pub height: u32,
    pub pixel_aspect: f32,
}

#[derive(Debug, Deserialize)]
pub struct JsonPerspective {
    pub focal_length_mm: f32,
    pub sensor_width_mm: f32,
    pub sensor_height_mm: f32,
    pub fov_y_rad: f32,
    pub aspect: f32,
}

#[derive(Debug, Deserialize)]
pub struct JsonDepthOfField {
    pub fstop: f32,
    pub focus_distance_m: f32,
}


#[derive(Debug, Deserialize)]
pub struct JsonTransform {
    pub translation: [f32; 3],
    pub rotation_quat_wxyz: [f32; 4], // quaternion
    pub scale: [f32; 3],
}