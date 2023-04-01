use crate::perigee_gltf::extras::GltfExtras;
use gltf::Gltf;
use rapier3d::na::{Isometry, Quaternion, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Index;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PointsOfInterestInitError {
    #[error("glTF must have Perigee extras to load points of interest")]
    PerigeeExtrasUndetected,
    #[error("invalid JSON stored in glTF node extras")]
    InvalidPerigeeExtrasData,
    #[error("glTF node must have a name")]
    UnnamedNode,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PointsOfInterest {
    map: HashMap<String, Isometry<f32, UnitQuaternion<f32>, 3>>,
}

impl PointsOfInterest {
    pub fn load_from_gltf(&mut self, gltf: &Gltf) -> Result<(), PointsOfInterestInitError> {
        for node in gltf.nodes() {
            let node_extra_data = match node.extras().as_ref() {
                Some(extra_data) => extra_data,
                None => return Err(PointsOfInterestInitError::PerigeeExtrasUndetected),
            };
            let node_extras_json = node_extra_data.get();
            let node_extras: GltfExtras = match serde_json::from_str(node_extras_json) {
                Ok(extras) => extras,
                Err(_) => return Err(PointsOfInterestInitError::InvalidPerigeeExtrasData),
            };
            if !node_extras.sim_settings.is_point_of_interest {
                continue;
            }

            let node_name = match node.name() {
                Some(name) => name,
                None => return Err(PointsOfInterestInitError::UnnamedNode),
            };

            let (translation, quaternion, _) = node.transform().decomposed();

            self.map.insert(
                String::from(node_name),
                Isometry::from_parts(
                    Vector3::new(translation[0], translation[1], translation[2]).into(),
                    UnitQuaternion::from_quaternion(Quaternion::new(
                        quaternion[3],
                        quaternion[0],
                        quaternion[1],
                        quaternion[2],
                    )),
                ),
            );
        }

        Ok(())
    }

    pub fn point_with_name(&self, name: &str) -> Option<&Isometry<f32, UnitQuaternion<f32>, 3>> {
        self.map.get(name)
    }
}

impl Index<&str> for PointsOfInterest {
    type Output = Isometry<f32, UnitQuaternion<f32>, 3>;
    fn index(&self, index: &str) -> &Self::Output {
        self.point_with_name(index)
            .expect("Unrecognized PoI name given!")
    }
}
