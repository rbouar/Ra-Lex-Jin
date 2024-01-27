
use bevy::{
    app::{App, Plugin, PluginGroup, PostStartup, Startup, Update}, asset::{AssetServer, Assets}, core_pipeline::core_2d::Camera2dBundle, ecs::{
        component::Component, event::EventReader, query::With, schedule::IntoSystemConfigs, system::{Commands, EntityCommands, Query, Res, ResMut, Resource}
    }, input::{keyboard::KeyCode, mouse::{MouseButton, MouseMotion, MouseWheel}, Input}, math::Vec2, reflect::Reflect, render::{camera::OrthographicProjection, color::Color, mesh::Mesh, render_resource::FilterMode, texture::ImagePlugin, view::Msaa}, sprite::TextureAtlas, time::Time, transform::components::Transform, utils::HashMap, DefaultPlugins
};
use bevy_entitiles::tilemap::tile::RawTileAnimation;
use bevy_entitiles::{
    ldtk::{
        app_ext::LdtkApp,
        events::LdtkEvent,
        json::{field::FieldInstance, level::EntityInstance, EntityRef},
        layer::physics::LdtkPhysicsLayer,
        resources::{LdtkAdditionalLayers, LdtkAssets, LdtkLevelManager, LdtkLoadConfig},
        sprite::LdtkEntityMaterial,
    },
    tilemap::physics::PhysicsTile,
    EntiTilesPlugin,
};
use bevy_entitiles_derive::{LdtkEntity, LdtkEntityTag, LdtkEnum};
use bevy_xpbd_2d::{
    components::{Collider, Friction, LinearVelocity, Mass, RigidBody},
    plugins::{debug::PhysicsDebugConfig, PhysicsDebugPlugin, PhysicsPlugins},
    resources::Gravity,
};
use helpers::EntiTilesHelpersPlugin;

mod helpers;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EntiTilesPlugin,
            EntiTilesHelpersPlugin::default(),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(PostStartup, setup)
        .add_systems(Update, (player_control, events,hot_reload))
        .insert_resource(Msaa::Off)
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(PhysicsDebugConfig::all())
        .register_type::<Player>()
        .insert_resource(LdtkLoadConfig {
            file_path: "assets/map.ldtk".to_string(),
            filter_mode: FilterMode::Nearest,
            ignore_unregistered_entities: true,
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

fn hot_reload(
    input: Res<Input<KeyCode>>,
    mut manager: ResMut<LdtkLevelManager>,
    config: Res<LdtkLoadConfig>,
    mut assets: ResMut<LdtkAssets>,
    asset_server: Res<AssetServer>,
    mut atlas_assets: ResMut<Assets<TextureAtlas>>,
    mut entity_material_assets: ResMut<Assets<LdtkEntityMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    if input.just_pressed(KeyCode::Return) {
        manager.reload_json(&config);
        assets.initialize(
            &config,
            &manager,
            &asset_server,
            &mut atlas_assets,
            &mut entity_material_assets,
            &mut mesh_assets,
        );
        println!("Hot reloaded!")
    }
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

fn player_control(mut query: Query<&mut LinearVelocity, With<Player>>, input: Res<Input<KeyCode>>) {
    let Ok(mut player) = query.get_single_mut() else {
        return;
    };
    // wasd is taken up by the camera controller.
    if input.pressed(KeyCode::Left) {
        player.x = -50.;
    }
    if input.pressed(KeyCode::Right) {
        player.x = 50.;
    }
    // I know this is not scientifically correct
    // because the player will be able to jump infinitely
    // but I'm lazy to do the detection :p
    if input.pressed(KeyCode::Up) {
        player.y = 50.;
    }
    if input.pressed(KeyCode::Down) {
        player.y = -50.;
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
    let size = Vec2::new(entity_instance.width as f32, entity_instance.height as f32);
    commands.insert((
        Collider::convex_hull(vec![
            Vec2::new(-0.5, 0.) * size,
            Vec2::new(0.5, 0.) * size,
            Vec2::new(0.5, 1.) * size,
            Vec2::new(-0.5, 1.) * size,
        ])
        .unwrap(),
        RigidBody::Kinematic,
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.5,
            ..Default::default()
        },
        Mass(100.),
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
