mod determine_next_move;

use std::time::Duration;

use actix_web::{rt::spawn, web::Data};

use tokio::{task::JoinHandle, time::interval};

use crate::{
    directions_container::DirectionsContainer,
    snake_context_wrapper::SnakeWebAppContext
};

use self::determine_next_move::determine_next_move;

const SCHEDULER_INTERVAL_MILLISECONDS: u64 = 10000;

pub struct NextMoveScheduler {
    join_handle: JoinHandle<()>,
}

impl NextMoveScheduler {
    pub fn run_scheduler(
        snake_ctx: Data<SnakeWebAppContext>,
        directions_container: Data<DirectionsContainer>,
    ) -> Self {
        let mut interval = interval(Duration::from_millis(SCHEDULER_INTERVAL_MILLISECONDS));
        interval.reset();

        let join_handle = spawn(async move {
            loop {
                interval.tick().await;
                let directions = directions_container.get_directions().await;
                let current_direction = snake_ctx.current_direction().await;
                let next_move = determine_next_move(directions, current_direction);
                if let Some(next_move) = next_move {
                    snake_ctx.update_direction(next_move.into()).await;
                }
                directions_container.clear_directions().await;
                snake_ctx.update_game_state().await;
            }
        });

        Self { join_handle }
    }
}

impl Drop for NextMoveScheduler {
    fn drop(&mut self) {
        self.join_handle.abort()
    }
}
