use bevy::{ecs::world::DynamicComponentFetch, prelude::*};

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
pub struct CharacterHandle(pub Handle<Scene>);

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut next_asset_loading_state: ResMut<NextState<AssetLoadingState>>
) {

    let dogman: Handle<Scene> = asset_server.load("dogman.glb#Scene0");

    commands.insert_resource(CharacterHandle(dogman));

    next_asset_loading_state.set(AssetLoadingState::Loaded);

}