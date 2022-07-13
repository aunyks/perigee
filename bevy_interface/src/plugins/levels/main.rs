use crate::components::LevelObject;
use crate::states::GameLevel;
use crate::systems::pausing::{pause_game, resume_game};
use crate::systems::teardown_game_level;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::{ActiveCamera, Camera3d};
#[allow(unused_imports)]
use bevy_fly_camera::FlyCamera;
use perigee_single_player::level_0::Sim;

#[derive(Component)]
struct PlayerCapsule;

#[derive(Component)]
struct PlayerHead;

#[derive(Component)]
struct PlayerAnimations(Vec<Handle<AnimationClip>>);

/// This plugin manages gameplay for the main game level
pub struct GameLevel0;

impl Plugin for GameLevel0 {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameLevel::Zero).with_system(setup_level))
            .add_system_set(SystemSet::on_pause(GameLevel::Zero).with_system(pause_game))
            .add_system_set(SystemSet::on_resume(GameLevel::Zero).with_system(resume_game))
            .add_system_set(
                SystemSet::on_update(GameLevel::Zero)
                    // Single-shot systems that are hard to run as
                    // setup systems
                    .with_system(load_gltf_camera)
                    // .with_system(start_player_animations)
                    // Process input and pass to the simulation
                    // for handling
                    .with_system(rotate_body)
                    .with_system(rotate_head)
                    .with_system(move_body)
                    .with_system(crouch_body)
                    .with_system(jump_body)
                    // Step the simulation
                    .with_system(step_simulation)
                    // Visualize the new simulation state
                    .with_system(sync_player_capsule)
                    .with_system(sync_head),
            )
            .add_system_set(
                SystemSet::on_exit(GameLevel::Zero)
                    .with_system(teardown_game_level)
                    .with_system(teardown_main_game_level),
            );
    }
}

// Set up physics and graphics
fn setup_level(
    mut commands: Commands,
    mut game: ResMut<Sim<'static>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    game.initialize().expect("Could not initialize game!");

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.25f32,
    });

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 15.0, 0.0),
        point_light: PointLight {
            intensity: 100000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            radius: 100000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    let player_global_isometry = game.player.body_isometry();
    let player_trans = player_global_isometry.translation;
    let player_quat = player_global_isometry.rotation;
    let cap_rad = 0.5;
    let cap_halfheight = 0.5;
    let rings = 3;
    let lats = 6;
    let longs = 6;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: cap_rad,
                depth: cap_halfheight * 2.0,
                rings,
                latitudes: lats,
                longitudes: longs,
                uv_profile: shape::CapsuleUvProfile::Fixed,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(player_trans.x, player_trans.y, player_trans.z)
                .with_rotation(Quat::from_xyzw(
                    player_quat.i,
                    player_quat.j,
                    player_quat.k,
                    player_quat.w,
                )),
            ..Default::default()
        })
        .insert(PlayerCapsule)
        .insert(LevelObject);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -cap_rad,
                max_x: cap_rad,
                min_y: -0.1,
                max_y: 0.1,
                min_z: -cap_rad,
                max_z: 0.0,
            })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .insert(PlayerHead)
        .insert(LevelObject)
        .with_children(|head| {
            head.spawn_bundle(TransformBundle::default())
                .insert(LevelObject)
                .with_children(|parent| {
                    parent.spawn_scene(ass.load("gltf/player-camera.glb#Scene0"));
                });
        });

    commands
        .spawn()
        .insert(PlayerAnimations(vec![
            ass.load("gltf/player-camera.glb#Animation0")
        ]))
        .insert(LevelObject);

    // commands
    //     .spawn_bundle(PerspectiveCameraBundle {
    //         transform: Transform::from_xyz(0.0, 3.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..Default::default()
    //     })
    //     .insert(FlyCamera::default())
    //     .insert(LevelObject);

    // to be able to position our 3d model:
    // spawn a parent entity with a Transform and GlobalTransform
    // and spawn our gltf as a scene under it
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(LevelObject)
        .with_children(|parent| {
            parent.spawn_scene(ass.load("gltf/levels/0/physics-world.glb#Scene0"));
        });
}

fn load_gltf_camera(
    mut commands: Commands,
    cameras_3d: Query<(&Camera, Entity), With<Camera3d>>,
    mut active_camera_3d: ResMut<ActiveCamera<Camera3d>>,
    mut cam_loaded: Local<bool>,
) {
    if !*cam_loaded {
        if let Some((gltf_camera, gltf_cam_entity)) = cameras_3d.iter().next() {
            commands.entity(gltf_cam_entity).with_children(|gltf_cam| {
                let true_cam = gltf_cam
                    .spawn_bundle(PerspectiveCameraBundle {
                        camera: gltf_camera.clone(),
                        perspective_projection: PerspectiveProjection {
                            near: gltf_camera.near,
                            far: gltf_camera.far,
                            ..Default::default()
                        },
                        ..PerspectiveCameraBundle::new_3d()
                    })
                    .insert(LevelObject)
                    .id();
                active_camera_3d.set(true_cam);
            });
            *cam_loaded = true;
        }
    }
}

fn step_simulation(mut game: ResMut<Sim<'static>>, time: Res<Time>) {
    game.step(time.delta_seconds());
}

fn sync_player_capsule(
    game: Res<Sim<'static>>,
    mut player_query: Query<&mut Transform, With<PlayerCapsule>>,
) {
    let player_global_isometry = game.player.body_isometry();
    let player_trans = player_global_isometry.translation;
    let player_quat = player_global_isometry.rotation;
    for mut transform in player_query.iter_mut() {
        transform.translation.x = player_trans.x;
        transform.translation.y = player_trans.y;
        transform.translation.z = player_trans.z;
        transform.rotation = Quat::from_xyzw(
            player_quat.coords.x,
            player_quat.coords.y,
            player_quat.coords.z,
            player_quat.coords.w,
        );
    }
}

fn sync_head(game: Res<Sim<'static>>, mut head_query: Query<&mut Transform, With<PlayerHead>>) {
    let head_global_isometry = game.player.body_isometry() * game.player.head_isometry();

    let head_trans = head_global_isometry.translation;
    let head_quat = head_global_isometry.rotation;
    for mut transform in head_query.iter_mut() {
        transform.translation.x = head_trans.x;
        transform.translation.y = head_trans.y;
        transform.translation.z = head_trans.z;
        transform.rotation = Quat::from_xyzw(
            head_quat.coords.x,
            head_quat.coords.y,
            head_quat.coords.z,
            head_quat.coords.w,
        );
    }
}

fn rotate_body(
    mut game: ResMut<Sim<'static>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
) {
    let mut right_magnitude = 0.0;

    for mouse_motion_event in mouse_motion_events.iter() {
        let delta_x = mouse_motion_event.delta.x;
        right_magnitude = delta_x * 0.7;
    }

    // Process gamepad input because they have precedence
    // over keyboard input
    for gamepad in gamepads.iter().cloned() {
        if let Some(magnitude) = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickX)) {
            if magnitude != 0f32 {
                right_magnitude = magnitude * 11f32;
            }
        }
    }
    game.input.set_rotate_right(right_magnitude);
}

fn rotate_head(
    mut game: ResMut<Sim<'static>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
) {
    let mut up_magnitude = 0.0;

    for mouse_motion_event in mouse_motion_events.iter() {
        let delta_y = mouse_motion_event.delta.y;
        up_magnitude = -delta_y / 10.0;
    }

    // Process gamepad input because they have precedence
    // over keyboard input
    for gamepad in gamepads.iter().cloned() {
        if let Some(magnitude) = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickY)) {
            if magnitude != 0f32 {
                up_magnitude = magnitude * 11f32;
            }
        }
    }
    game.input.set_rotate_up(up_magnitude);
}

fn move_body(mut game: ResMut<Sim<'static>>, keyboard_input: Res<Input<KeyCode>>) {
    let mut right_magnitude = 0.0;
    let mut forward_back_magnitude = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        right_magnitude -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        right_magnitude += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        forward_back_magnitude -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        forward_back_magnitude += 1.0;
    }
    game.input.set_move_right(right_magnitude);
    game.input.set_move_forward(forward_back_magnitude);
}

fn crouch_body(mut game: ResMut<Sim<'static>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::M) {
        game.input.set_crouch(true);
    } else {
        game.input.set_crouch(false);
    }
}

fn jump_body(mut game: ResMut<Sim<'static>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        game.input.set_jump(true);
    }
}

fn teardown_main_game_level(mut commands: Commands) {
    commands.remove_resource::<AmbientLight>();
}
