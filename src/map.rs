use avian3d::prelude::{ColliderConstructor, ColliderConstructorHierarchy, RigidBody};
use bevy::prelude::*;

use crate::asset_loader::{AssetLoadingState, MapHandle};

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    map_assets: Res<MapHandle>
) {
    let id = commands.spawn((
        SceneRoot(map_assets.scene.clone()), 
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh)
    )).id();

    //commands.entity(id).insert(ColliderConstructor::ConvexDecompositionFromMesh);
}