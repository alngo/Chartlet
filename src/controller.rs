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

    pub fn call(&mut self, message: ControllerMessage) {
        match message {
            ControllerMessage::DataController(message) => {
                self.data_controller.call(message);
            }
        }
    }
}

#[cfg(test)]
mod controller_tests {
    use super::*;
    use crate::model::{data::Data, list::List, Model};

    #[test]
    fn test_controller() {
        let model = Model::new();
        let mut controller = Controller::new(&model);
        let message = ControllerMessage::DataController(DataControllerMessage::Push(Data::new(
            0, 1.0, 2.0, 3.0, 4.0, 5.0,
        )));
        controller.call(message);
        assert_eq!(model.data_list.borrow().len(), 1);
    }
}
