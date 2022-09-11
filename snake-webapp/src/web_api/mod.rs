mod error;

use actix_web::web::Data;
use actix_web::{get, post, web::Path, Responder};

use crate::directions_container::DirectionsContainer;
use crate::http::error::NotFoundError;
use crate::model::direction_request::{DirectionRequest, UnknownDirectionError};
use crate::snake_context_wrapper::SnakeWebAppContext;

#[get("/snake")]
async fn show_plane(game: actix_web::web::Data<SnakeWebAppContext>) -> impl Responder {
    let plane = game.get_plane_state().await;
    format!("{}", plane)
}

#[post("/snake/direction/{path}")]
async fn request_direction(
    directions_container: Data<DirectionsContainer>,
    path: Path<String>,
) -> Result<String, NotFoundError> {
    let direction: DirectionRequest =
        path.clone()
            .try_into()
            .map_err(|unknown_direction_err: UnknownDirectionError|  NotFoundError::from(unknown_direction_err))?;

    directions_container
        .register_direction_request(direction)
        .await
        .map_err(|register_error| NotFoundError::from(register_error))?;
    Ok("Success".into())
}
