use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use collisions::*;

pub mod collisions;
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CollisionsPlugin)
            .add_systems(Startup, setup_ldtk)
            .insert_resource(LevelSelection::index(0))
            .insert_resource(LdtkSettings {
                set_clear_color: SetClearColor::FromLevelBackground,
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            });
    }
}

fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk"),
        ..Default::default()
    });
}
