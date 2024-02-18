use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

use bevy_ecs_ldtk::LdtkPlugin;
use bevy_xpbd_2d::prelude::*;

use dungeon::DungeonPlugin;
use helpers::HelpersPlugin;
use mob::MobPlugin;
use player::PlayerPlugin;

// mod character_controller;
mod character_controller_dynamic;
mod dungeon;
mod helpers;
mod mob;
mod player;

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
                        ..default()
                    }),
                    ..default()
                }),
            // bevy_ecs_ldtk
            LdtkPlugin,
            // bevy_xpbd_2d
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            // FPS counter + bevy egui debug
            HelpersPlugin::default(),
            PlayerPlugin,
            DungeonPlugin,
            MobPlugin,
            // Limit FPS
            bevy_framepace::FramepacePlugin,
        ))
        .add_systems(Startup, maximize_window)
        .insert_resource(Msaa::Off)
        .insert_resource(Gravity::ZERO)
        .run();
}

fn maximize_window(mut window_query: Query<&mut Window>) {
    let mut window = window_query.single_mut();
    window.set_maximized(true);
}
