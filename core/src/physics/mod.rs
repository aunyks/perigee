use core::panic;

use crate::config::PhysicsConfig;
use event_queue::CollisionEventQueue;
use gltf::{accessor::DataType as GltfDataType, Gltf, Semantic as PrimitiveSemantic};
use rapier3d::{
    na::{Point3, Quaternion, Translation3, UnitQuaternion, Vector3},
    prelude::*,
};
use serde::{Deserialize, Serialize};

mod event_queue;

#[derive(Serialize, Deserialize)]
pub struct PhysicsWorld {
    pub gravity: Vector3<f32>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub integration_parameters: IntegrationParameters,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    #[serde(skip)]
    pub pipeline: PhysicsPipeline,
    pub event_queue: CollisionEventQueue,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        let default_config = PhysicsConfig::default();
        Self {
            gravity: default_config.gravity().into(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            pipeline: PhysicsPipeline::new(),
            event_queue: CollisionEventQueue::with_capacity(default_config.event_queue_capacity()),
        }
    }
}

impl PhysicsWorld {
    /// Remove a RigidBody from the physics world using its handle.
    pub fn remove_body(&mut self, body_handle: RigidBodyHandle) -> Option<RigidBody> {
        self.rigid_body_set.remove(
            body_handle,
            &mut self.island_manager,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            true,
        )
    }

    /// Load the meshes of the provided glTF file and load them as static
    /// physics objects. Use this to build static parts of the scene.
    ///
    /// Note: Meshes that are children of others will be ignored.
    pub fn add_static_objects_from_gltf(
        &mut self,
        gltf: &Gltf,
    ) -> Result<Vec<RigidBodyHandle>, String> {
        // Allocate enough space for every mesh in the glTF
        // so we can do it all at once instead of allocating
        // each time we arrive at a new mesh
        let mut rigid_body_handles: Vec<RigidBodyHandle> =
            Vec::with_capacity(gltf.document.meshes().len());

        let gltf_bytes = match &gltf.blob {
            Some(bytes) => bytes,
            None => {
                return Err(String::from(
                    "Could not access the provided glTF's binary payload. Does it exist?",
                ));
            }
        };
        // Note that this skips over any nodes
        // that are childed to others
        for node in gltf.nodes() {
            if let Some(mesh) = node.mesh() {
                let mesh_name = mesh.name().unwrap_or("Unknown");
                let mut rigid_body_builder = RigidBodyBuilder::fixed();
                // Translations and scales in xyz, quats in xyzw
                let (translation, quaternion, scale) = node.transform().decomposed();

                let body_isometry = Isometry::from_parts(
                    Translation3::new(translation[0], translation[1], translation[2]),
                    UnitQuaternion::from_quaternion(Quaternion::new(
                        quaternion[3],
                        quaternion[0],
                        quaternion[1],
                        quaternion[2],
                    )),
                );
                rigid_body_builder = rigid_body_builder.sleeping(true).position(body_isometry);

                // We need to initialize this since we're not
                // certain that any mesh primitives will exist
                #[allow(unused_assignments)]
                let mut collider_builder: Option<ColliderBuilder> = None;

                for primitive in mesh.primitives() {
                    let indices_accesor = primitive.indices().unwrap_or_else(|| {
                        panic!(
                            "Could not get an primitive indices accessor from {}!",
                            mesh_name
                        )
                    });
                    match indices_accesor.view() {
                        Some(indices_buffer_view) => {
                            let view_start =
                                indices_buffer_view.offset() + indices_accesor.offset();
                            let view_bytes =
                                &gltf_bytes[view_start..view_start + indices_buffer_view.length()];

                            // 3 indices make a face. GLTF stores all indices as bytes in a *flat* array. We want them
                            // in groups of three so that each group comprises a face of the geometry
                            let mut indices: Vec<[u32; 3]> =
                                Vec::with_capacity(indices_accesor.count() / 3);

                            match indices_accesor.data_type() {
                                GltfDataType::U16 => {
                                    let flattened_indices: Vec<u16> = view_bytes
                                        .chunks_exact(2)
                                        .map(|uint_bytes| {
                                            let uint_byte_array: [u8; 2] = uint_bytes[0..2]
                                            .try_into()
                                            .expect(
                                            "Could not convert u16 byte slice into u16 byte array",
                                        );
                                            u16::from_le_bytes(uint_byte_array)
                                        })
                                        .collect();
                                    let chunked_indices: Vec<&[u16]> =
                                        flattened_indices.chunks(3).collect();
                                    assert_eq!(chunked_indices.len(), indices.capacity(), "u16 chunked indices and final u32 indices vectors had different lengths!");
                                    for face_u16 in chunked_indices {
                                        indices.push([
                                            u32::from(face_u16[0]),
                                            u32::from(face_u16[1]),
                                            u32::from(face_u16[2]),
                                        ]);
                                    }
                                }
                                GltfDataType::U32 => {
                                    let flattened_indices: Vec<u32> = view_bytes
                                        .chunks_exact(4)
                                        .map(|uint_bytes| {
                                            let uint_byte_array: [u8; 4] = uint_bytes[0..4]
                                            .try_into()
                                            .expect(
                                            "Could not convert u32 byte slice into u32 byte array",
                                        );
                                            u32::from_le_bytes(uint_byte_array)
                                        })
                                        .collect();
                                    let chunked_indices: Vec<&[u32]> =
                                        flattened_indices.chunks(3).collect();
                                    assert_eq!(chunked_indices.len(), indices.capacity(), "u32 chunked indices and final u32 indices vectors had different lengths!");
                                    for face_u32 in chunked_indices {
                                        indices.push([face_u32[0], face_u32[1], face_u32[2]]);
                                    }
                                }
                                _ => {
                                    panic!(
                                        "Indices accessor data type was neither U16 not U32! Was {:?}",
                                        indices_accesor.data_type()
                                    );
                                }
                            };
                            match primitive.get(&PrimitiveSemantic::Positions) {
                                Some(vertex_positions_accessor) => {
                                    if let Some(positions_buffer_view) =
                                        vertex_positions_accessor.view()
                                    {
                                        let view_start = positions_buffer_view.offset()
                                            + vertex_positions_accessor.offset();
                                        let view_bytes = &gltf_bytes[view_start
                                            ..view_start + positions_buffer_view.length()];

                                        let mut floats: Vec<f32> =
                                            Vec::with_capacity(view_bytes.len() / 4);
                                        for float_bytes in view_bytes.chunks_exact(4) {
                                            let float_byte_array: [u8; 4] = float_bytes[0..4]
                                                .try_into()
                                                .expect(
                                                "Could not convert float byte slice into float byte array",
                                            );
                                            floats.push(f32::from_le_bytes(float_byte_array));
                                        }
                                        let mut vertices: Vec<Point3<f32>> =
                                            Vec::with_capacity(floats.len() / 3);
                                        for float_chunk in floats.chunks(3) {
                                            vertices.push(Point3::new(
                                                float_chunk[0],
                                                float_chunk[1],
                                                float_chunk[2],
                                            ));
                                        }

                                        let trimesh = TriMesh::new(vertices, indices)
                                            .scaled(&vector![scale[0], scale[1], scale[2]]);

                                        collider_builder = Some(ColliderBuilder::trimesh(
                                            trimesh.vertices().to_vec(),
                                            trimesh.indices().to_vec(),
                                        ));
                                    } else {
                                        return Err(String::from(
                                            "No vertex positions found for this mesh!",
                                        ));
                                    }
                                }
                                None => {
                                    return Err(format!(
                                        "No vertex positions accessor found for mesh {}",
                                        mesh_name
                                    ));
                                }
                            };
                        }
                        None => {
                            return Err(format!(
                                "No vertex indices accessor found for mesh {}",
                                mesh_name,
                            ));
                        }
                    };
                    if collider_builder.is_none() {
                        return Err(String::from(
                            "Unknown error occurred before building collider!",
                        ));
                    }
                    let body_handle = self.rigid_body_set.insert(rigid_body_builder.build());
                    rigid_body_handles.push(body_handle);
                    self.collider_set.insert_with_parent(
                        collider_builder
                            .unwrap_or_else(|| panic!("Collider builder was None!"))
                            .build(),
                        body_handle,
                        &mut self.rigid_body_set,
                    );
                }
            }
        }
        Ok(rigid_body_handles)
    }

    /// Step the physics simulation by the provided number of seconds.
    pub fn step(&mut self, delta_seconds: f32) {
        self.integration_parameters.dt = delta_seconds;

        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &(),
            &self.event_queue,
        );
        self.query_pipeline.update(
            &self.island_manager,
            &self.rigid_body_set,
            &self.collider_set,
        );
    }

    /// Recover the handle from a RigidBody using its `user_data` field.
    pub fn get_body_handle(body: &RigidBody) -> RigidBodyHandle {
        let lower_32_bits_mask = 0xffffffff_u128;
        let body_user_data = body.user_data;
        let handle_generation_u128 = body_user_data & lower_32_bits_mask;
        let handle_index_u128 = body_user_data.rotate_right(32) & lower_32_bits_mask;
        let handle_generation = u32::try_from(handle_generation_u128)
            .expect("Could not downcast rigid handle generation part from u128 to u32!");
        let handle_index = u32::try_from(handle_index_u128)
            .expect("Could not downcast rigid handle index part from u128 to u32!");
        RigidBodyHandle::from_raw_parts(handle_index, handle_generation)
    }

    /// Store the parts of the RigidBody's handle in its `user_data`  field.
    pub fn store_handle_in_body(handle: &RigidBodyHandle, body: &mut RigidBody) {
        let handle_parts = handle.into_raw_parts();
        let handle_index = handle_parts.0;
        let handle_generation = handle_parts.1;
        body.user_data = u128::from(handle_index).rotate_left(32) | u128::from(handle_generation);
    }

    /// Create physics world with default parameters in addition to the provided gravity.
    pub fn with_config(config: PhysicsConfig) -> Self {
        Self {
            gravity: config.gravity().into(),
            event_queue: CollisionEventQueue::with_capacity(config.event_queue_capacity()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_and_recover_rigid_body_handles() {
        let mut world = PhysicsWorld::default();
        for _ in 0..10 {
            let body = RigidBodyBuilder::dynamic().build();
            let handle = world.rigid_body_set.insert(body);
            // Get the body again since it was moved into the simulation
            let body = world.rigid_body_set.get_mut(handle).unwrap();

            PhysicsWorld::store_handle_in_body(&handle, body);

            let recovered_handle = PhysicsWorld::get_body_handle(body);
            assert_eq!(handle, recovered_handle);
        }
    }
}
