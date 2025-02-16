use bevy::prelude::*;
use avian3d::prelude::*;

use crate::asset_loader::AssetLoadingState;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        //.add_systems(Update, move_camera.run_if(in_state(AssetLoadingState::Loaded)))
        ;
}

#[derive(Component)]
pub struct Ground;

pub fn setup(
    mut commands: Commands,
    //dogman: Res<CharacterHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let plane = meshes.add(Plane3d {
        half_size: Vec2::new(100.,100.),
        ..default()
    });

    let cube_mat = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.0, 0.0,1.0),
        ..default()
    });

    commands.spawn((
        Ground,
        Mesh3d(plane),
        MeshMaterial3d(cube_mat),
        Transform::from_translation(Vec3::new(0., -1., 0.)),
        RigidBody::Static,
        Collider::cuboid(200.0, 0.1, 200.0),
    ));

    commands.spawn((
        
        PointLight {
            intensity: 1_000_000.0,
            range: 100_000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 12.0, 0.0),
    ));

    commands.spawn(
        DirectionalLight::default()
    );
}