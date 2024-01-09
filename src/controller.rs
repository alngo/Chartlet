mod data;
mod frame;

use std::{cell::RefCell, rc::Rc};

use data::{DataController, DataControllerMessage};
use frame::{FrameController, FrameControllerMessage};

use crate::{
    model::{list::List, Model},
    view::View,
};

pub enum ControllerMessage {
    DataController(DataControllerMessage),
    FrameController(FrameControllerMessage),
}

#[derive(Default)]
pub struct Controller {
    model: Rc<Model>,
    view: Rc<RefCell<View>>,
    data_controller: DataController,
    frame_controller: FrameController,
}

impl Controller {
    pub fn new(model: Rc<Model>, view: Rc<RefCell<View>>) -> Controller {
        Controller {
            model: model.clone(),
            view: view.clone(),
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
        self.update();
    }

    fn update(&mut self) {
        self.view.borrow_mut().update(&self.model);
    }
}

#[cfg(test)]
mod controller_tests {
    use super::*;
    use crate::model::{data::Data, list::List, Model};

    #[test]
    fn test_controller() {
        let model = Rc::new(Model::new());
        let view = Rc::new(RefCell::new(View::new()));
        let mut controller = Controller::new(model.clone(), view.clone());
        let message = ControllerMessage::DataController(DataControllerMessage::Push(Data::new(
            0, 1.0, 2.0, 3.0, 4.0, 5.0,
        )));
        controller.call(message);
        assert_eq!(model.data_list.borrow().len(), 1);
    }

    #[test]
    fn test_call_frame_controller() {
        let model = Rc::new(Model::new());
        let view = Rc::new(RefCell::new(View::new()));
        let mut controller = Controller::new(model.clone(), view.clone());

        let message = ControllerMessage::FrameController(FrameControllerMessage::SetShift(1));
        controller.call(message);
        assert_eq!(model.frame.borrow().shift, 1);

        let message = ControllerMessage::FrameController(FrameControllerMessage::AutoMoveX(true));
        controller.call(message);
        assert_eq!(model.frame.borrow().auto_move_x, true);

        let message = ControllerMessage::FrameController(FrameControllerMessage::AutoAdjustY(true));
        controller.call(message);
        assert_eq!(model.frame.borrow().auto_adjust_y, true);
    }
}
