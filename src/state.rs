use crate::{
    components::{
        initialize_paddles,
        load_sprite_sheet,
        initialize_ball,
        Paddle
    },
    constants::{ARENA_WIDTH, ARENA_HEIGHT}
};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::Camera,
    renderer::SpriteSheet
};

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        // when used in System, below Paddle or any resource registration can be removed
        // world.register::<Paddle>();
        // world.register::<Handle<SpriteSheet>>();
        initialize_ball(world, sprite_sheet_handle.clone());
        initialize_paddles(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}
