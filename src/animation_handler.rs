use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_tnua::prelude::TnuaController;
use std::time::Duration;

use crate::{asset_loader::{AssetLoadingState, CharacterHandle, EnemyHandle, MyGameHandle}, combat_manager::{AttackType, CombatAction}, AnimationEntityLink};

#[derive(Debug, PartialEq, Eq)]
pub enum ResourceHandle {
    Character,
    Enemy
}

#[derive(Component)]
pub struct AnimationHandler {
    pub current_animation: usize,
    pub resource_type: ResourceHandle
}

pub fn plugin(app: &mut App) {
    app
    .add_systems(Update, (
        add_animation_transition_to_player::<CharacterHandle>,
        add_animation_transition_to_player::<EnemyHandle>,
        animation_handler
    ).run_if(in_state(AssetLoadingState::Loaded)));
}


fn add_animation_transition_to_player<T: Resource + MyGameHandle>(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer)>,
    anim_link_query: Query<(&AnimationHandler, &AnimationEntityLink), Added<AnimationEntityLink>>,
    character_handle: Res<T>,
) {
    for (anim_handler, anim_link) in anim_link_query.iter() {

        if anim_handler.resource_type != character_handle.get_resource_type() {
            continue;
        }
        
        let Ok((entity, mut player)) = players.get_mut(anim_link.0) else {
            continue;
        };

        println!("THIS IS IN ANIM TRANSITION: {:?}, {:?}, {:?}", character_handle.get_animation_name_reference("Idle"), anim_link.0, anim_link);


        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, *character_handle.get_animations(0), Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(character_handle.get_animation_graph().clone()))
            .insert(transitions);
    }
}


fn animation_handler(
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    character_handle: Res<CharacterHandle>,
    enemy_handle: Res<EnemyHandle>,
    mut animated_scene_query: Query<(&LinearVelocity, Option<&TnuaController>, Option<&CombatAction>, &mut AnimationHandler, &AnimationEntityLink), With<AnimationHandler>>
) {
    let mut count = 0;


    for (velocity, tnua_context_option, combat_action_option, mut animation_handler, animation_entity_link) in animated_scene_query.iter_mut() {


        //println!("anim link: {:?}", animation_entity_link.0);

        let Ok((mut anim_player, mut transitions)) = animation_players.get_mut(animation_entity_link.0) else {
            return;
        };

        let is_player: bool;

        match tnua_context_option {
            None => is_player = false,
            Some(tnua_context) => {
                is_player = true;
            }
        }

        
        //println!("is player: {:?}", is_player);

        if is_player {
            let tnua_context = tnua_context_option.unwrap();
            let Ok(is_airborne) = tnua_context.is_airborne() else {
                println!("Failed to check if tnua_context is airborne");
                continue;
            };
    
            if combat_action_option.is_some() {
                if combat_action_option.unwrap().attack_type == AttackType::Light {
                    if animation_handler.current_animation != *character_handle.animation_name_reference.get("LightAttack").unwrap() {
                        animation_handler.current_animation = *character_handle.animation_name_reference.get("LightAttack").unwrap();
                        transitions
                        .play(
                            &mut anim_player,
                            character_handle.animations[animation_handler.current_animation],
                            Duration::from_millis(50),
                        )
                        .set_speed(2.);
                    } 
                } else {
                    if animation_handler.current_animation != *character_handle.animation_name_reference.get("HeavyAttack").unwrap() {
                        animation_handler.current_animation = *character_handle.animation_name_reference.get("HeavyAttack").unwrap();
                        transitions
                        .play(
                            &mut anim_player,
                            character_handle.animations[animation_handler.current_animation],
                            Duration::from_millis(50),
                        )
                        .set_speed(2.);
                    }
                }
            } else {
                if is_airborne {
                    if animation_handler.current_animation != *character_handle.animation_name_reference.get("Jumping").unwrap() {
                        animation_handler.current_animation = *character_handle.animation_name_reference.get("Jumping").unwrap();
                        transitions
                        .play(
                            &mut anim_player,
                            character_handle.animations[animation_handler.current_animation],
                            Duration::from_millis(50),
                        )
                        .set_speed(0.8);
                    }
                } else if velocity.length() > 0.25 && !is_airborne {
                    if animation_handler.current_animation != *character_handle.animation_name_reference.get("Running").unwrap() {
                        animation_handler.current_animation = *character_handle.animation_name_reference.get("Running").unwrap();
                        transitions
                        .play(
                            &mut anim_player,
                            character_handle.animations[animation_handler.current_animation],
                            Duration::from_millis(250),
                        )
                        .repeat();
                    }
                } else {
                    if animation_handler.current_animation != *character_handle.animation_name_reference.get("Idle").unwrap() {
                        animation_handler.current_animation = *character_handle.animation_name_reference.get("Idle").unwrap();
                        println!("{}", animation_handler.current_animation);
                    transitions
                        .play(
                            &mut anim_player,
                            character_handle.animations[animation_handler.current_animation],
                            Duration::from_millis(250),
                        )
                        .repeat();
                    }
                }
            }
        } else {
            if animation_handler.current_animation != *enemy_handle.animation_name_reference.get("Idle").unwrap() {
                animation_handler.current_animation = *enemy_handle.animation_name_reference.get("Idle").unwrap();
                println!("{}", animation_handler.current_animation);
            transitions
                .play(
                    &mut anim_player,
                    enemy_handle.animations[animation_handler.current_animation],
                    Duration::from_millis(250),
                )
                .repeat();
            }
        }
    }


    // for (mut anim_player, mut transitions, animation_entity_link) in &mut animation_players {
    //     println!("HERTE: {:?}, {}", transitions.get_main_animation(), count);
    //     count+=1;
    //     println!("anim link: {:?}", animation_entity_link.0);
    //     let Ok((velocity, tnua_context_option, combat_action_option, mut animation_handler)) = player_query.get_mut(animation_entity_link.0) else {
    //         continue;
    //     };

        
    // }
}

