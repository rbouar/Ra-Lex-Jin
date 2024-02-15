use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ldtk)
            .add_systems(Update, (despawn_collision_tile, despawn_door_tile))
            .insert_resource(LevelSelection::index(0))
            .insert_resource(LdtkSettings {
                set_clear_color: SetClearColor::FromLevelBackground,
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .register_ldtk_int_cell_for_layer::<CollisionTileBundle>("Collisions", 1)
            .register_ldtk_int_cell_for_layer::<DoorTileBundle>("Collisions", 2);
    }
}

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

fn despawn_collision_tile(
    mut commands: Commands,
    collision_tile_query: Query<Entity, Added<CollisionTile>>,
) {
    for entity in &collision_tile_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_door_tile(mut commands: Commands, door_tile_query: Query<Entity, Added<DoorTile>>) {
    for entity in &door_tile_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk"),
        ..Default::default()
    });
}
