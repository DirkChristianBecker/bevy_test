use bevy::prelude::*;
use bevy_scrolling_test::prelude::*;


fn setup(
    mut commands: Commands, 
    _assets: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScrollingExamplePlugin))
        .add_systems(Startup, setup)
        .run();
}
