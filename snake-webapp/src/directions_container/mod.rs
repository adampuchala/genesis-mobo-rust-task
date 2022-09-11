pub mod directions_array;

use std::sync::Arc;

use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::model::direction_request::DirectionRequest;

use self::directions_array::{
    ContainDirections,
    DirectionsArray
};
pub struct DirectionsContainer {
    directions: Arc<RwLock<DirectionsArray>>,
    direction_sender: Sender<DirectionRequest>,
    join_handle: JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct DirectionRegisterError;

impl DirectionsContainer {
    pub fn init_container() -> Self {
        let directions = Arc::new(RwLock::new(Vec::<DirectionRequest>::new()));
        let (direction_sender, mut direction_recv) = channel::<DirectionRequest>(1024);
        let arr = directions.clone();
        let join_handle = actix_web::rt::spawn(async move {
            loop {
                let value = direction_recv.recv().await;
                if let Some(direction) = value {
                    let directions = &mut arr.write().await;
                    directions.push(direction)
                }
            }
        });
        Self {
            directions,
            direction_sender,
            join_handle,
        }
    }

    pub async fn register_direction_request(
        &self,
        direction: DirectionRequest,
    ) -> Result<(), DirectionRegisterError> {
        self.direction_sender
            .send(direction)
            .await
            .map(|_| ())
            .map_err(|_| DirectionRegisterError)
    }

    pub async fn get_directions(&self) -> DirectionsArray {
        self.directions.read().await.clone()
    }

    pub async fn clear_directions(&self) {
        self.directions.write().await.reset_all_directions()
    }
}

impl Drop for DirectionsContainer {
    fn drop(&mut self) {
        self.join_handle.abort();
    }
}
