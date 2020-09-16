use amethyst::core::ecs::{Entity, World};
use amethyst::assets::Loader;
use amethyst::ui::{TtfFormat, UiTransform, Anchor, UiText, LineMode};
use amethyst::prelude::*;

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

pub struct ScoreText {
    pub left_player_score: Entity,
    pub right_player_score: Entity,
}

pub fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>()
        .load("font/square.ttf", TtfFormat, (), &world.read_resource());
    let left_transform = UiTransform::new(
        "player_1_score".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        -50.0, -50.0, 1.0, 200.0, 50.0
    );
    let right_transform = UiTransform::new(
        "player_2_score".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
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