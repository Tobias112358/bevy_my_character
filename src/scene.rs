use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::asset_loader::{AssetLoadingState, CharacterHandle};

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AssetLoadingState::Loaded), setup)
        .add_systems(Update, move_camera.run_if(in_state(AssetLoadingState::Loaded)));
}

pub fn setup(
    mut commands: Commands,
    dogman: Res<CharacterHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SceneRoot(dogman.0.clone()), 
        Transform::from_xyz(0.0, 4.0, -4.0)
    ));

    commands.spawn(
        (
            Camera3d::default(),
            Transform::from_translation(Vec3::new(0.0, 2.0, 4.0)).with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 2.0)),
        )
    );

    let cube = meshes.add(Cuboid {
        half_size: Vec3::new(0.5,0.5,0.5),
        ..default()
    });

    let cube_mat = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.0, 0.0,1.0),
        ..default()
    });

    

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(cube_mat.clone()),
        Transform::from_translation(Vec3::new(0., -1., 0.)),

    ));

    commands.spawn(
        PointLight::default()
    );

}

pub fn move_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>, 
    mut camera_transform_query: Query<&mut Transform, With<Camera3d>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let mut camera_transform = camera_transform_query.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;

        
        camera_transform.rotate_y(yaw);
        camera_transform.rotate_local_x(pitch);
    }

    let mut frame_transation = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) {
        frame_transation += camera_transform.forward().as_vec3();
    } else if input.pressed(KeyCode::KeyS) {
        frame_transation -= camera_transform.forward().as_vec3();
    }
    if input.pressed(KeyCode::KeyD) {
        frame_transation += camera_transform.right().as_vec3();
    } else if input.pressed(KeyCode::KeyA) {
        frame_transation -= camera_transform.right().as_vec3();
    }

    if input.pressed(KeyCode::KeyE) {
        frame_transation += camera_transform.up().as_vec3();
    } else if input.pressed(KeyCode::KeyQ) {
        frame_transation -= camera_transform.up().as_vec3();
    }

    let speed = 10.0;

    camera_transform.translation += frame_transation *  speed * time.delta_secs();
}