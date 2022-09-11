use crate::model::direction_request::DirectionRequest;

pub type DirectionsArray = Vec::<DirectionRequest>;

pub trait ContainDirections {
    fn no_more_directions(&self) -> bool;
    fn reset_all_directions(&mut self);
    fn reset_direction(&mut self, direction: DirectionRequest);
    fn direction_if_only_one(&self) -> Option<DirectionRequest>;
}

impl ContainDirections for DirectionsArray {
    fn no_more_directions(&self) -> bool {
        self.is_empty()
    }

    fn reset_all_directions(&mut self) {
        self.clear()
    }

    fn reset_direction(&mut self, direction: DirectionRequest) {
        *self = (*self.into_iter()
        .filter(|direction_request| **direction_request != direction)
        .map(|direction|*direction).collect::<Vec<DirectionRequest>>()).to_vec()
    }

    fn direction_if_only_one(&self) -> Option<DirectionRequest> {
        if self.is_empty() {
            None
        } else if self.len() == 1 || self.iter().all(|&direction_req| direction_req == self[0]) {
            Some(self[0])
        } else { None }
    }
}
