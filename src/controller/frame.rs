use std::{cell::RefCell, rc::Rc};

use crate::model::{frame::Frame, Model};

pub enum FrameControllerMessage {
    SetAuto(bool),
    SetShift(usize),
    SetMinX(usize),
    SetMinY(f64),
    SetMaxX(usize),
    SetMaxY(f64),
}

#[derive(Default)]
pub(crate) struct FrameController {
    frame: Rc<RefCell<Frame>>,
}

impl FrameController {
    pub fn new(frame: Rc<RefCell<Frame>>) -> FrameController {
        FrameController { frame }
    }

    pub fn call(&mut self, message: FrameControllerMessage) {
        match message {
            FrameControllerMessage::SetAuto(auto) => self.set_auto(auto),
            FrameControllerMessage::SetShift(shift) => self.set_shift(shift),
            FrameControllerMessage::SetMinX(min_x) => self.set_width(min_x),
            FrameControllerMessage::SetMinY(min_y) => self.set_height(min_y),
            FrameControllerMessage::SetMaxX(max_x) => self.set_max_x(max_x),
            FrameControllerMessage::SetMaxY(max_y) => self.set_max_y(max_y),
        }
    }

    fn set_auto(&mut self, auto: bool) {
        self.frame.borrow_mut().set_auto(auto);
    }

    fn set_shift(&mut self, shift: usize) {
        self.frame.borrow_mut().set_shift(shift);
    }

    fn set_width(&mut self, min_x: usize) {
        self.frame.borrow_mut().set_min_x(min_x);
    }

    fn set_height(&mut self, min_y: f64) {
        self.frame.borrow_mut().set_min_y(min_y);
    }

    fn set_max_x(&mut self, max_x: usize) {
        self.frame.borrow_mut().set_max_x(max_x);
    }

    fn set_max_y(&mut self, max_y: f64) {
        self.frame.borrow_mut().set_max_y(max_y);
    }

    pub fn update(&self, _model: &Model) {}
}

#[cfg(test)]
mod frame_controller_tests {
    use super::*;
    use crate::model::Model;

    #[test]
    fn test_frame_controller() {
        let model = Model::new();
        let mut controller = FrameController::new(model.frame.clone());
        controller.call(FrameControllerMessage::SetAuto(true));
        controller.call(FrameControllerMessage::SetShift(5));
        controller.call(FrameControllerMessage::SetMinX(1));
        controller.call(FrameControllerMessage::SetMinY(2.0));
        controller.call(FrameControllerMessage::SetMaxX(3));
        controller.call(FrameControllerMessage::SetMaxY(4.0));
        assert_eq!(model.frame.borrow().auto, true);
        assert_eq!(model.frame.borrow().shift, 5);
        assert_eq!(model.frame.borrow().min_x, 1);
        assert_eq!(model.frame.borrow().min_y, 2.0);
        assert_eq!(model.frame.borrow().max_x, 3);
        assert_eq!(model.frame.borrow().max_y, 4.0);
    }
}
