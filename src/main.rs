use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera_system)
        .run();
}

fn spawn_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        SpriteBundle {
            transform: Transform::from_xyz(100., 0., 0.),
            texture: asset_server.load("playersprite.png"),
            ..default()
        },
    ));
}
