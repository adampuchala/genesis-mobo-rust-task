use std::fmt;

use actix_web::{HttpResponse, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;

#[derive(Debug, Clone, Copy)]
pub struct NotFoundError(&'static str);

impl NotFoundError {
    pub fn new(message: &'static str) -> Self {
        Self(message)
    }
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for NotFoundError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
}
