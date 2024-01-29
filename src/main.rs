use bevy::{
    app::{App, PluginGroup, PostStartup, Update},
    asset::{AssetServer, Assets},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Commands, EntityCommands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
    math::Vec2,
    reflect::Reflect,
    render::{mesh::Mesh, render_resource::FilterMode, texture::ImagePlugin, view::Msaa},
    sprite::TextureAtlas,
    utils::HashMap,
    DefaultPlugins,
};
use bevy_entitiles::{
    ldtk::{
        app_ext::LdtkApp,
        events::LdtkEvent,
        json::{field::FieldInstance, level::EntityInstance},
        layer::physics::LdtkPhysicsLayer,
        resources::{LdtkAdditionalLayers, LdtkAssets, LdtkLevelManager, LdtkLoadConfig},
        sprite::LdtkEntityMaterial,
    },
    tilemap::physics::PhysicsTile,
    EntiTilesPlugin,
};
use bevy_entitiles_derive::{LdtkEntity, LdtkEntityTag};
use bevy_xpbd_2d::{
    components::{
        Collider, Friction, LinearVelocity, LockedAxes, Mass, Position, RigidBody, Rotation,
    },
    math::{Scalar, Vector},
    plugins::{
        collision::Collisions, debug::PhysicsDebugConfig, PhysicsDebugPlugin, PhysicsPlugins,
    },
    resources::Gravity,
};
use character_controller::{CharacterControllerBundle, CharacterControllerPlugin};
use helpers::EntiTilesHelpersPlugin;

mod character_controller;
mod helpers;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EntiTilesPlugin,
            EntiTilesHelpersPlugin::default(),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            CharacterControllerPlugin,
        ))
        .add_systems(PostStartup, setup)
        .add_systems(Update, (events))
        .insert_resource(Msaa::Off)
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .insert_resource(PhysicsDebugConfig::all())
        .register_type::<Player>()
        .insert_resource(LdtkLoadConfig {
            file_path: "assets/map.ldtk".to_string(),
            filter_mode: FilterMode::Nearest,
            ignore_unregistered_entities: false,
            ..Default::default()
        })
        .insert_resource(LdtkAdditionalLayers {
            physics_layer: Some(LdtkPhysicsLayer {
                identifier: "PhysicsColliders".to_string(),
                air: 0,
                parent: "Collisions".to_string(),
                tiles: Some(HashMap::from([(
                    1,
                    PhysicsTile {
                        rigid_body: true,
                        friction: Some(0.9),
                    },
                )])),
            }),
            ..Default::default()
        })
        .register_ldtk_entity::<Player>("Player")
        .register_ldtk_entity_tag::<Actor>("actor")
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

// this function will be called when the player entity is GOING TO BE spawned
// which means the entity still has no components
// you can consider this as a "extension"
// you don't need to impl the entire LdtkEntity trait but still able to do something
// that are not supported by generated code
fn player_spawn(
    // the entity commands for the entity
    commands: &mut EntityCommands,
    // all the data from ldtk
    entity_instance: &EntityInstance,
    // the fields of this entity you can access them using their identifiers(ldtk side)
    // generally you don't need to use this, and this fields will be applyed to the entity later
    // with generated code
    _fields: &HashMap<String, FieldInstance>,
    // the asset server
    _asset_server: &AssetServer,
    // the ldtk assets, like sprites and meshes
    _ldtk_assets: &LdtkAssets,
) {
    // this is takes params that are exactly the same as the LdtkEntity trait
    // you can use this to add more fancy stuff to your entity
    // like adding a collider:
    let width = entity_instance.width as f32;
    let height = entity_instance.height as f32;
    let cuboid = Collider::cuboid(width, height);

    commands.insert((
        // RigidBody::Kinematic,
        // LockedAxes::ROTATION_LOCKED,
        CharacterControllerBundle::new(cuboid).with_movement(1250.0, 0.92),
    ));
}

#[derive(Component, Default, LdtkEntity, Reflect)]
#[spawn_sprite]
// this means the entity will not disappear when the level is unloaded
#[global_entity]
#[callback(player_spawn)]
pub struct Player {
    #[ldtk_name = "HP"]
    pub hp: i32,
    // this will be deafult as it not exists in the ldtk file
    #[ldtk_default]
    pub mp: i32,
}

#[derive(Component, LdtkEntityTag)]
pub struct Actor;
