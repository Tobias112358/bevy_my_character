use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app
        .init_state::<AssetLoadingState>()
        .add_systems(Startup, setup);
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Loaded
}

#[derive(Resource)]
pub struct CharacterHandle {
    pub scene: Handle<Scene>,
    pub animations: Vec<AnimationNodeIndex>,
    pub animation_graph: Handle<AnimationGraph>
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut next_asset_loading_state: ResMut<NextState<AssetLoadingState>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {

    let dogman_scene: Handle<Scene> = asset_server.load("dogman.glb#Scene0");
    
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset("dogman.glb")),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset("dogman.glb")),
        asset_server.load(GltfAssetLabel::Animation(2).from_asset("dogman.glb")),
    ]);

    let graph_handle = graphs.add(graph);

    commands.insert_resource(CharacterHandle {
        scene: dogman_scene,
        animations: node_indices,
        animation_graph: graph_handle
    });

    next_asset_loading_state.set(AssetLoadingState::Loaded);
}