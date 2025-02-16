use std::process::Command;

use bevy::{input::mouse::MouseButtonInput, prelude::*, state::commands};

use crate::{
    asset_loader::AssetLoadingState,
    character_controller::PlayerCharacter
};

pub fn plugin(app: &mut App) {
    app
        .add_systems(Update, (
            setup,
            attack_trigger,
            attack_update
        ).run_if(in_state(AssetLoadingState::Loaded)))
        .add_systems(PostUpdate, (
            update_combat_manager_after_attack
        ).run_if(in_state(AssetLoadingState::Loaded)))
        .add_observer(
            |trigger: Trigger<AttackEvent>,

            | {
                let event = trigger.event();
                println!("Attack event triggered.");
        });
}

#[derive(Event)]
struct AttackEvent;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum AttackType {
    Light,
    Heavy
}

#[derive(Default, Debug, Clone)]
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
    attack_state: AttackState,
    combat_timer: CombatTimer,
    windup: f32,
    attack_time: f32,
    cooldown: f32,
    damage: f32
}

#[derive(Debug)]
pub struct WeaponStats {
    light_attack: CombatAction,
    heavy_attack: CombatAction
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
                damage: 10.0
            },
            heavy_attack: CombatAction {
                attack_type: AttackType::Heavy,
                attack_state: AttackState::Idle,
                combat_timer: CombatTimer::default(),
                windup: 0.4,
                attack_time: 0.2,
                cooldown: 1.0,
                damage: 20.0
            }
        }
    }
}

#[derive(Component, Debug)]
#[require(Transform)]
pub struct Weapon {
    weapon_entity: Option<Entity>,
    weapon_stats: WeaponStats
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
    weapon: Weapon,
    last_attack_cooldown: f32,
    in_attack: bool
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

fn attack_trigger(
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

fn attack_update(
    mut combat_action_query: Query<(Entity, &mut CombatAction)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (entity,  mut combat_action) in combat_action_query.iter_mut() {
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
                commands.trigger(AttackEvent);
                combat_action.combat_timer.timer.tick(time.delta());
                println!("Attack");
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
            }
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