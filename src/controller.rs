mod data;
mod frame;

use data::{DataController, DataControllerMessage};
use frame::{FrameController, FrameControllerMessage};

use crate::{model::Model, view::View};

pub enum ControllerMessage {
    DataController(DataControllerMessage),
    FrameController(FrameControllerMessage),
}

#[derive(Default)]
pub struct Controller {
    data_controller: DataController,
    frame_controller: FrameController,
}

impl Controller {
    pub fn new(model: &Model, view: &View) -> Controller {
        Controller {
            data_controller: DataController::new(model.data_list.clone()),
            frame_controller: FrameController::new(model.frame.clone()),
        }
    }

    pub fn call(&mut self, message: ControllerMessage) {
        match message {
            ControllerMessage::DataController(message) => {
                self.data_controller.call(message);
            }
            ControllerMessage::FrameController(message) => {
                self.frame_controller.call(message);
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
        let view = View::new();
        let mut controller = Controller::new(&model, &view);
        let message = ControllerMessage::DataController(DataControllerMessage::Push(Data::new(
            0, 1.0, 2.0, 3.0, 4.0, 5.0,
        )));
        controller.call(message);
        assert_eq!(model.data_list.borrow().len(), 1);
    }
}
