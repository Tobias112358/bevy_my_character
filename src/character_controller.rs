use std::time::Duration;

use bevy::{animation::RepeatAnimation, prelude::*, render::camera};
use avian3d::prelude::*;

use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*; 

use bevy::input::mouse::MouseMotion;

use crate::asset_loader::{AssetLoadingState, CharacterHandle};

#[derive(Component)]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct CameraState {
    pub rotation: Quat,
    pub yaw: f32
}

pub fn plugin(app: &mut App) {
    app
        .add_plugins((
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(Update, (
            move_camera, 
            add_animation_transition_to_player,
            cycle_animation,
            apply_controls,
            //rotate_camera,
            //camera_look_at_player,
            animation_handler,
            //camera_follow_player
            //focus_camera_on_player
        ).run_if(in_state(AssetLoadingState::Loaded))

        )
        .add_systems(FixedUpdate, (
            camera_move_to_player,
            camera_rotate_to_player,
            rotate_camera
        ).run_if(in_state(AssetLoadingState::Loaded)))
        ;
}

pub fn setup(
    mut commands: Commands,
    dogman: Res<CharacterHandle>
) {
    let mut camera_rotation = Quat::from_rotation_y(std::f32::consts::PI);

    camera_rotation *= Quat::from_rotation_x(-std::f32::consts::FRAC_PI_8);

    commands.spawn((
        PlayerCharacter,
        SceneRoot(dogman.scene.clone()), 
        Transform::from_xyz(0.0, 4.0, 0.0),
        RigidBody::Dynamic,
        Collider::cylinder(1.5, 7.3),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(1.4, 7.2))
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 15.0, -12.0)).with_rotation(camera_rotation),
        CameraState {
            rotation: Quat::IDENTITY,
            yaw: 0.
        },
    ));
}

pub fn move_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>, 
    mut camera_transform_query: Query<&mut Transform, With<Camera3d>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let mut camera_transform = camera_transform_query.single_mut();
    
    // let mut frame_transation = Vec3::ZERO;

    // if input.pressed(KeyCode::KeyW) {
    //     frame_transation += camera_transform.forward().as_vec3();
    // } else if input.pressed(KeyCode::KeyS) {
    //     frame_transation -= camera_transform.forward().as_vec3();
    // }
    // if input.pressed(KeyCode::KeyD) {
    //     frame_transation += camera_transform.right().as_vec3();
    // } else if input.pressed(KeyCode::KeyA) {
    //     frame_transation -= camera_transform.right().as_vec3();
    // }

    // if input.pressed(KeyCode::KeyE) {
    //     frame_transation += camera_transform.up().as_vec3();
    // } else if input.pressed(KeyCode::KeyQ) {
    //     frame_transation -= camera_transform.up().as_vec3();
    // }

    // let speed = 10.0;

    // camera_transform.translation += frame_transation *  speed * time.delta_secs();
}

fn add_animation_transition_to_player(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    character_handle: Res<CharacterHandle>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, character_handle.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(character_handle.animation_graph.clone()))
            .insert(transitions);
    }
}

fn cycle_animation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    character_handle: Res<CharacterHandle>,
    mut current_animation: Local<usize>,
) {
    // for (mut anim_player, mut transitions) in &mut animation_players {
    //     // let Some((&playing_animation_index, _)) = anim_player.playing_animations().next() else {
    //     //     continue;
    //     // };

    //     if keyboard_input.just_pressed(KeyCode::Space) {
    //         *current_animation = (*current_animation + 1) % character_handle.animations.len();

    //         transitions
    //             .play(
    //                 &mut anim_player,
    //                 character_handle.animations[*current_animation],
    //                 Duration::from_millis(250),
    //             )
    //             .repeat();

    //     }
    // }
}

fn rotate_camera(
    mut query: Query<(&mut Transform, &mut CameraState), With<Camera3d>>,
    player_query: Query<&Transform, (With<PlayerCharacter>, Without<Camera3d>)>,
    mut mouse_motion: EventReader<MouseMotion>
) {
    let Ok((mut transform, mut camera_state)) = query.get_single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;

        let mut y_rot: Quat;
        if yaw > 0.05 {
            y_rot = Quat::from_rotation_y(0.05);
        } else if yaw < -0.05 {
            y_rot = Quat::from_rotation_y(-0.05);
        } else {
            y_rot = Quat::from_rotation_y(yaw);
        }

        println!("{}", pitch);

        camera_state.rotation *= y_rot;// * Quat::from_rotation_x(pitch);
        //camera_state.yaw += yaw;
        //transform.rotate_local_y(yaw);

        //println!("{:?}", camera_state.rotation);

        //transform.translate_around(player_transform.translation, camera_state.rotation);
    }
}

fn camera_move_to_player(
    player_transform_query: Query<&Transform, With<PlayerCharacter>>,
    mut camera_transform_query: Query<(&mut Transform, &CameraState), (With<Camera3d>, Without<PlayerCharacter>)>,
    mut mouse_motion: EventReader<MouseMotion>
) {
    let Ok(player_transform) = player_transform_query.get_single() else {
        return;
    };

    let Ok((mut camera_transform, camera_state)) = camera_transform_query.get_single_mut() else {
        return;
    };

    
    let mut new_cam_pos = Vec3::new(0.0, 8.0, -15.0);

    
    new_cam_pos = camera_state.rotation.mul_vec3(new_cam_pos);
    new_cam_pos += player_transform.translation;

    let mut yaw = 0.0;
    for motion in mouse_motion.read() {
        yaw += -motion.delta.x * 0.001;

        //println!("{}", yaw);


    }

    camera_transform.translation = camera_transform.translation.lerp(new_cam_pos, 0.2 + yaw.abs());
    
}

fn camera_rotate_to_player(
    player_transform_query: Query<&Transform, With<PlayerCharacter>>,
    mut camera_transform_query: Query<(&mut Transform, &CameraState), (With<Camera3d>, Without<PlayerCharacter>)>
) {
    let Ok(player_transform) = player_transform_query.get_single() else {
        return;
    };

    let Ok((mut camera_transform, camera_state)) = camera_transform_query.get_single_mut() else {
        return;
    };

    let player_transform_overhead = Vec3::new(player_transform.translation.x, player_transform.translation.y + 2.0, player_transform.translation.z);

    let look_at_player = camera_transform.looking_at(player_transform_overhead, Dir3::Y);

    camera_transform.rotation = camera_transform.rotation.slerp(look_at_player.rotation, 0.5);
}

fn animation_handler(
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    character_handle: Res<CharacterHandle>,
    rigidbody_query: Query<(&LinearVelocity, &Transform), With<PlayerCharacter>>,
    mut current_animation: Local<usize>,
) {
    for (mut anim_player, mut transitions) in &mut animation_players {
        let Ok((velocity, transform)) = rigidbody_query.get_single() else {
            continue;
        };

        if velocity.length() > 0.25 {
            if *current_animation != 2 {
                *current_animation = 2;
                transitions
                .play(
                    &mut anim_player,
                    character_handle.animations[*current_animation],
                    Duration::from_millis(250),
                )
                .repeat();
            }
            
        } else {
            if *current_animation != 0 {
                *current_animation = 0;
            transitions
                .play(
                    &mut anim_player,
                    character_handle.animations[*current_animation],
                    Duration::from_millis(250),
                )
                .repeat();
            }
        }
    }
}


fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&mut Transform, &mut TnuaController), With<PlayerCharacter>>,
    camera_query: Query<&CameraState, (With<Camera3d>, Without<PlayerCharacter>)>
) {
    let Ok((mut transform, mut controller)) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction += Vec3::X;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction -= Vec3::X;
    }

    let Ok(camera_state) = camera_query.get_single() else {
        return;
    };

    direction = camera_state.rotation.mul_vec3(direction);

    if direction != Vec3::ZERO {
        let face_direction = transform.looking_to(-direction, Dir3::Y);

        transform.rotation = transform.rotation.slerp(face_direction.rotation, 0.05);
    }
    

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 20.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.5,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 4.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }
}