use bevy::prelude::*;

mod asset_loader;
mod scene;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins((asset_loader::plugin, scene::plugin))
        .run();
}