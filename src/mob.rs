use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<MobBundle>("Mob");
    }
}

#[derive(Default, Component)]
pub struct Mob;

#[derive(Default, Bundle, LdtkEntity)]
pub struct MobBundle {
    pub mob: Mob,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[with(mob_hitbox)]
    pub collider: Collider,
}

fn mob_hitbox(entity_instance: &EntityInstance) -> Collider {
    let width = entity_instance.width as f32;
    let height = entity_instance.height as f32;

    Collider::cuboid(width, height)
}
