use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    asset_loader::AssetLoadingState,
    character_controller::PlayerCharacter
};

#[derive(Component)]
pub struct CameraState {
    pub rotation: Quat
}

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(FixedUpdate, (
            camera_move_to_player,
            camera_rotate_to_player,
            rotate_camera
        ).run_if(in_state(AssetLoadingState::Loaded)));
}

pub fn setup(
    mut commands: Commands
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 15.0, -12.0)).with_rotation(Quat::IDENTITY),
        CameraState {
            rotation: Quat::IDENTITY
        },
    ));
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
    //println!("{:?}", camera_state.rotation);
    new_cam_pos += player_transform.translation;

    let mut yaw = 0.0;
    for motion in mouse_motion.read() {
        yaw += -motion.delta.x * 0.001;
    }

    camera_transform.translation = camera_transform.translation.lerp(new_cam_pos, 0.2 + yaw.abs());
    
}

fn camera_rotate_to_player(
    player_transform_query: Query<&Transform, With<PlayerCharacter>>,
    mut camera_transform_query: Query<&mut Transform, (With<Camera3d>, Without<PlayerCharacter>)>
) {
    let Ok(player_transform) = player_transform_query.get_single() else {
        return;
    };

    let Ok(mut camera_transform) = camera_transform_query.get_single_mut() else {
        return;
    };

    let player_transform_overhead = Vec3::new(player_transform.translation.x, player_transform.translation.y + 2.0, player_transform.translation.z);

    let look_at_player = camera_transform.looking_at(player_transform_overhead, Dir3::Y);

    camera_transform.rotation = camera_transform.rotation.slerp(look_at_player.rotation, 0.5);
}

fn rotate_camera(
    mut query: Query<&mut CameraState, With<Camera3d>>,
    mut mouse_motion: EventReader<MouseMotion>
) {
    let Ok(mut camera_state) = query.get_single_mut() else {
        return;
    };

    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;

        if yaw > 0.15 {
            camera_state.rotation *= Quat::from_rotation_y(0.15);
        } else if yaw < -0.15 {
            camera_state.rotation *= Quat::from_rotation_y(-0.15);
        } else {
            camera_state.rotation *= Quat::from_rotation_y(yaw);
        }
    }
}