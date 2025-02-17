use bevy::{asset::AssetIndex, gltf::GltfNode, prelude::*, reflect::Map};
use std::collections::HashMap;

use crate::animation_handler::ResourceHandle;

#[derive(Resource)]
pub struct MyAssets {
    pub asset_paths: Vec<String>,
}

pub fn plugin(app: &mut App) {

    let asset_paths: Vec<String> = vec![
        "dogman.glb".to_string(), 
        "AlienEnemy.glb".to_string()
        ];

    app
        .insert_resource(MyAssets {
            asset_paths,
        })
        .init_state::<AssetLoadingState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (wait_for_gltf_to_load).run_if(in_state(AssetLoadingState::Init2)))
        .add_systems(OnEnter(AssetLoadingState::Loading), parse_gltf);
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AssetLoadingState {
    #[default]
    Init,
    Init2,
    Loading,
    Loaded
}

#[derive(Resource)]
pub struct CharacterHandle {
    pub scene: Handle<Scene>,
    pub animations: Vec<AnimationNodeIndex>,
    pub animation_graph: Handle<AnimationGraph>,
    pub animation_name_reference: HashMap<String, usize>,
}

#[derive(Resource)]
pub struct DogmanGltf {
    pub gltf: Handle<Gltf>,
}

#[derive(Resource)]
pub struct EnemyGltf {
    pub gltf: Handle<Gltf>,
}

#[derive(Resource)]
pub struct EnemyHandle {
    pub scene: Handle<Scene>,
    pub animations: Vec<AnimationNodeIndex>,
    pub animation_graph: Handle<AnimationGraph>,
    pub animation_name_reference: HashMap<String, usize>,
}

pub trait MyGameHandle {
    fn get_scene(&self) -> &Handle<Scene>;
    fn get_animations(&self, index: usize) -> &AnimationNodeIndex;
    fn get_animation_graph(&self) -> &Handle<AnimationGraph>;
    fn get_animation_name_reference(&self, key: &str) -> Option<&usize>;
    fn get_resource_type(&self) -> ResourceHandle;
}

impl MyGameHandle for CharacterHandle {

    fn get_scene(&self) -> &Handle<Scene> {
        &self.scene
    }

    fn get_animations(&self, index: usize) -> &AnimationNodeIndex {
        &self.animations[index]
    }

    fn get_animation_graph(&self) -> &Handle<AnimationGraph> {
        &self.animation_graph
    }

    fn get_animation_name_reference(&self, key: &str) -> Option<&usize> {
        self.animation_name_reference.get(key)
    }

    fn get_resource_type(&self) -> ResourceHandle {
        ResourceHandle::Character
    }
}

impl MyGameHandle for EnemyHandle {

    fn get_scene(&self) -> &Handle<Scene> {
        &self.scene
    }

    fn get_animations(&self, index: usize) -> &AnimationNodeIndex {
        &self.animations[index]
    }

    fn get_animation_graph(&self) -> &Handle<AnimationGraph> {
        &self.animation_graph
    }

    fn get_animation_name_reference(&self, key: &str) -> Option<&usize> {
        self.animation_name_reference.get(key)
    }

    fn get_resource_type(&self) -> ResourceHandle {
        ResourceHandle::Enemy
    }
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut next_asset_loading_state: ResMut<NextState<AssetLoadingState>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {

    let domgan_gltf: Handle<Gltf> = asset_server.load("dogman.glb");

    let alien_gltf: Handle<Gltf> = asset_server.load("AlienEnemy.glb");

    commands.insert_resource(DogmanGltf {
        gltf: domgan_gltf,
    });

    commands.insert_resource(EnemyGltf {
        gltf: alien_gltf,
    });

    next_asset_loading_state.set(AssetLoadingState::Init2);
}

fn wait_for_gltf_to_load(
    gltf_assets: Res<Assets<Gltf>>,
    dogman_gltf: Res<DogmanGltf>,
    alien_gltf: Res<EnemyGltf>,
    mut next_asset_loading_state: ResMut<NextState<AssetLoadingState>>,
) {
    let Some(_dogman_gltf) = gltf_assets.get(&dogman_gltf.gltf) else {
        return;
    };

    let Some(_alien_gltf) = gltf_assets.get(&alien_gltf.gltf) else {
        return;
    };

    next_asset_loading_state.set(AssetLoadingState::Loading);
}

fn parse_gltf(
    gltf_assets: Res<Assets<Gltf>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    dogman_gltf: Res<DogmanGltf>,
    alien_gltf: Res<EnemyGltf>,
    mut commands: Commands,
    //character_handle: Res<CharacterHandle>,
    asset_server: Res<AssetServer>,
    animation_clip_resource: Res<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut next_asset_loading_state: ResMut<NextState<AssetLoadingState>>,
) {


    let dogman_scene: Handle<Scene> = asset_server.load("dogman.glb#Scene0");

    let Some(dogman_gltf) = gltf_assets.get(&dogman_gltf.gltf) else {
        return;
    };

    let mut clips = Vec::new();
    let mut name_mapping = HashMap::new();

    dogman_gltf.named_animations.iter().enumerate().for_each(|(index, animation)| {

        let anim_path = animation.1.path().unwrap();
        clips.push(asset_server.load(anim_path));

        name_mapping.insert(animation.0.to_string(), index);
        
    });
    
    let (graph, node_indices) = AnimationGraph::from_clips(clips);

    node_indices.iter().for_each(|node| {
        println!("{}", node.index());
    });

    let graph_handle = graphs.add(graph);

    commands.insert_resource(CharacterHandle {
        scene: dogman_scene,
        animations: node_indices.clone(),
        animation_graph: graph_handle.clone(),
        animation_name_reference: name_mapping.clone(),
    });

    //Handle alien now.

    
    let alien_enemy_scene: Handle<Scene> = asset_server.load("AlienEnemy.glb#Scene0");

    let Some(alien_gltf) = gltf_assets.get(&alien_gltf.gltf) else {
        return;
    };

    let mut alien_clips = Vec::new();
    let mut alien_name_mapping = HashMap::new();

    alien_gltf.named_animations.iter().enumerate().for_each(|(index, animation)| {

        let anim_path = animation.1.path().unwrap();
        alien_clips.push(asset_server.load(anim_path));

        alien_name_mapping.insert(animation.0.to_string(), index);
        
    });
    
    let (alien_graph, alien_node_indices) = AnimationGraph::from_clips(alien_clips);

    alien_node_indices.iter().for_each(|node| {
        println!("{}", node.index());
    });

    let alien_graph_handle = graphs.add(alien_graph);

    commands.insert_resource(EnemyHandle {
        scene: alien_enemy_scene,
        animations: alien_node_indices,
        animation_graph: alien_graph_handle,
        animation_name_reference: alien_name_mapping,
    });

    
    next_asset_loading_state.set(AssetLoadingState::Loaded);
}