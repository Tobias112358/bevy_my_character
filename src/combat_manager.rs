use std::process::Command;

use avian3d::prelude::{collider, Collider, Collisions, Sensor};
use bevy::{input::mouse::MouseButtonInput, prelude::*, state::commands};
use rand::Rng;

use crate::{
    asset_loader::AssetLoadingState,
    character_controller::PlayerCharacter, health_manager::HealthModifyEvent
};

pub fn plugin(app: &mut App) {
    app
        .add_systems(Update, (
            setup,
            player_attack_trigger,
            attack_time_system,
            npc_attack
        ).run_if(in_state(AssetLoadingState::Loaded)))
        .add_systems(PostUpdate, (
            update_combat_manager_after_attack,
            setup_attack_colliders,
        ).run_if(in_state(AssetLoadingState::Loaded)))
        .add_observer(in_attack);
}

#[derive(Component)]
pub struct AttackMode;


#[derive(Component)]
pub struct AttackCollider {
    pub parent: Entity
}

#[derive(Event)]
struct AttackEvent {
    damage: f32,
    attacker: Entity
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum AttackType {
    Light,
    Heavy
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum AttackState {
    Windup,
    Attack,
    Cooldown,
    #[default]
    Idle
}

#[derive(Default, Debug, Clone)]
pub struct CombatTimer {
    timer: Timer
}

#[derive(Component, Debug, Clone)]
pub struct CombatAction {
    pub attack_type: AttackType,
    pub attack_state: AttackState,
    pub combat_timer: CombatTimer,
    pub windup: f32,
    pub attack_time: f32,
    pub cooldown: f32,
    pub damage: f32
}

impl CombatAction {
    pub fn new(attack_type: AttackType, windup: f32, attack_time: f32, cooldown: f32, damage: f32) -> Self {
        Self {
            attack_type,
            attack_state: AttackState::Idle,
            combat_timer: CombatTimer::default(),
            windup,
            attack_time,
            cooldown,
            damage
        }
    }
}

#[derive(Debug)]
pub struct WeaponStats {
    pub light_attack: CombatAction,
    pub heavy_attack: CombatAction
}

impl Default for WeaponStats {
    fn default() -> Self {
        Self {
            light_attack: CombatAction {
                attack_type: AttackType::Light,
                attack_state: AttackState::Idle,
                combat_timer: CombatTimer::default(),
                windup: 0.2,
                attack_time: 0.1,
                cooldown: 0.45,
                damage: 2.0
            },
            heavy_attack: CombatAction {
                attack_type: AttackType::Heavy,
                attack_state: AttackState::Idle,
                combat_timer: CombatTimer::default(),
                windup: 0.4,
                attack_time: 0.2,
                cooldown: 1.0,
                damage: 4.0
            }
        }
    }
}

#[derive(Component, Debug)]
#[require(Transform)]
pub struct Weapon {
    pub weapon_entity: Option<Entity>,
    pub weapon_stats: WeaponStats
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            weapon_entity: None,
            weapon_stats: WeaponStats::default()
        }
    }
}

#[derive(Component, Debug)]
#[require(Weapon)]
pub struct CombatManager {
    pub weapon: Weapon,
    pub last_attack_cooldown: f32,
    pub in_attack: bool
}

fn setup(
    mut commands: Commands,
    player_query: Query<Entity, Added<PlayerCharacter>>
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    commands.entity(player).insert(CombatManager {
        weapon: Weapon {
            weapon_entity: None,
            weapon_stats: WeaponStats::default()
        },
        last_attack_cooldown: 0.0,
        in_attack: false,
    });
}

fn player_attack_trigger(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut CombatManager, &mut Weapon), With<PlayerCharacter>>,
    mut mouse_click: EventReader<MouseButtonInput>
) {
    for event in mouse_click.read() {
        let Ok((player_entity, mut combat_manager, mut weapon)) = player_query.get_single_mut() else {
            return;
        };

        println!("{:?}",combat_manager);
        println!("button event: {:?}", event.button);
        // if not currently attacking
        if !combat_manager.in_attack {
            let mut combat_action: CombatAction;
            match event.button {
                MouseButton::Left => {
                    
                    println!("In left branch.");
                    combat_action = combat_manager.weapon.weapon_stats.light_attack.clone();

                }
                MouseButton::Right => {
                    println!("In right branch.");
                    combat_action = combat_manager.weapon.weapon_stats.heavy_attack.clone();
                }
                _ => {
                    continue;
                }
            }
            combat_action.attack_state = AttackState::Windup;
            combat_action.combat_timer.timer = Timer::from_seconds(combat_action.windup, TimerMode::Once);

            commands.entity(player_entity).insert(combat_action);

            combat_manager.in_attack = true;

            println!("{:?}",combat_manager);
        }
        
        println!("#################### ");
        
    }
}

fn attack_time_system(
    mut combat_action_query: Query<(Entity, &mut CombatAction, Option<&AttackMode>)>,

    time: Res<Time>,
    mut commands: Commands
) {
    for (entity,  mut combat_action, attack_mode_option) in combat_action_query.iter_mut() {
        match combat_action.attack_state {
            AttackState::Windup => {
                combat_action.combat_timer.timer.tick(time.delta());
                println!("Windup");
                if combat_action.combat_timer.timer.finished() {
                    combat_action.attack_state = AttackState::Attack;
                    combat_action.combat_timer.timer = Timer::from_seconds(combat_action.attack_time, TimerMode::Once);
                }
            }
            AttackState::Attack => {
                //Emit attack event
                commands.trigger(AttackEvent {
                    damage: combat_action.damage,
                    attacker: entity,
                    
                });
                combat_action.combat_timer.timer.tick(time.delta());
                //println!("Attack");
                if combat_action.combat_timer.timer.finished() {
                    combat_action.attack_state = AttackState::Cooldown;
                    combat_action.combat_timer.timer = Timer::from_seconds(combat_action.cooldown, TimerMode::Once);
                }
            }
            AttackState::Cooldown => {
                println!("Cooldown");
                
                combat_action.combat_timer.timer.tick(time.delta());
                if combat_action.combat_timer.timer.finished() {
                    combat_action.attack_state = AttackState::Idle;
                }
            }
            _ => {
                println!("idle");
                commands.entity(entity).remove::<CombatAction>();
                if attack_mode_option.is_some() {
                    commands.entity(entity).remove::<AttackMode>();
                }
            }
        }
    }
}

fn in_attack(
    trigger: Trigger<AttackEvent>,
    collider_query: Query<(Entity, &Collider, &AttackCollider), With<AttackCollider>>,
    collisions: Res<Collisions>,
    mut health_modify_event_writer: EventWriter<HealthModifyEvent>
) {
    trigger.event().damage;
    
    for (entity, collider, attack_collider) in collider_query.iter() {

        if attack_collider.parent != trigger.event().attacker {
            continue;
        }
        
        println!("Attacker: Attack collider - {:?}, Parent: {:?}", entity, attack_collider.parent);
        for colliding_with_hand in collisions.collisions_with_entity(entity) {
            health_modify_event_writer.send(HealthModifyEvent {
                amount: -trigger.event().damage as i32,
                damaged_entity: colliding_with_hand.entity1
            });
            println!("COLLIDING WITH HAND: {:?}", colliding_with_hand);
        }
    }

    
}


fn update_combat_manager_after_attack(
    mut query: Query<&mut CombatManager, Without<CombatAction>>,
) {
    for mut combat_manager in query.iter_mut() {
        if combat_manager.in_attack {
            combat_manager.in_attack = false;
        }
    }
}

fn setup_attack_colliders(
    mut commands: Commands,
    entity_query: Query<Entity, Added<CombatManager>>, 
    children_query: Query<&Children>,
    name_query: Query<&Name>,
) {

    for entity in entity_query.iter() {
        for descendant in children_query.iter_descendants(entity) {
            // Do something!
            let Ok(descendant_name) = name_query.get(descendant) else {
                println!("Failed to get descendant name");
                continue;
            };

            if descendant_name.to_string() == "Bone.023" || descendant_name.to_string().contains("Finger") {
                commands.entity(descendant).insert((
                    AttackCollider {
                        parent: entity
                    },
                    Collider::sphere(0.5),
                    Sensor
                ));

                println!("makin bone collider");
            }
        }
    }

}

fn npc_attack(
    mut commands: Commands,
    mut attack_mode_query: Query<(Entity, &mut CombatManager), (With<AttackMode>, Without<PlayerCharacter>)>,
) {
    for (entity, mut combat_manager) in attack_mode_query.iter_mut() {
        if !combat_manager.in_attack {
            let mut combat_action: CombatAction;

            let mut rng = rand::rng();

            let random_number: f32 = rng.random_range(0.0..1.0);


            if random_number < 1. {
                println!("In left branch.");
                combat_action = combat_manager.weapon.weapon_stats.light_attack.clone();

            } else {
                println!("In right branch.");
                combat_action = combat_manager.weapon.weapon_stats.heavy_attack.clone();
            }
            combat_action.attack_state = AttackState::Windup;
            combat_action.combat_timer.timer = Timer::from_seconds(combat_action.windup, TimerMode::Once);

            commands.entity(entity).insert(combat_action);

            combat_manager.in_attack = true;

            println!("{:?}",combat_manager);
        }
    }
}