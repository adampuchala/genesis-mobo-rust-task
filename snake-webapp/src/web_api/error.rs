use crate::{
    directions_container::DirectionRegisterError, http::error::NotFoundError,
    model::direction_request::UnknownDirectionError,
};

impl From<UnknownDirectionError> for NotFoundError {
    fn from(_: UnknownDirectionError) -> Self {
        NotFoundError::new("Direction not found")
    }
}

impl From<DirectionRegisterError> for NotFoundError {
    fn from(_: DirectionRegisterError) -> Self {
        NotFoundError::new("Direction register revoked")
    }
}
