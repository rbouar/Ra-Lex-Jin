use bevy::prelude::*;
use bevy_entitiles::{
    ldtk::{
        events::LdtkEvent,
        resources::{LdtkLevelManager, LdtkLoadConfig},
    },
    EntiTilesPlugin,
};
use bevy_xpbd_2d::{
    plugins::{debug::PhysicsDebugConfig, PhysicsDebugPlugin, PhysicsPlugins},
    resources::Gravity,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EntiTilesPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, events)
        .insert_resource(Msaa::Off)
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(PhysicsDebugConfig::all())
        .insert_resource(LdtkLoadConfig {
            file_path: "assets/map.ldtk".to_string(),
            ignore_unregistered_entities: false,
            ..Default::default()
        })
        .run();
}

fn setup(mut commands: Commands, mut manager: ResMut<LdtkLevelManager>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Load Level
    manager.load(&mut commands, "Level_0".to_string(), None);
}

fn events(mut ldtk_events: EventReader<LdtkEvent>) {
    for event in ldtk_events.read() {
        match event {
            LdtkEvent::LevelLoaded(level) => {
                println!("Level loaded: {}", level.identifier);
            }
            LdtkEvent::LevelUnloaded(level) => {
                println!("Level unloaded: {}", level.identifier);
            }
        }
    }
}
