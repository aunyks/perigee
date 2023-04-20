use std::collections::HashMap;

use crate::config::PhysicsConfig;
use crate::perigee_gltf::extras::{GltfBodyType, GltfExtras, GltfOptimizedShape};
use crate::perigee_gltf::util::access_gltf_bytes;
use crate::physics::contact_event_mgmt::ContactEventManager;
use crate::physics::handle_map::{NamedColliderHandleMap, NamedRigidBodyHandleMap};
use crate::traits::{physics::ColliderEventListener, FromConfig};
pub use collider_event_listener::*;
use gltf::Node;
use gltf::{accessor::DataType as GltfDataType, Gltf, Semantic as PrimitiveSemantic};
use log::debug;
use log::warn;
use rapier3d::{
    na::{Isometry, Point3, Quaternion, Translation3, UnitQuaternion, Vector3},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod collider_event_listener;
mod contact_event_mgmt;
mod handle_map;

#[derive(Error, Debug)]
pub enum PhysicsWorldInitError {
    /// The binary payload for the glTF couldn't be found.
    #[error("can't access the provided glTF's binary payload")]
    CantAccessBlob,
    /// The Perigee-specific glTF extras for a glTF node couldn't be found.
    #[error("glTF must have Perigee extras to load physics world")]
    PerigeeExtrasUndetected,
    /// The Perigee-specific glTF extras didn't follow the expected schema.
    #[error("invalid JSON stored in glTF node extras")]
    InvalidPerigeeExtrasData,
    /// A glTF mesh didn't have a name.
    #[error("glTF mesh must have a name")]
    UnnamedMesh,
    /// A glTF node didn't have a name.
    #[error("glTF node must have a name")]
    UnnamedNode,
    /// A glTF meshes was defined as a sensor type.
    #[error("glTF mesh cannot be imported as sensor")]
    MeshCantBeSensor,
    /// The accessor for the primitive indices of a trimesh couldn't be found.
    #[error("no primitive accessor for trimesh")]
    NoPrimitiveAccessorForTrimesh,
    /// An accessor for a trimesh's vertex positions couldn't be found.
    #[error("no vertex positions accessor found for mesh")]
    NoVertexPositionsAccessor,
    /// Mesh indices accessor data type was neither u16 nor u32.
    #[error("indices accessor data type was neither U16 nor U32")]
    InvalidIndicesDataType,
    /// No mesh indices were found for a mesh.
    #[error("no indices found for mesh")]
    NoIndicesFound,
    /// No vertices were found for a mesh.
    #[error("no vertices found for mesh")]
    NoVerticesFound,
    #[error("could not get accessor bytes")]
    CouldntAccessBytes,
    #[error("mesh defined as convex is not convex")]
    MeshNotConvex,
}

/// The physics management structure. This is a
/// thin wrapper around [the Rapier physics engine](https://rapier.rs)
/// with additional utilities.
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
    pub named_rigid_bodies: NamedRigidBodyHandleMap,
    pub named_sensors: NamedColliderHandleMap,
    #[serde(skip)]
    collider_event_handlers: HashMap<ColliderHandle, Vec<Box<dyn ColliderEventListener>>>,
    #[serde(skip)]
    pub pipeline: PhysicsPipeline,
    #[serde(skip)]
    contact_event_manager: ContactEventManager,
}

impl FromConfig for PhysicsWorld {
    type Config<'a> = &'a PhysicsConfig;
    fn from_config<'a>(config: Self::Config<'a>) -> Self {
        Self {
            gravity: config.gravity().into(),
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
            contact_event_manager: ContactEventManager::with_capacity(
                config.event_queue_capacity(),
            ),
            named_rigid_bodies: NamedRigidBodyHandleMap::default(),
            named_sensors: NamedColliderHandleMap::default(),
            collider_event_handlers: HashMap::default(),
        }
    }

    fn set_config<'a>(&mut self, _config: Self::Config<'a>) {
        warn!("Perigee PhysicsWorld doesn't allow resetting configuration");
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::from_config(&PhysicsConfig::default())
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

    fn visit_node(
        &mut self,
        node: &Node,
        parent_isometry: &Isometry<f32, UnitQuaternion<f32>, 3>,
        gltf_bytes: &Vec<u8>,
        visited_nodes: &mut HashMap<usize, ()>,
    ) -> Result<(), PhysicsWorldInitError> {
        debug!("{:?}", node.name());
        let node_extra_data = match node.extras().as_ref() {
            Some(extra_data) => extra_data,
            None => return Err(PhysicsWorldInitError::PerigeeExtrasUndetected),
        };
        let node_extras_json = node_extra_data.get();
        let node_extras: GltfExtras = match serde_json::from_str(node_extras_json) {
            Ok(extras) => extras,
            Err(_) => return Err(PhysicsWorldInitError::InvalidPerigeeExtrasData),
        };

        let body_type = node_extras.sim_settings.physics.body_type;
        let (translation, quaternion, scale) = node.transform().decomposed();
        let object_isometry = Isometry::from_parts(
            Translation3::new(translation[0], translation[1], translation[2]),
            UnitQuaternion::from_quaternion(Quaternion::new(
                quaternion[3],
                quaternion[0],
                quaternion[1],
                quaternion[2],
            )),
        );
        let global_isometry = parent_isometry * object_isometry;
        debug!("{}", global_isometry);
        debug!("---------");
        for child_node in node.children() {
            self.visit_node(&child_node, &global_isometry, gltf_bytes, visited_nodes)?;
        }
        if !node_extras.sim_settings.physics.enabled || visited_nodes.contains_key(&node.index()) {
            return Ok(());
        }

        // Create a rigid body
        if let Some(mesh) = node.mesh() {
            let mesh_name = match node.name() {
                Some(name) => name,
                None => return Err(PhysicsWorldInitError::UnnamedMesh),
            };
            let mesh_name = String::from(mesh_name);
            let mut rigid_body_builder = match body_type {
                GltfBodyType::Static => RigidBodyBuilder::fixed().sleeping(true),
                GltfBodyType::Kinematic => {
                    RigidBodyBuilder::kinematic_position_based().sleeping(true)
                }
                GltfBodyType::Dynamic => RigidBodyBuilder::dynamic(),
                GltfBodyType::Sensor => return Err(PhysicsWorldInitError::MeshCantBeSensor),
            };
            rigid_body_builder = rigid_body_builder.position(global_isometry);

            let base_scale = node_extras.sim_settings.physics.base_scale;
            let collider_silhouette = match node_extras.sim_settings.physics.optimized_shape {
                GltfOptimizedShape::Cuboid => {
                    let cuboid_half_dimensions =
                        base_scale.component_mul(&Vector3::from(scale)) / 2.0;
                    SharedShape::cuboid(
                        cuboid_half_dimensions.x,
                        cuboid_half_dimensions.y,
                        cuboid_half_dimensions.z,
                    )
                }
                GltfOptimizedShape::Sphere => {
                    let ball_dimensions = base_scale.component_mul(&Vector3::from(scale));
                    SharedShape::ball(ball_dimensions.x / 2.0)
                }
                GltfOptimizedShape::ConvexMesh => {
                    let mut maybe_indices: Option<Vec<[u32; 3]>> = None;
                    let mut maybe_vertices: Option<Vec<Point3<f32>>> = None;
                    for primitive in mesh.primitives() {
                        let indices_accesor = match primitive.indices() {
                            Some(accessor) => accessor,
                            None => {
                                return Err(PhysicsWorldInitError::NoPrimitiveAccessorForTrimesh)
                            }
                        };

                        let indices_bytes = if let Ok(indices_bytes) =
                            access_gltf_bytes(gltf_bytes, &indices_accesor)
                        {
                            indices_bytes
                        } else {
                            return Err(PhysicsWorldInitError::CouldntAccessBytes);
                        };
                        let mut indices: Vec<[u32; 3]> =
                            Vec::with_capacity(indices_accesor.count() / 3);

                        match indices_accesor.data_type() {
                            GltfDataType::U16 => {
                                let flattened_indices: Vec<u16> = indices_bytes
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
                                for face_u16 in chunked_indices {
                                    indices.push([
                                        u32::from(face_u16[0]),
                                        u32::from(face_u16[1]),
                                        u32::from(face_u16[2]),
                                    ]);
                                }
                                maybe_indices = Some(indices);
                            }
                            GltfDataType::U32 => {
                                let flattened_indices: Vec<u32> = indices_bytes
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
                                for face_u32 in chunked_indices {
                                    indices.push([face_u32[0], face_u32[1], face_u32[2]]);
                                }
                                maybe_indices = Some(indices);
                            }
                            _ => {
                                return Err(PhysicsWorldInitError::InvalidIndicesDataType);
                            }
                        };
                        match primitive.get(&PrimitiveSemantic::Positions) {
                            None => {
                                return Err(PhysicsWorldInitError::NoVertexPositionsAccessor);
                            }
                            Some(vertex_positions_accessor) => {
                                let positions_bytes = if let Ok(positions_bytes) =
                                    access_gltf_bytes(gltf_bytes, &vertex_positions_accessor)
                                {
                                    positions_bytes
                                } else {
                                    return Err(PhysicsWorldInitError::CouldntAccessBytes);
                                };

                                let mut floats: Vec<f32> =
                                    Vec::with_capacity(positions_bytes.len() / 4);
                                for float_bytes in positions_bytes.chunks_exact(4) {
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
                                maybe_vertices = Some(vertices);
                            }
                        };
                    }
                    if maybe_indices.is_none() {
                        return Err(PhysicsWorldInitError::NoIndicesFound);
                    }
                    if maybe_vertices.is_none() {
                        return Err(PhysicsWorldInitError::NoVerticesFound);
                    }
                    let scaled_trimesh: TriMesh = TriMesh::new(
                        maybe_vertices
                            .expect("Trimesh vertices were None despite asserting they weren't!"),
                        maybe_indices
                            .expect("Trimesh indices were None despite asserting they weren't!"),
                    )
                    .scaled(&Vector3::new(scale[0], scale[1], scale[2]));
                    if let Some(shape) =
                        SharedShape::convex_hull(scaled_trimesh.vertices().to_vec().as_ref())
                    {
                        shape
                    } else {
                        return Err(PhysicsWorldInitError::MeshNotConvex);
                    }
                }
                GltfOptimizedShape::None => {
                    let mut maybe_indices: Option<Vec<[u32; 3]>> = None;
                    let mut maybe_vertices: Option<Vec<Point3<f32>>> = None;
                    for primitive in mesh.primitives() {
                        let indices_accesor = match primitive.indices() {
                            Some(accessor) => accessor,
                            None => {
                                return Err(PhysicsWorldInitError::NoPrimitiveAccessorForTrimesh)
                            }
                        };

                        let indices_bytes = if let Ok(indices_bytes) =
                            access_gltf_bytes(gltf_bytes, &indices_accesor)
                        {
                            indices_bytes
                        } else {
                            return Err(PhysicsWorldInitError::CouldntAccessBytes);
                        };
                        let mut indices: Vec<[u32; 3]> =
                            Vec::with_capacity(indices_accesor.count() / 3);

                        match indices_accesor.data_type() {
                            GltfDataType::U16 => {
                                let flattened_indices: Vec<u16> = indices_bytes
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
                                for face_u16 in chunked_indices {
                                    indices.push([
                                        u32::from(face_u16[0]),
                                        u32::from(face_u16[1]),
                                        u32::from(face_u16[2]),
                                    ]);
                                }
                                maybe_indices = Some(indices);
                            }
                            GltfDataType::U32 => {
                                let flattened_indices: Vec<u32> = indices_bytes
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
                                for face_u32 in chunked_indices {
                                    indices.push([face_u32[0], face_u32[1], face_u32[2]]);
                                }
                                maybe_indices = Some(indices);
                            }
                            _ => {
                                return Err(PhysicsWorldInitError::InvalidIndicesDataType);
                            }
                        };
                        match primitive.get(&PrimitiveSemantic::Positions) {
                            None => {
                                return Err(PhysicsWorldInitError::NoVertexPositionsAccessor);
                            }
                            Some(vertex_positions_accessor) => {
                                let positions_bytes = if let Ok(positions_bytes) =
                                    access_gltf_bytes(gltf_bytes, &vertex_positions_accessor)
                                {
                                    positions_bytes
                                } else {
                                    return Err(PhysicsWorldInitError::CouldntAccessBytes);
                                };

                                let mut floats: Vec<f32> =
                                    Vec::with_capacity(positions_bytes.len() / 4);
                                for float_bytes in positions_bytes.chunks_exact(4) {
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
                                maybe_vertices = Some(vertices);
                            }
                        };
                    }
                    if maybe_indices.is_none() {
                        return Err(PhysicsWorldInitError::NoIndicesFound);
                    }
                    if maybe_vertices.is_none() {
                        return Err(PhysicsWorldInitError::NoVerticesFound);
                    }
                    let scaled_trimesh: TriMesh = TriMesh::new(
                        maybe_vertices
                            .expect("Trimesh vertices were None despite asserting they weren't!"),
                        maybe_indices
                            .expect("Trimesh indices were None despite asserting they weren't!"),
                    )
                    .scaled(&Vector3::new(scale[0], scale[1], scale[2]));
                    SharedShape::trimesh(
                        scaled_trimesh.vertices().to_vec(),
                        scaled_trimesh.indices().to_vec(),
                    )
                }
            };

            let mut collider_builder = ColliderBuilder::new(collider_silhouette);
            if matches!(body_type, GltfBodyType::Dynamic) {
                collider_builder = collider_builder.mass(node_extras.sim_settings.physics.mass);
            }

            let rb_handle = self.rigid_body_set.insert(rigid_body_builder.build());
            let _col_handle = self.collider_set.insert_with_parent(
                collider_builder.build(),
                rb_handle,
                &mut self.rigid_body_set,
            );
            if !node_extras.sim_settings.physics.is_anonymous {
                self.named_rigid_bodies.insert(mesh_name.clone(), rb_handle);
            }
        } else {
            // Create a sensor
            let sensor_name = match node.name() {
                Some(name) => name,
                None => return Err(PhysicsWorldInitError::UnnamedNode),
            };
            let sensor_name = String::from(sensor_name);

            let base_scale = node_extras.sim_settings.physics.base_scale;
            let sensor_silhouette = match node_extras.sim_settings.physics.optimized_shape {
                GltfOptimizedShape::Cuboid => {
                    let cuboid_half_dimensions =
                        base_scale.component_mul(&Vector3::from(scale)) / 2.0;
                    SharedShape::cuboid(
                        cuboid_half_dimensions.x,
                        cuboid_half_dimensions.y,
                        cuboid_half_dimensions.z,
                    )
                }
                GltfOptimizedShape::Sphere => {
                    let ball_dimensions = base_scale.component_mul(&Vector3::from(scale));
                    SharedShape::ball(ball_dimensions.x / 2.0)
                }
                _ => return Err(PhysicsWorldInitError::CantAccessBlob),
            };
            let collider_builder = ColliderBuilder::new(sensor_silhouette)
                .position(global_isometry)
                .sensor(true);

            let sensor_handle = self.collider_set.insert(collider_builder.build());
            self.named_sensors.insert(sensor_name, sensor_handle);
        }

        visited_nodes.insert(node.index(), ());
        Ok(())
    }

    /// Load physics-enabled objects from a Perigee-enabled
    /// glTF into the physics world.
    ///
    /// Note: Nodes that are children of others will be ignored.
    pub fn load_from_gltf(&mut self, gltf: &Gltf) -> Result<(), PhysicsWorldInitError> {
        let gltf_bytes = match &gltf.blob {
            Some(bytes) => bytes,
            None => {
                return Err(PhysicsWorldInitError::CantAccessBlob);
            }
        };

        let mut visited_nodes: HashMap<usize, ()> = HashMap::new();
        if let Some(scene) = gltf.scenes().next() {
            for node in scene.nodes() {
                self.visit_node(&node, &Isometry::identity(), gltf_bytes, &mut visited_nodes)?;
            }
        }

        return Ok(());
    }

    pub fn listen_to_collider<L: ColliderEventListener + 'static>(
        &mut self,
        handle: ColliderHandle,
        listener: L,
    ) {
        let wrapped_listener = Box::new(listener);
        if let Some(handlers) = self.collider_event_handlers.get_mut(&handle) {
            handlers.push(wrapped_listener);
        } else {
            self.collider_event_handlers
                .insert(handle, vec![wrapped_listener]);
        }
    }

    pub fn rekey_listeners(&mut self, old_handle: ColliderHandle, new_handle: ColliderHandle) {
        if let Some(listeners) = self.collider_event_handlers.remove(&old_handle) {
            self.collider_event_handlers.insert(new_handle, listeners);
        }
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
            Some(&mut self.query_pipeline),
            &(),
            self.contact_event_manager.event_collector(),
        );

        while let Ok(collision_event) = self.contact_event_manager.get_collider_event() {
            match collision_event {
                CollisionEvent::Started(collider_a, collider_b, collision_type) => {
                    if collision_type != CollisionEventFlags::SENSOR {
                        if let Some(handlers) = self.collider_event_handlers.get_mut(&collider_a) {
                            for handler in handlers {
                                handler.on_collision_start(&collider_b);
                            }
                        };
                        if let Some(handlers) = self.collider_event_handlers.get_mut(&collider_b) {
                            for handler in handlers {
                                handler.on_collision_start(&collider_a);
                            }
                        }
                    } else {
                        if let Some(handlers) = self.collider_event_handlers.get_mut(&collider_a) {
                            for handler in handlers {
                                handler.on_intersection_start(&collider_b);
                            }
                        };
                        if let Some(handlers) = self.collider_event_handlers.get_mut(&collider_b) {
                            for handler in handlers {
                                handler.on_intersection_start(&collider_a);
                            }
                        }
                    }
                }
                CollisionEvent::Stopped(collider_a, collider_b, collision_type) => {
                    if collision_type != CollisionEventFlags::SENSOR {
                        if let Some(handlers) = self.collider_event_handlers.get_mut(&collider_a) {
                            for handler in handlers {
                                handler.on_collision_end(&collider_b);
                            }
                        };
                        if let Some(handlers) = self.collider_event_handlers.get_mut(&collider_b) {
                            for handler in handlers {
                                handler.on_collision_end(&collider_a);
                            }
                        }
                    } else {
                        if let Some(handlers) = self.collider_event_handlers.get_mut(&collider_a) {
                            for handler in handlers {
                                handler.on_intersection_end(&collider_b);
                            }
                        };
                        if let Some(handlers) = self.collider_event_handlers.get_mut(&collider_b) {
                            for handler in handlers {
                                handler.on_intersection_end(&collider_a);
                            }
                        }
                    }
                }
            }
        }
        while let Ok(contact_force_event) = self.contact_event_manager.get_contact_force_event() {
            if let Some(handlers) = self
                .collider_event_handlers
                .get_mut(&contact_force_event.collider1)
            {
                for handler in handlers {
                    handler.on_contact_force_event(
                        &contact_force_event.collider2,
                        contact_force_event,
                    );
                }
            };
            if let Some(handlers) = self
                .collider_event_handlers
                .get_mut(&contact_force_event.collider2)
            {
                for handler in handlers {
                    handler.on_contact_force_event(
                        &contact_force_event.collider1,
                        contact_force_event,
                    );
                }
            }
        }
    }

    /// Recover the handle from a RigidBody using its `user_data` field.
    pub unsafe fn get_body_handle(body: &RigidBody) -> RigidBodyHandle {
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
    pub unsafe fn store_handle_in_body(handle: &RigidBodyHandle, body: &mut RigidBody) {
        let handle_parts = handle.into_raw_parts();
        let handle_index = handle_parts.0;
        let handle_generation = handle_parts.1;
        body.user_data = u128::from(handle_index).rotate_left(32) | u128::from(handle_generation);
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

            unsafe {
                PhysicsWorld::store_handle_in_body(&handle, body);

                let recovered_handle = PhysicsWorld::get_body_handle(body);
                assert_eq!(handle, recovered_handle);
            }
        }
    }
}
