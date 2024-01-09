use std::{cell::RefCell, rc::Rc};

use crate::model::frame::Frame;

pub enum FrameControllerMessage {
    AutoAdjustY(bool),
    AutoMoveX(bool),
    SetShift(usize),
    VerticalZoomIn(f64),
    VerticalZoomOut(f64),
    HorizontalZoomIn(usize),
    HorizontalZoomOut(usize),
    VerticalMove(f64),
    HorizontalMove(i32),
    MoveToMostRecentData(usize),
    MoveToOldestData,
}

#[derive(Default)]
pub(crate) struct FrameController {
    frame: Rc<RefCell<Frame>>,
}

impl FrameController {
    pub fn new(frame: Rc<RefCell<Frame>>) -> FrameController {
        FrameController { frame }
    }

    pub fn call(&self, message: FrameControllerMessage) {
        match message {
            FrameControllerMessage::AutoAdjustY(auto_adjust_y) => {
                self.set_auto_adjust_y(auto_adjust_y)
            }
            FrameControllerMessage::AutoMoveX(auto_move_x) => self.set_auto_move_x(auto_move_x),
            FrameControllerMessage::SetShift(shift) => self.set_shift(shift),
            FrameControllerMessage::VerticalZoomIn(zoom_in) => self.vertical_zoom_in(zoom_in),
            FrameControllerMessage::VerticalZoomOut(zoom_out) => self.vertical_zoom_out(zoom_out),
            FrameControllerMessage::HorizontalZoomIn(zoom_in) => self.horizontal_zoom_in(zoom_in),
            FrameControllerMessage::HorizontalZoomOut(zoom_out) => {
                self.horizontal_zoom_out(zoom_out)
            }
            FrameControllerMessage::VerticalMove(y) => self.vertical_move(y),
            FrameControllerMessage::HorizontalMove(x) => self.horizontal_move(x),
            FrameControllerMessage::MoveToMostRecentData(data_len) => {
                self.move_to_most_recent_data(data_len)
            }
            FrameControllerMessage::MoveToOldestData => self.move_to_oldest_data(),
        }
    }

    fn set_auto_adjust_y(&self, auto_adjust_y: bool) {
        self.frame.borrow_mut().set_auto_adjust_y(auto_adjust_y);
    }

    fn set_auto_move_x(&self, auto_move_x: bool) {
        self.frame.borrow_mut().set_auto_move_x(auto_move_x);
    }

    fn set_shift(&self, shift: usize) {
        self.frame.borrow_mut().set_shift(shift);
    }

    fn vertical_zoom_in(&self, zoom_in: f64) {
        self.frame.borrow_mut().vertical_zoom_in(zoom_in);
    }

    fn vertical_zoom_out(&self, zoom_out: f64) {
        self.frame.borrow_mut().vertical_zoom_out(zoom_out);
    }

    fn horizontal_zoom_in(&self, zoom_in: usize) {
        self.frame.borrow_mut().horizontal_zoom_in(zoom_in);
    }

    fn horizontal_zoom_out(&self, zoom_out: usize) {
        self.frame.borrow_mut().horizontal_zoom_out(zoom_out);
    }

    fn vertical_move(&self, y: f64) {
        self.frame.borrow_mut().vertical_move(y);
    }

    fn horizontal_move(&self, x: i32) {
        self.frame.borrow_mut().horizontal_move(x);
    }

    fn move_to_most_recent_data(&self, data_len: usize) {
        self.frame.borrow_mut().move_to_most_recent_data(data_len);
    }

    fn move_to_oldest_data(&self) {
        self.frame.borrow_mut().move_to_oldest_data();
    }
}

#[cfg(test)]
mod frame_controller_tests {
    use super::*;

    #[test]
    fn test_call_set_auto_adjust_y() {
        let frame = Rc::new(RefCell::new(Frame::default()));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::AutoAdjustY(true));
        assert_eq!(frame.borrow().auto_adjust_y, true);
    }

    #[test]
    fn test_call_set_auto_move_x() {
        let frame = Rc::new(RefCell::new(Frame::default()));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::AutoMoveX(true));
        assert_eq!(frame.borrow().auto_move_x, true);
    }

    #[test]
    fn test_call_set_shift() {
        let frame = Rc::new(RefCell::new(Frame::default()));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::SetShift(1));
        assert_eq!(frame.borrow().shift, 1);
    }

    #[test]
    fn test_call_vertical_zoom_in() {
        let frame = Rc::new(RefCell::new(Frame::new(false, false, 0, 0, 1.0, 0, 2.0)));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::VerticalZoomIn(0.2));
        assert_eq!(frame.borrow().min_y, 1.2);
        assert_eq!(frame.borrow().max_y, 1.8);
    }

    #[test]
    fn test_call_vertical_zoom_out() {
        let frame = Rc::new(RefCell::new(Frame::new(false, false, 0, 0, 1.0, 0, 2.0)));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::VerticalZoomOut(0.2));
        assert_eq!(frame.borrow().min_y, 0.8);
        assert_eq!(frame.borrow().max_y, 2.2);
    }

    #[test]
    fn test_call_horizontal_zoom_in() {
        let frame = Rc::new(RefCell::new(Frame::new(false, false, 0, 0, 0.0, 20, 0.0)));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::HorizontalZoomIn(1));
        assert_eq!(frame.borrow().min_x, 1);
        assert_eq!(frame.borrow().max_x, 19);
    }

    #[test]
    fn test_call_horizontal_zoom_out() {
        let frame = Rc::new(RefCell::new(Frame::new(false, false, 0, 10, 0.0, 20, 0.0)));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::HorizontalZoomOut(1));
        assert_eq!(frame.borrow().min_x, 9);
        assert_eq!(frame.borrow().max_x, 21);
    }

    #[test]
    fn test_call_vertical_move() {
        let frame = Rc::new(RefCell::new(Frame::new(false, false, 0, 0, 0.0, 0, 5.0)));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::VerticalMove(1.0));
        assert_eq!(frame.borrow().min_y, 1.0);
        assert_eq!(frame.borrow().max_y, 6.0);
    }

    #[test]
    fn test_call_horizontal_move() {
        let frame = Rc::new(RefCell::new(Frame::new(false, false, 0, 0, 0.0, 5, 0.0)));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::HorizontalMove(1));
        assert_eq!(frame.borrow().min_x, 1);
        assert_eq!(frame.borrow().max_x, 6);
    }

    #[test]
    fn test_call_move_to_most_recent_data() {
        let frame = Rc::new(RefCell::new(Frame::new(false, false, 0, 0, 0.0, 20, 0.0)));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::MoveToMostRecentData(40));
        assert_eq!(frame.borrow().min_x, 20);
        assert_eq!(frame.borrow().max_x, 40);
    }

    #[test]
    fn test_call_move_to_oldest_data() {
        let frame = Rc::new(RefCell::new(Frame::new(false, false, 0, 10, 0.0, 40, 0.0)));
        let controller = FrameController::new(frame.clone());
        controller.call(FrameControllerMessage::MoveToOldestData);
        assert_eq!(frame.borrow().min_x, 0);
        assert_eq!(frame.borrow().max_x, 20);
    }
}
