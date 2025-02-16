use asset_loader::{AssetLoadingState, CharacterHandle, DogmanGltf};
use bevy::{gltf::GltfNode, prelude::*, scene::ron::de};
use avian3d::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use combat_manager::CombatAction;

mod asset_loader;
mod scene;
mod character_controller;
mod combat_manager;

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
        .add_plugins((asset_loader::plugin, scene::plugin, character_controller::plugin, combat_manager::plugin))
        .add_systems(Update, egui_setup)
        .init_state::<DescribedDogman>()
        .add_systems(Update, (describe_dogman).run_if(in_state(DescribedDogman::False)))
        .run();
}


fn egui_setup(
    mut contexts: EguiContexts,
    combat_manager_query: Query<&CombatAction>
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
    dogman_resource: Option<Res<CharacterHandle>>,
    dogman_gltf_resource: Option<Res<DogmanGltf>>,
    mut dogman_next_state: ResMut<NextState<DescribedDogman>>
) {
    
    // match dogman_resource {
    //     None => println!("Dogman resource not found"),
    //     Some(this_resource) => {
            
    //         let Some(this_scene) = scene.get(&this_resource.scene) else {
    //             println!("Dogman scene not found");
    //             return;
    //         };

    //         match dogman_gltf_resource {
    //             None => println!("Dogman resource not found"),
    //             Some(this_gltf_resource) => {
    //                 let Some(this_gltf) = gltf.get(&this_gltf_resource.gltf) else {
    //                     println!("Dogman gltf not found");
    //                     return;
    //                 };
        
        
        
    //                 println!("Dogman scene: {:#?}", this_scene);
        
                    
    //                 println!("Dogman this_gltf: {:#?}", this_gltf.named_nodes);
        
    //                 dogman_next_state.set(DescribedDogman::True);
    //             }
    //         }

            
    //     }
    // }
}