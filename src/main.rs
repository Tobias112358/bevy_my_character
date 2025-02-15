use bevy::prelude::*;
use avian3d::prelude::*;

mod asset_loader;
mod scene;
mod character_controller;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins((asset_loader::plugin, scene::plugin, character_controller::plugin))
        .run();
}