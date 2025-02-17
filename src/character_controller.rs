use std::time::Duration;

use bevy::prelude::*;
use avian3d::prelude::*;

use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*; 

mod character_camera;

use character_camera::CameraState;

use crate::{
    animation_handler::{AnimationHandler, ResourceHandle}, asset_loader::{AssetLoadingState, CharacterHandle}, combat_manager::{AttackType, CombatAction}
};

#[derive(Component)]
pub struct PlayerCharacter;

pub fn plugin(app: &mut App) {
    app
        .add_plugins(character_camera::plugin)
        .add_plugins((
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(Update, (
            apply_controls
        ).run_if(in_state(AssetLoadingState::Loaded)));
}

pub fn setup(
    mut commands: Commands,
    dogman: Res<CharacterHandle>
) {

    let id = commands.spawn((
        PlayerCharacter,
        AnimationHandler {
            current_animation: *dogman.animation_name_reference.get("Idle").unwrap(),
            resource_type: ResourceHandle::Character
        },
        SceneRoot(dogman.scene.clone()), 
        Transform::from_xyz(0.0, 4.0, 0.0),
        RigidBody::Dynamic,
        Collider::cylinder(1.5, 7.3),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(1.4, 7.2))
    )).id();


    
    println!("player id: {:?}", id);
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
            height: 15.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            fall_extra_gravity: 30.0,
            ..Default::default()
        });
    }
}