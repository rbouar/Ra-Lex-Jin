use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::*;

use character_controller::*;
use helpers::HelpersPlugin;

mod character_controller;
mod helpers;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ra Lex Jin".into(),
                        resolution: (1600., 900.).into(),
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        // This will spawn an invisible window
                        // The window will be made visible in the make_visible() system after 3 frames.
                        // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                        // visible: false,
                        ..default()
                    }),
                    ..default()
                }),
            LdtkPlugin,
            CharacterControllerPlugin,
            PhysicsPlugins::default(),
            HelpersPlugin::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, (setup, maximize_window))
        .add_systems(
            Update,
            (
                despawn_collision_tile,
                despawn_door_tile,
                init_player_camera,
                level_selection_follow_player,
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

fn level_selection_follow_player(
    players: Query<&GlobalTransform, With<Player>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    if let Ok(player_transform) = players.get_single() {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("ldtk project should be loaded before player is spawned");

        for (level_iid, level_transform) in levels.iter() {
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("level should exist in only project");

            let level_bounds = Rect {
                min: Vec2::new(
                    level_transform.translation().x,
                    level_transform.translation().y,
                ),
                max: Vec2::new(
                    level_transform.translation().x + level.px_wid as f32,
                    level_transform.translation().y + level.px_hei as f32,
                ),
            };

            if level_bounds.contains(player_transform.translation().truncate()) {
                *level_selection = LevelSelection::Iid(level_iid.clone());
            }
        }
    }
}

fn maximize_window(mut window_query: Query<&mut Window>) {
    let mut window = window_query.single_mut();
    window.set_maximized(true);
}
