use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;

use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::plugins::PhysicsDebugPlugin;
use bevy_xpbd_2d::plugins::PhysicsPlugins;
use character_controller::CharacterControllerBundle;
use character_controller::CharacterControllerPlugin;
use helpers::HelpersPlugin;

mod character_controller;
mod helpers;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            LdtkPlugin,
            CharacterControllerPlugin,
            PhysicsPlugins::default(),
            HelpersPlugin::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                despawn_collision_tile,
                despawn_door_tile,
                init_player_camera,
            ),
        )
        .insert_resource(Msaa::Off)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings {
            set_clear_color: SetClearColor::FromLevelBackground,
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
    #[from_entity_instance]
    character_controller: CharacterControllerBundle,
}

fn init_player_camera(mut commands: Commands, player_query: Query<Entity, Added<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        let mut camera_2d = Camera2dBundle::default();
        camera_2d.projection.scale = 0.35;

        commands.entity(player_entity).with_children(|parent| {
            parent.spawn(camera_2d);
        });
    }
}

const PLAYER_ACCELERATION: f32 = 6_000.;
const PLAYER_DAMPING: f32 = 0.9;

impl From<&EntityInstance> for CharacterControllerBundle {
    fn from(entity_instance: &EntityInstance) -> CharacterControllerBundle {
        let width = entity_instance.width as f32;
        let height = entity_instance.height as f32;

        let collider = Collider::cuboid(width, height);

        CharacterControllerBundle::new(collider).with_movement(PLAYER_ACCELERATION, PLAYER_DAMPING)
    }
}
