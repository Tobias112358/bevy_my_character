use bevy::prelude::*;
use avian3d::prelude::*;

use crate::{animation_handler::{AnimationHandler, ResourceHandle}, asset_loader::{AssetLoadingState, EnemyHandle}, character_controller::PlayerCharacter};

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(Update, (move_alien).run_if(in_state(AssetLoadingState::Loaded)));
}

#[derive(Component)]
pub struct Enemy;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut alien_assets: ResMut<EnemyHandle>
) {
    let id = commands.spawn((
        Enemy,
        AnimationHandler {
            current_animation: 0,
            resource_type: ResourceHandle::Enemy
        },
        SceneRoot(alien_assets.scene.clone()),
        Transform::from_xyz(0.0, 8.0, 4.0),
        RigidBody::Dynamic,
        Collider::cuboid(1.5, 7.3, 1.5),
        LockedAxes::ROTATION_LOCKED
    )).id();

    println!("enemy id: {:?}", id);
}

pub fn move_alien(
    mut query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<PlayerCharacter>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let Ok(target) = player_query.get_single() else {
            return;
        };

        let mut direction_translation = target.translation.clone();

        direction_translation.y = 0.0;

        transform.look_at(direction_translation, Vec3::Y);
        let forward = transform.forward();
        transform.translation += forward * 5.0 * time.delta_secs();
    }
}