use bevy::{input::{keyboard::Key, mouse::MouseMotion}, prelude::*};

use crate::{
    asset_loader::AssetLoadingState,
    character_controller::PlayerCharacter, enemy::Enemy
};

#[derive(Component)]
pub struct CameraState {
    pub rotation: Quat,
    pub camera_distance: f32,
    pub target_entity: Option<Entity>,
}

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(FixedUpdate, (
            camera_position_and_rotate,
            control_camera,
            find_target
        ).run_if(in_state(AssetLoadingState::Loaded)));
}

pub fn setup(
    mut commands: Commands
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 15.0, -12.0)).with_rotation(Quat::IDENTITY),
        CameraState {
            rotation: Quat::IDENTITY,
            camera_distance: 20.,
            target_entity: None
        },
    ));
}

fn camera_position_and_rotate(
    player_transform_query: Query<&Transform, With<PlayerCharacter>>,
    mut camera_transform_query: Query<(&mut Transform, &CameraState), (With<Camera3d>, Without<PlayerCharacter>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<PlayerCharacter>, Without<Camera3d>)>
) {
    let Ok(player_transform) = player_transform_query.get_single() else {
        return;
    };

    let Ok((mut camera_transform, camera_state)) = camera_transform_query.get_single_mut() else {
        return;
    };

    

    let look_at_position = player_transform.translation + Vec3::new(0.0, 5.0, 0.0);

    if let Some(target_entity) = camera_state.target_entity {
        let Ok(target_transform) = enemy_query.get(target_entity) else {
            return;
        };
        let midpoint = look_at_position + (target_transform.translation - look_at_position)/2.;

        camera_transform.look_at(midpoint, Vec3::Y);
    } else {
        camera_transform.rotation = camera_state.rotation;
    }

    
    camera_transform.translation = look_at_position - camera_transform.forward() * camera_state.camera_distance;
}

fn control_camera(
    mut query: Query<(&Transform, &mut CameraState), With<Camera3d>>,
    mut mouse_motion: EventReader<MouseMotion>
) {
    let Ok((camera_transform, mut camera_state)) = query.get_single_mut() else {
        return;
    };

    if let Some(_target_entity) = camera_state.target_entity {

        return;
    } else {
        let (mut yaw, mut pitch, mut roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        for motion in mouse_motion.read() {
    
            let delta_yaw = -motion.delta.x * 0.01;
            let delta_pitch = -motion.delta.y * 0.007;
            let delta_roll: f32 = 0.0;
    
            (yaw, pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    
            yaw = yaw + delta_yaw;
            pitch = (pitch + delta_pitch).clamp(
                -std::f32::consts::FRAC_PI_2 + 0.6,
                std::f32::consts::FRAC_PI_2 - 0.6
            );
            roll = roll + delta_roll;
    
        }
        
        camera_state.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn find_target(
    keyboard: Res<ButtonInput<KeyCode>>, 
    enemy_query: Query<Entity, With<Enemy>>,
    mut camera_state_query: Query<&mut CameraState, With<Camera3d>>
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        println!("pressed!");
        let Ok(mut camera_state) = camera_state_query.get_single_mut() else {
            return;
        };
        println!("HERE!");
        if camera_state.target_entity.is_some() {
            camera_state.target_entity = None;
            println!("HERE@");
            return;
        } else {
            for entity in enemy_query.iter() {
                camera_state.target_entity = Some(entity);
            }
            
            println!("HERE {:?}", camera_state.target_entity);
        }
    }
}