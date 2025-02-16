use bevy::{asset::AssetIndex, gltf::GltfNode, prelude::*};
use std::collections::HashMap;


pub fn plugin(app: &mut App) {
    app
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

impl CharacterHandle {
    // pub fn get_scene(&self) -> Scene {
    //     self.scene.
    // }
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut next_asset_loading_state: ResMut<NextState<AssetLoadingState>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {

    let domgan_gltf: Handle<Gltf> = asset_server.load("dogman.glb");

    commands.insert_resource(DogmanGltf {
        gltf: domgan_gltf,
    });

    next_asset_loading_state.set(AssetLoadingState::Init2);
}

fn wait_for_gltf_to_load(
    gltf_assets: Res<Assets<Gltf>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    dogman_gltf: Res<DogmanGltf>,
    mut next_asset_loading_state: ResMut<NextState<AssetLoadingState>>,
) {
    let Some(dogman_gltf) = gltf_assets.get(&dogman_gltf.gltf) else {
        return;
    };

    dogman_gltf.named_animations.iter().for_each(|animation| {
        println!("{:?}", animation.0);

        
    });

    next_asset_loading_state.set(AssetLoadingState::Loading);
}

fn parse_gltf(
    gltf_assets: Res<Assets<Gltf>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    dogman_gltf: Res<DogmanGltf>,
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
        animations: node_indices,
        animation_graph: graph_handle,
        animation_name_reference: name_mapping,
    });

    
    next_asset_loading_state.set(AssetLoadingState::Loaded);
}