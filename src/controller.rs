mod data;

use data::{DataController, DataControllerMessage};

use crate::model::Model;

pub enum ControllerMessage {
    DataController(DataControllerMessage),
}

struct Controller {
    data_controller: DataController,
}

impl Controller {
    pub fn new(model: &Model) -> Controller {
        Controller {
            data_controller: DataController::new(model.data_list.clone()),
        }
    }
}
