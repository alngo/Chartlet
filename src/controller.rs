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
    model: Rc<RefCell<Model>>,
    view: Rc<RefCell<View>>,
    data_controller: DataController,
    frame_controller: FrameController,
}

impl Controller {
    pub fn new(model: Rc<RefCell<Model>>, view: Rc<RefCell<View>>) -> Controller {
        Controller {
            model: model.clone(),
            view: view.clone(),
            data_controller: DataController::new(model.borrow().data_list.clone()),
            frame_controller: FrameController::new(model.borrow().frame.clone()),
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

    fn update(&self) {
        let model = self.model.borrow();
        let data_list = model.data_list.borrow();
        self.frame_controller.update(data_list.get_all());
        // update view
    }
}

#[cfg(test)]
mod controller_tests {
    use super::*;
    use crate::model::{data::Data, list::List, Model};

    #[test]
    fn test_controller() {
        let model = Rc::new(RefCell::new(Model::new()));
        let view = Rc::new(RefCell::new(View::new()));
        let mut controller = Controller::new(model.clone(), view.clone());
        let message = ControllerMessage::DataController(DataControllerMessage::Push(Data::new(
            0, 1.0, 2.0, 3.0, 4.0, 5.0,
        )));
        controller.call(message);
        assert_eq!(model.borrow().data_list.borrow().len(), 1);
    }

    #[test]
    fn test_update() {
        let model = Rc::new(RefCell::new(Model::new()));
        {
            let model = model.borrow_mut();
            let mut data_list = model.data_list.borrow_mut();
            let mut frame = model.frame.borrow_mut();
            frame.set_auto(true);
            frame.set_shift(1);
            frame.set_min_x(0);
            frame.set_min_y(2.0);
            frame.set_max_x(3);
            frame.set_max_y(4.0);
            data_list.push(Data::new(0, 1.0, 4.0, 1.0, 4.0, 5.0));
            data_list.push(Data::new(0, 1.0, 5.0, 2.0, 4.0, 5.0));
            data_list.push(Data::new(0, 1.0, 6.0, 3.0, 4.0, 5.0));
        }
        let view = Rc::new(RefCell::new(View::new()));
        let mut controller = Controller::new(model.clone(), view.clone());
        let message = ControllerMessage::DataController(DataControllerMessage::Push(Data::new(
            0, 1.0, 7.0, 4.0, 4.0, 5.0,
        )));
        controller.call(message);
        assert_eq!(model.borrow().data_list.borrow().len(), 4);
        assert_eq!(model.borrow().frame.borrow().auto, true);
        assert_eq!(model.borrow().frame.borrow().shift, 1);
        assert_eq!(model.borrow().frame.borrow().min_x, 2);
        assert_eq!(model.borrow().frame.borrow().min_y, 3.0);
        assert_eq!(model.borrow().frame.borrow().max_x, 5);
        assert_eq!(model.borrow().frame.borrow().max_y, 7.0);
    }
}
