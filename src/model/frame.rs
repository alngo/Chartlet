#[derive(Default, Clone, Debug)]
pub struct Frame {
    pub auto: bool,
    pub width: f64,
    pub height: f64,
    pub offset_x: f64,
    pub offset_y: f64,
}

impl Frame {
    pub fn new(auto: bool, width: f64, height: f64, offset_x: f64, offset_y: f64) -> Frame {
        Frame {
            auto,
            width,
            height,
            offset_x,
            offset_y,
        }
    }

    pub fn set_auto(&mut self, auto: bool) {
        self.auto = auto;
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn set_offset_x(&mut self, offset_x: f64) {
        self.offset_x = offset_x;
    }

    pub fn set_offset_y(&mut self, offset_y: f64) {
        self.offset_y = offset_y;
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;

    #[test]
    fn test_frame() {
        let mut frame = Frame::new(false, 0.0, 0.0, 0.0, 0.0);
        frame.set_auto(true);
        frame.set_width(1.0);
        frame.set_height(2.0);
        frame.set_offset_x(3.0);
        frame.set_offset_y(4.0);
        assert_eq!(frame.auto, true);
        assert_eq!(frame.width, 1.0);
        assert_eq!(frame.height, 2.0);
        assert_eq!(frame.offset_x, 3.0);
        assert_eq!(frame.offset_y, 4.0);
    }
}
