use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{ReadExpect, Join, System, SystemData, World, Write, WriteStorage},
    ui::UiText,
};

use crate::components::Ball;
use crate::state::{ScoreBoard, ScoreText};
use crate::constants::{
    ARENA_WIDTH,
    ARENA_HEIGHT,
    BALL_RADIUS,
    BALL_VELOCITY_X,
    BALL_VELOCITY_Y,
};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (
        mut balls,
        mut locals,
        mut ui_text,
        mut scores,
        score_text
    ): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Right player scored on the left side.
                // We top the score at 999 to avoid text overlap.
                scores.score_right = (scores.score_right + 1)
                    .min(999);

                if let Some(text) = ui_text.get_mut(score_text.right_player_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                // We top the score at 999 to avoid text overlap.
                scores.score_left = (scores.score_left + 1)
                    .min(999);
                if let Some(text) = ui_text.get_mut(score_text.left_player_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0]; // Reverse Direction
                transform.set_translation_x(ARENA_WIDTH / 2.0); // Reset Position
                transform.set_translation_y(ARENA_HEIGHT / 2.0); // Reset Position
                // Print the scoreboard.
                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_left, scores.score_right
                );
            }
        }
    }
}