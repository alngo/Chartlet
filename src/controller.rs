mod data;
mod frame;
mod viewport;

use data::{DataController, DataControllerMessage};
use frame::{FrameController, FrameControllerMessage};
use viewport::ViewportController;

pub use viewport::ViewportControllerMessage;

use crate::{model::Model, view::View};

#[derive(Debug)]
pub enum ControllerMessage {
    DataController(DataControllerMessage),
    FrameController(FrameControllerMessage),
    ViewportController(ViewportControllerMessage),
}

#[derive(Default)]
pub struct Controller {
    model: Model,
    view: View,
    frame_controller: FrameController,
    viewport_controller: ViewportController,
    data_controller: DataController,
}

impl Controller {
    pub fn new(model: Model, view: View) -> Controller {
        let data_controller = DataController::default();
        let frame_controller = FrameController::default();
        let viewport_controller = ViewportController::default();
        Controller {
            model,
            view,
            data_controller,
            frame_controller,
            viewport_controller,
        }
    }

    pub fn call(&mut self, message: ControllerMessage) {
        match message {
            ControllerMessage::FrameController(message) => {
                self.frame_controller.call(message, &mut self.model.frame);
            }
            ControllerMessage::ViewportController(message) => {
                self.viewport_controller
                    .call(message, &mut self.model.viewport);
            }
            ControllerMessage::DataController(message) => {
                self.data_controller
                    .call(message, &mut self.model.data_list);
            }
        }
        self.update();
    }

    fn update(&mut self) {
        self.view.update(&self.model);
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
        let mut controller = Controller::new(model, view);
        let message = ControllerMessage::DataController(DataControllerMessage::Push(Data::new(
            0, 1.0, 2.0, 3.0, 4.0, 5.0,
        )));
        controller.call(message);
        assert_eq!(controller.model.data_list.len(), 1);
    }

    #[test]
    fn test_call_frame_controller() {
        let model = Model::new();
        let view = View::new();
        let mut controller = Controller::new(model, view);
        let messages = vec![
            ControllerMessage::FrameController(FrameControllerMessage::SetShift(1)),
            ControllerMessage::FrameController(FrameControllerMessage::AutoMoveX(true)),
            ControllerMessage::FrameController(FrameControllerMessage::AutoAdjustY(true)),
        ];
        for message in messages {
            controller.call(message);
        }
        assert_eq!(controller.model.frame.shift, 1);
        assert_eq!(controller.model.frame.auto_move_x, true);
        assert_eq!(controller.model.frame.auto_adjust_y, true);
    }

    #[test]
    fn test_call_viewport_controller() {
        let model = Model::new();
        let view = View::new();
        let mut controller = Controller::new(model, view);
        let messages = vec![
            ControllerMessage::ViewportController(ViewportControllerMessage::SetWidth(1)),
            ControllerMessage::ViewportController(ViewportControllerMessage::SetHeight(2)),
        ];
        for message in messages {
            controller.call(message);
        }
        assert_eq!(controller.model.viewport.width, 1);
        assert_eq!(controller.model.viewport.height, 2);
    }
}
