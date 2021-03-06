use amethyst::{
    core::transform::Transform,
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage}
};
use crate::components::Ball;

#[derive(SystemDesc)]
pub struct MoveBallSystem;

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        let level: f32 = (time.absolute_real_time_seconds() / 60.0) as f32 + 1.0;
        let level = level.max(10.0);
        for (ball, local) in (&balls, &mut locals).join() {
            local.prepend_translation_x(ball.velocity[0] * level * time.delta_seconds());
            local.prepend_translation_y(ball.velocity[1] * level * time.delta_seconds());
        }
    }
}