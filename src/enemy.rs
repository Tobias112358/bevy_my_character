use bevy::prelude::*;
use avian3d::prelude::*;

use crate::{animation_handler::{AnimationHandler, ResourceHandle}, asset_loader::{AssetLoadingState, EnemyHandle}, character_controller::PlayerCharacter, combat_manager::{
    AttackMode, AttackType, CombatAction, CombatManager, Weapon, WeaponStats
}};

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
        Transform::from_xyz(0.0, 8.0, 8.0),
        RigidBody::Dynamic,
        Collider::cuboid(1.5, 7.3, 1.5),
        LockedAxes::ROTATION_LOCKED,
        CombatManager {
            in_attack: false,
            last_attack_cooldown: 0.0,
            weapon: Weapon {
                weapon_entity: None,
                weapon_stats: WeaponStats {
                    light_attack : CombatAction::new(AttackType::Light, 0.5, 0.25, 0.55, 10.),
                    ..default()
                }
            }
        }
    )).id();

    println!("enemy id: {:?}", id);
}

pub fn move_alien(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Enemy>>,
    player_query: Query<&Transform, (With<PlayerCharacter>, Without<Enemy>)>,
    time: Res<Time>,
) {

    let Ok(target) = player_query.get_single() else {
        return;
    };


    for (entity, mut transform) in query.iter_mut() {

        if transform.translation.distance(target.translation) < 6.0 {
            commands.entity(entity).insert(AttackMode);
            continue;
        }
        
        let mut direction_translation = target.translation.clone();

        direction_translation.y = 0.0;

        let looking_at = transform.looking_at(direction_translation, Vec3::Y);

        transform.rotation = looking_at.rotation;
        let forward = transform.forward();
        transform.translation += forward * 5.0 * time.delta_secs();
    }
}