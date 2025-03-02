use asset_loader::{AssetLoadingState, CharacterHandle, DogmanGltf, EnemyGltf, EnemyHandle};
use bevy::{gltf::GltfNode, prelude::*, scene::ron::de};
use avian3d::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use character_controller::PlayerCharacter;
use combat_manager::CombatAction;
use health_manager::Health;

mod asset_loader;
mod scene;
mod character_controller;
mod combat_manager;
mod enemy;
mod animation_handler;
mod health_manager;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum DescribedDogman {
    True,
    #[default]
    False
}

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins((asset_loader::plugin, scene::plugin, character_controller::plugin, combat_manager::plugin, enemy::plugin, animation_handler::plugin, health_manager::plugin))
        .add_systems(Update, (egui_setup, get_nodes_in_scene, link_animations))
        .init_state::<DescribedDogman>()
        .add_systems(Update, (describe_dogman).run_if(in_state(DescribedDogman::False)))
        .run();
}


fn egui_setup(
    mut contexts: EguiContexts,
    combat_manager_query: Query<&CombatAction>,
    health_query: Query<&Health>,
) {


    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        combat_manager_query.iter().for_each(|combat_action| {
            ui.label(format!("{:?}", combat_action));
            
        });

    });

}

fn describe_dogman(
    
    scene: Res<Assets<Scene>>,
    gltf: Res<Assets<Gltf>>,
    dogman_resource: Option<Res<EnemyHandle>>,
    dogman_gltf_resource: Option<Res<EnemyGltf>>,
    mut dogman_next_state: ResMut<NextState<DescribedDogman>>
) {
    
    match dogman_resource {
        None => println!("Dogman resource not found"),
        Some(this_resource) => {
            
            let Some(this_scene) = scene.get(&this_resource.scene) else {
                println!("Dogman scene not found");
                return;
            };

            match dogman_gltf_resource {
                None => println!("Dogman resource not found"),
                Some(this_gltf_resource) => {
                    let Some(this_gltf) = gltf.get(&this_gltf_resource.gltf) else {
                        println!("Dogman gltf not found");
                        return;
                    };
        
        
        
                    println!("Dogman scene: {:#?}", this_scene);
        
                    
                    println!("Dogman this_gltf: {:#?}", this_gltf);
        
                    dogman_next_state.set(DescribedDogman::True);
                }
            }

            
        }
    }
}

fn get_nodes_in_scene(
    entity_query: Query<Entity, With<PlayerCharacter>>, 
    children_query: Query<&Children>,
    name_query: Query<&Name>,
) {

    let Ok(entity) = entity_query.get_single() else {
        println!("Failed to get entity");
        return;
    };
    for descendant in children_query.iter_descendants(entity) {
        // Do something!
        let Ok(descendant_name) = name_query.get(descendant) else {
            //println!("Failed to get descendant name");
            continue;
        };
        //println!("{}", descendant_name);
    }

}


#[derive(Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

//Pinkponk's cool code: https://github.com/bevyengine/bevy/discussions/5564#discussioncomment-3333257
fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            println!("Top Entity is : {:?}",    top_entity);
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}