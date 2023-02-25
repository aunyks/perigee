use rapier3d::na::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GltfBodyType {
    #[serde(rename = "SENSOR")]
    Sensor,
    #[serde(rename = "STATIC")]
    Static,
    #[serde(rename = "KINEMATIC")]
    Kinematic,
    #[serde(rename = "DYNAMIC")]
    Dynamic,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GltfOptimizedShape {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "CUBOID")]
    Cuboid,
    #[serde(rename = "SPHERE")]
    Sphere,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GltfPhysicsSettings {
    pub enabled: bool,
    #[serde(rename = "isAnonymous")]
    pub is_anonymous: bool,
    #[serde(rename = "bodyType")]
    pub body_type: GltfBodyType,
    pub mass: f32,
    #[serde(rename = "optimizedShape")]
    pub optimized_shape: GltfOptimizedShape,
    #[serde(rename = "baseScale")]
    #[serde(default)]
    pub base_scale: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GltfSimSettings {
    pub physics: GltfPhysicsSettings,
    #[serde(rename = "isPointOfInterest")]
    pub is_point_of_interest: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GltfExtras {
    #[serde(rename = "simSettings")]
    pub sim_settings: GltfSimSettings,
}
