use amethyst::{
    core::Transform,
    prelude::World,
    core::ecs::{Component, DenseVecStorage},
    renderer::{SpriteRender, SpriteSheet},
    assets::Handle,
    prelude::{Builder, WorldExt}
};
use crate::constants::{
    ARENA_WIDTH,
    ARENA_HEIGHT,
    BALL_RADIUS,
    BALL_VELOCITY_X,
    BALL_VELOCITY_Y
};

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}