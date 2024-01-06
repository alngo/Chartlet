#[derive(Default, Clone, Debug)]
pub struct Frame {
    pub auto: bool,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl Frame {
    pub fn new(auto: bool, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Frame {
        Frame {
            auto,
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn set_auto(&mut self, auto: bool) {
        self.auto = auto;
    }

    pub fn set_min_x(&mut self, min_x: f64) {
        self.min_x = min_x;
    }

    pub fn set_min_y(&mut self, min_y: f64) {
        self.min_y = min_y;
    }

    pub fn set_max_x(&mut self, max_x: f64) {
        self.max_x = max_x;
    }

    pub fn set_max_y(&mut self, max_y: f64) {
        self.max_y = max_y;
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;

    #[test]
    fn test_frame() {
        let mut frame = Frame::new(false, 0.0, 0.0, 0.0, 0.0);
        frame.set_auto(true);
        frame.set_min_x(1.0);
        frame.set_min_y(2.0);
        frame.set_max_x(3.0);
        frame.set_max_y(4.0);
        assert_eq!(frame.auto, true);
        assert_eq!(frame.min_x, 1.0);
        assert_eq!(frame.min_y, 2.0);
        assert_eq!(frame.max_x, 3.0);
        assert_eq!(frame.max_y, 4.0);
    }
}
