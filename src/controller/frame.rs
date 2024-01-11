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
pub(crate) struct FrameController;

impl FrameController {
    pub fn call(&self, message: FrameControllerMessage, frame: &mut Frame) {
        match message {
            FrameControllerMessage::AutoAdjustY(auto_adjust_y) => {
                frame.set_auto_adjust_y(auto_adjust_y)
            }
            FrameControllerMessage::AutoMoveX(auto_move_x) => frame.set_auto_move_x(auto_move_x),
            FrameControllerMessage::SetShift(shift) => frame.set_shift(shift),
            FrameControllerMessage::VerticalZoomIn(zoom_in) => frame.vertical_zoom_in(zoom_in),
            FrameControllerMessage::VerticalZoomOut(zoom_out) => frame.vertical_zoom_out(zoom_out),
            FrameControllerMessage::HorizontalZoomIn(zoom_in) => frame.horizontal_zoom_in(zoom_in),
            FrameControllerMessage::HorizontalZoomOut(zoom_out) => {
                frame.horizontal_zoom_out(zoom_out)
            }
            FrameControllerMessage::VerticalMove(y) => frame.vertical_move(y),
            FrameControllerMessage::HorizontalMove(x) => frame.horizontal_move(x),
            FrameControllerMessage::MoveToMostRecentData(data_len) => {
                frame.move_to_most_recent_data(data_len)
            }
            FrameControllerMessage::MoveToOldestData => frame.move_to_oldest_data(),
        }
    }
}

#[cfg(test)]
mod frame_controller_tests {
    use super::*;

    #[test]
    fn test_call_set_auto_adjust_y() {
        let mut frame = Frame::default();
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::AutoAdjustY(true), &mut frame);
        assert_eq!(frame.auto_adjust_y, true);
    }

    #[test]
    fn test_call_set_auto_move_x() {
        let mut frame = Frame::default();
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::AutoMoveX(true), &mut frame);
        assert_eq!(frame.auto_move_x, true);
    }

    #[test]
    fn test_call_set_shift() {
        let mut frame = Frame::default();
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::SetShift(1), &mut frame);
        assert_eq!(frame.shift, 1);
    }

    #[test]
    fn test_call_vertical_zoom_in() {
        let mut frame = Frame::new(false, false, 0, 0, 1.0, 0, 2.0);
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::VerticalZoomIn(0.2), &mut frame);
        assert_eq!(frame.min_y, 1.2);
        assert_eq!(frame.max_y, 1.8);
    }

    #[test]
    fn test_call_vertical_zoom_out() {
        let mut frame = Frame::new(false, false, 0, 0, 1.0, 0, 2.0);
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::VerticalZoomOut(0.2), &mut frame);
        assert_eq!(frame.min_y, 0.8);
        assert_eq!(frame.max_y, 2.2);
    }

    #[test]
    fn test_call_horizontal_zoom_in() {
        let mut frame = Frame::new(false, false, 0, 0, 0.0, 20, 0.0);
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::HorizontalZoomIn(1), &mut frame);
        assert_eq!(frame.min_x, 1);
        assert_eq!(frame.max_x, 19);
    }

    #[test]
    fn test_call_horizontal_zoom_out() {
        let mut frame = Frame::new(false, false, 0, 10, 0.0, 20, 0.0);
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::HorizontalZoomOut(1), &mut frame);
        assert_eq!(frame.min_x, 9);
        assert_eq!(frame.max_x, 21);
    }

    #[test]
    fn test_call_vertical_move() {
        let mut frame = Frame::new(false, false, 0, 0, 0.0, 0, 5.0);
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::VerticalMove(1.0), &mut frame);
        assert_eq!(frame.min_y, 1.0);
        assert_eq!(frame.max_y, 6.0);
    }

    #[test]
    fn test_call_horizontal_move() {
        let mut frame = Frame::new(false, false, 0, 0, 0.0, 5, 0.0);
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::HorizontalMove(1), &mut frame);
        assert_eq!(frame.min_x, 1);
        assert_eq!(frame.max_x, 6);
    }

    #[test]
    fn test_call_move_to_most_recent_data() {
        let mut frame = Frame::new(false, false, 0, 0, 0.0, 20, 0.0);
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::MoveToMostRecentData(40), &mut frame);
        assert_eq!(frame.min_x, 20);
        assert_eq!(frame.max_x, 40);
    }

    #[test]
    fn test_call_move_to_oldest_data() {
        let mut frame = Frame::new(false, false, 0, 10, 0.0, 40, 0.0);
        let controller = FrameController::default();
        controller.call(FrameControllerMessage::MoveToOldestData, &mut frame);
        assert_eq!(frame.min_x, 0);
        assert_eq!(frame.max_x, 20);
    }
}
