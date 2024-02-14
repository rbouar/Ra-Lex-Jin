use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_xpbd_2d::{math::Vector, prelude::*};

use character_controller::CharacterControllerPlugin;
use helpers::EntiTilesHelpersPlugin;

mod character_controller;
mod helpers;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            LdtkPlugin,
            EntiTilesHelpersPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (despawn_collision_tile, despawn_door_tile))
        .insert_resource(Msaa::Off)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..Default::default()
        })
        .register_ldtk_int_cell_for_layer::<CollisionTileBundle>("Collisions", 1)
        .register_ldtk_int_cell_for_layer::<DoorTileBundle>("Collisions", 2)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct CollisionTile;

#[derive(Default, Bundle, LdtkIntCell)]
struct CollisionTileBundle {
    collision_tile: CollisionTile,
}

#[derive(Default, Component)]
struct DoorTile;

#[derive(Default, Bundle, LdtkIntCell)]
struct DoorTileBundle {
    doorr_tile: DoorTile,
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

#[derive(Default, Component)]
struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[worldly]
    worldly: Worldly,
}

fn player_initial_position(
    mut player_query: Query<(&mut Transform, &EntityInstance), Added<Player>>,
) {
    let (mut player_transform, entity_instance) = player_query.single_mut();

    player_transform.translation.x = entity_instance.world_x.unwrap() as f32;
    player_transform.translation.y = entity_instance.world_y.unwrap() as f32;
}
