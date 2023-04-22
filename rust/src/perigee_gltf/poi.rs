use crate::perigee_gltf::extras::GltfExtras;
use gltf::{Gltf, Node};
use rapier3d::na::{Isometry3, Quaternion, UnitQuaternion, Vector3};
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
    map: HashMap<String, Isometry3<f32>>,
}

impl PointsOfInterest {
    fn visit_gltf_node(
        &mut self,
        node: &Node,
        parent_isometry: &Isometry3<f32>,
        visited_nodes: &mut HashMap<usize, ()>,
    ) -> Result<(), PointsOfInterestInitError> {
        if visited_nodes.contains_key(&node.index()) {
            return Ok(());
        }

        let node_extra_data = match node.extras().as_ref() {
            Some(extra_data) => extra_data,
            None => return Err(PointsOfInterestInitError::PerigeeExtrasUndetected),
        };
        let node_extras_json = node_extra_data.get();
        let node_extras: GltfExtras = match serde_json::from_str(node_extras_json) {
            Ok(extras) => extras,
            Err(_) => return Err(PointsOfInterestInitError::InvalidPerigeeExtrasData),
        };

        let (translation, quaternion, _) = node.transform().decomposed();
        let global_isometry = parent_isometry
            * Isometry3::from_parts(
                Vector3::new(translation[0], translation[1], translation[2]).into(),
                UnitQuaternion::from_quaternion(Quaternion::new(
                    quaternion[3],
                    quaternion[0],
                    quaternion[1],
                    quaternion[2],
                )),
            );

        for child_node in node.children() {
            self.visit_gltf_node(&child_node, &global_isometry, visited_nodes)?;
        }

        if node_extras.sim_settings.is_point_of_interest {
            let node_name = match node.name() {
                Some(name) => name,
                None => return Err(PointsOfInterestInitError::UnnamedNode),
            };

            self.map.insert(String::from(node_name), global_isometry);
        }

        visited_nodes.insert(node.index(), ());
        Ok(())
    }

    pub fn load_from_gltf(&mut self, gltf: &Gltf) -> Result<(), PointsOfInterestInitError> {
        let mut visited_nodes: HashMap<usize, ()> = HashMap::new();
        // Only loads the first scene
        if let Some(scene) = gltf.scenes().next() {
            for node in scene.nodes() {
                self.visit_gltf_node(&node, &Isometry3::identity(), &mut visited_nodes)?;
            }
        }

        Ok(())
    }

    pub fn point_with_name(&self, name: &str) -> Option<&Isometry3<f32>> {
        self.map.get(name)
    }
}

impl Index<&str> for PointsOfInterest {
    type Output = Isometry3<f32>;
    fn index(&self, index: &str) -> &Self::Output {
        self.point_with_name(index)
            .expect("Unrecognized PoI name given!")
    }
}
