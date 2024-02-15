use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use collisions::spawn_wall_collision;

mod collisions;
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ldtk)
            .add_systems(Update, (spawn_wall_collision, hide_collisions_layer))
            .insert_resource(LevelSelection::index(0))
            .insert_resource(LdtkSettings {
                set_clear_color: SetClearColor::FromLevelBackground,
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .register_ldtk_int_cell_for_layer::<CollisionTileBundle>(COLLISIONS_LAYER_ID, 1)
            .register_ldtk_int_cell_for_layer::<DoorTileBundle>(COLLISIONS_LAYER_ID, 2);
    }
}

const COLLISIONS_LAYER_ID: &str = "Collision";

#[derive(Default, Component)]
pub struct CollisionTile;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct CollisionTileBundle {
    pub collision_tile: CollisionTile,
}

#[derive(Default, Component)]
pub struct DoorTile;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct DoorTileBundle {
    pub door_tile: DoorTile,
}

fn hide_collisions_layer(
    mut layer_query: Query<(&mut Visibility, &LayerMetadata), Added<LayerMetadata>>,
) {
    for (mut visibility, layer_metadata) in layer_query.iter_mut() {
        if layer_metadata.identifier == COLLISIONS_LAYER_ID {
            *visibility = Visibility::Hidden;
        }
    }
}

fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk"),
        ..Default::default()
    });
}
