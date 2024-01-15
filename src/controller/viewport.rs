use crate::model::viewport::Viewport;

#[derive(Debug)]
pub enum ViewportControllerMessage {
    SetWidth(usize),
    SetHeight(usize),
}

#[derive(Default)]
pub(crate) struct ViewportController;

impl ViewportController {
    pub fn call(&mut self, message: ViewportControllerMessage, viewport: &mut Viewport) {
        match message {
            ViewportControllerMessage::SetWidth(width) => {
                viewport.set_width(width);
            }
            ViewportControllerMessage::SetHeight(height) => {
                viewport.set_height(height);
            }
        }
    }
}

#[cfg(test)]
mod viewport_controller_tests {
    use super::*;

    #[test]
    fn test_viewport_controller() {
        let mut viewport = Viewport::default();
        let mut viewport_controller = ViewportController::default();
        viewport_controller.call(ViewportControllerMessage::SetWidth(1), &mut viewport);
        viewport_controller.call(ViewportControllerMessage::SetHeight(2), &mut viewport);
        assert_eq!(viewport.width, 1);
        assert_eq!(viewport.height, 2);
    }
}
