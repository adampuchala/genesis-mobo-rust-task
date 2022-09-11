use std::fmt;

use snake_core::Direction;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DirectionRequest {
    Up,
    Down,
    Left,
    Right,
}

impl DirectionRequest {
    pub fn opposite_direction(&self) -> Self {
        match self {
            DirectionRequest::Up => DirectionRequest::Down,
            DirectionRequest::Down => DirectionRequest::Up,
            DirectionRequest::Left => DirectionRequest::Right,
            DirectionRequest::Right => DirectionRequest::Left,
        }
    }
}

impl From<usize> for DirectionRequest {
    fn from(value: usize) -> Self {
        match value {
            0 => DirectionRequest::Up,
            1 => DirectionRequest::Down,
            2 => DirectionRequest::Left,
            3 => DirectionRequest::Right,
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for DirectionRequest {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => DirectionRequest::Up,
            Direction::Down => DirectionRequest::Down,
            Direction::Left => DirectionRequest::Left,
            Direction::Right => DirectionRequest::Right,
        }
    }
}

impl Into<Direction> for DirectionRequest {
    fn into(self) -> snake_core::Direction {
        match self {
            DirectionRequest::Up => snake_core::Direction::Up,
            DirectionRequest::Down => snake_core::Direction::Down,
            DirectionRequest::Left => snake_core::Direction::Left,
            DirectionRequest::Right => snake_core::Direction::Right,
        }
    }
}

#[derive(Debug)]
pub struct UnknownDirectionError;

impl fmt::Display for UnknownDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Direction of provided type does not exist")
    }
}

impl TryFrom<String> for DirectionRequest {
    type Error = UnknownDirectionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "up" => Ok(DirectionRequest::Up),
            "down" => Ok(DirectionRequest::Down),
            "left" => Ok(DirectionRequest::Left),
            "right" => Ok(DirectionRequest::Right),
            _ => Err(UnknownDirectionError),
        }
    }
}
