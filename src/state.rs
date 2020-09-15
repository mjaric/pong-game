use crate::{components::{
    initialize_paddles,
    load_sprite_sheet,
    initialize_ball
}, constants::{ARENA_WIDTH, ARENA_HEIGHT}, ui};
use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::Camera
};

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        initialize_ball(world, sprite_sheet_handle.clone());
        initialize_paddles(world, sprite_sheet_handle);
        initialize_camera(world);
        ui::initialize_scoreboard(world);
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
