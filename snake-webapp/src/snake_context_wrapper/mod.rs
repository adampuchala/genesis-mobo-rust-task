use tokio::sync::RwLock;

use snake_core::SnakeContext;

use crate::model::direction_request::DirectionRequest;

pub struct SnakeWebAppContext {
    snake_ctx: RwLock<SnakeContext>,
}

impl SnakeWebAppContext {
    pub fn new() -> Self {
        Self {
            snake_ctx: RwLock::new(SnakeContext::new())
        }
    }

    pub async fn new_game(&self) {
        self.snake_ctx.write().await.new_game()
    }

    pub async fn current_direction(&self) -> DirectionRequest {
        self.snake_ctx.read().await.current_direction().into()
    }

    pub async fn get_plane_state(&self) -> String {
        self.snake_ctx.read().await.get_plane_string()
    }

    pub async fn update_game_state(&self) {
        self.snake_ctx.write().await.update_position()
    }

    pub async fn update_direction(&self, direction: DirectionRequest) {
        let _ = self.snake_ctx.write().await.change_direction(direction.into());
    }
}
