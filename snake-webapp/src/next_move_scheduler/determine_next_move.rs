use rand::{distributions::Uniform, prelude::Distribution};

use std::{
    cmp::Ordering,
    collections::HashSet,
    iter::FromIterator
};

use crate::model::direction_request::DirectionRequest;
use crate::directions_container::directions_array::{
    ContainDirections, 
    DirectionsArray
};

pub fn determine_next_move(
    requested_directions: DirectionsArray,
    current_direction: DirectionRequest,
) -> Option<DirectionRequest> {
    let mut requested_directions = requested_directions;
    requested_directions.reset_direction(current_direction.opposite_direction());

    if requested_directions.no_more_directions() {
        return None;
    }

    if let Some(direction) = requested_directions.direction_if_only_one() {
        return Some(direction);
    }

    let mut rng = rand::thread_rng();
    let random_index = Uniform::from(0usize..requested_directions.len() - 1);

    let stage_two = (1..5)
        .map(|_| random_index.sample(&mut rng))
        .map(|idx| requested_directions[idx])
        .collect::<Vec<DirectionRequest>>();

    let mut last_stage = HashSet::<DirectionRequest>::from_iter(stage_two.clone())
        .into_iter()
        .map(|direction_unique| {
            let count = stage_two
                .iter()
                .filter(|&&direction| direction == direction_unique)
                .count();
            (direction_unique, count)
        })
        .collect::<Vec<(DirectionRequest, usize)>>();
    last_stage.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

    let one_of_two = Uniform::from(0..1);
    Some(last_stage[one_of_two.sample(&mut rng)].0)
}

#[cfg(test)]
mod tests {
    use crate::model::direction_request::DirectionRequest;

    use super::determine_next_move;

    #[test]
    fn test_should_return_none_when_all_input_directions_are_false() {
        let directions = vec![];
        let result = determine_next_move(directions, DirectionRequest::Up);
        assert_eq!(None, result);
    }

    #[test]
    fn test_should_return_none_when_only_opposite_direction_is_true() {
        let directions = vec![DirectionRequest::Up];
        let result = determine_next_move(directions, DirectionRequest::Down);
        assert_eq!(None, result);
    }

    #[test]
    fn test_should_return_not_direction_opposite_direction_when_only_this_and_opposite_directions_are_true(
    ) {
        let directions = vec![DirectionRequest::Right, DirectionRequest::Up];
        let result = determine_next_move(directions, DirectionRequest::Down);
        assert_eq!(Some(DirectionRequest::Right), result);
    }
}
