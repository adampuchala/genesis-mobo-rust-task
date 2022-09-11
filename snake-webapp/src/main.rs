mod directions_container;
mod http;
mod model;
mod next_move_scheduler;
mod snake_context_wrapper;
mod web_api;

use actix_settings::{ApplySettings, Settings};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

use crate::{
    directions_container::DirectionsContainer,
    next_move_scheduler::NextMoveScheduler,
    snake_context_wrapper::SnakeWebAppContext,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::parse_toml("./Configuration.toml")
        .expect("Configuration.toml loading error occured");

    let directions_container = Data::new(DirectionsContainer::init_container());
    let snake_ctx = Data::new(SnakeWebAppContext::new());
    snake_ctx.new_game().await;

    let _scheduler =
        NextMoveScheduler::run_scheduler(snake_ctx.clone(), directions_container.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(snake_ctx.clone())
            .app_data(directions_container.clone())
            .service(web_api::show_plane)
            .service(web_api::request_direction)
    })
    .apply_settings(&settings)
    .run()
    .await
}
