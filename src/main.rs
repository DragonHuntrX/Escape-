use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera_system)
        .run();
}

fn spawn_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
