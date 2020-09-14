use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::Camera,
};

use crate::components::{initialize_paddles, load_sprite_sheet, Paddle};
use crate::constants;
use amethyst::assets::Handle;
use amethyst::renderer::SpriteSheet;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        initialize_camera(world);
        // when used in System, below Paddle or any resource registration can be removed
        // world.register::<Paddle>();
        // world.register::<Handle<SpriteSheet>>();
        initialize_paddles(world, sprite_sheet_handle);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(constants::ARENA_WIDTH * 0.5, constants::ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(constants::ARENA_WIDTH, constants::ARENA_HEIGHT))
        .with(transform)
        .build();
}
