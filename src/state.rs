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
    assets::{Handle, Loader},
    core::{transform::Transform, ecs::Entity},
    prelude::*,
    renderer::Camera,
    renderer::SpriteSheet,
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32
}

pub struct ScoreText {
    pub left_player_score: Entity,
    pub right_player_score: Entity,
}

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
        initialize_scoreboard(world);
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

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>()
        .load("font/square.ttf", TtfFormat, (), &world.read_resource());
    let left_transform = UiTransform::new(
        "Player 1".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        -50.0, -50.0, 1.0, 200.0, 50.0
    );
    let right_transform = UiTransform::new(
        "Player 1".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        50.0, -50.0, 1.0, 200.0, 50.0
    );

    let left_player_score = world
        .create_entity()
        .with(left_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
            LineMode::Single,
            Anchor::Middle
        ))
        .build();

    let right_player_score = world
        .create_entity()
        .with(right_transform)
        .with(UiText::new(
            font,
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
            LineMode::Single,
            Anchor::Middle
        ))
        .build();

    world.insert(ScoreText {left_player_score, right_player_score});
}