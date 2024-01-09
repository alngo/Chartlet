#[derive(Default, Copy, Clone, Debug)]
pub struct Frame {
    pub auto_adjust_y: bool,
    pub auto_move_x: bool,
    pub shift: usize,
    pub min_x: usize,
    pub min_y: f64,
    pub max_x: usize,
    pub max_y: f64,
    // ? minimum width
    // ? maximum width
    // ? minimum height
    // ? maximum height
}

impl Frame {
    pub fn new(
        auto_adjust_y: bool,
        auto_move_x: bool,
        shift: usize,
        min_x: usize,
        min_y: f64,
        max_x: usize,
        max_y: f64,
    ) -> Frame {
        Frame {
            auto_adjust_y,
            auto_move_x,
            shift,
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn vertical_zoom_in(&mut self, zoom_in: f64) {
        let temp_min_y = self.min_y + zoom_in;
        let temp_max_y = self.max_y - zoom_in;
        let is_negative_height = temp_min_y >= temp_max_y;

        if is_negative_height {
            return;
        }
        self.min_y = temp_min_y;
        self.max_y = temp_max_y;
    }

    pub fn vertical_zoom_out(&mut self, zoom_out: f64) {
        self.min_y -= zoom_out;
        self.max_y += zoom_out;
    }

    pub fn horizontal_zoom_in(&mut self, zoom_in: usize) {
        let minimum_width = 10;
        let temp_min_x = self.min_x + zoom_in;
        let temp_max_x = self.max_x - zoom_in;
        let is_negative_width = temp_min_x >= temp_max_x;
        let is_less_than_minimum_width = temp_max_x - temp_min_x < minimum_width;

        if is_negative_width || is_less_than_minimum_width {
            return;
        }
        self.min_x = temp_min_x;
        self.max_x = temp_max_x;
    }

    pub fn horizontal_zoom_out(&mut self, zoom_out: usize) {
        if self.min_x != 0 && zoom_out < self.min_x {
            self.min_x -= zoom_out;
        }
        self.max_x += zoom_out;
    }

    pub fn vertical_move(&mut self, y: f64) {
        self.min_y += y;
        self.max_y += y;
    }

    pub fn horizontal_move(&mut self, x: i32) {
        if self.min_x == 0 && x < 0 {
            return;
        }

        if x < 0 {
            if self.min_x < x.abs() as usize {
                self.min_x = 0;
            } else {
                self.min_x -= x.abs() as usize;
            }
            self.max_x -= x.abs() as usize;
        } else {
            self.min_x += x.abs() as usize;
            self.max_x += x.abs() as usize;
        }
    }

    pub fn move_to_most_recent_data(&mut self, data_len: usize) {
        let width = 20;
        if data_len < width {
            self.min_x = 0;
        } else {
            self.min_x = data_len - width;
        }
        self.max_x = data_len;
    }

    pub fn move_to_oldest_data(&mut self) {
        let width = 20;
        self.min_x = 0;
        self.max_x = width;
    }

    pub fn set_shift(&mut self, shift: usize) {
        self.shift = shift;
    }

    pub fn set_auto_move_x(&mut self, auto_move_x: bool) {
        self.auto_move_x = auto_move_x;
    }

    pub fn set_auto_adjust_y(&mut self, auto_adjust_y: bool) {
        self.auto_adjust_y = auto_adjust_y;
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;

    #[test]
    fn test_default_frame() {
        let frame = Frame::default();
        assert_eq!(frame.auto_adjust_y, false);
        assert_eq!(frame.auto_move_x, false);
        assert_eq!(frame.shift, 0);
        assert_eq!(frame.min_x, 0);
        assert_eq!(frame.min_y, 0.0);
        assert_eq!(frame.max_x, 0);
        assert_eq!(frame.max_y, 0.0);
    }

    #[test]
    fn test_new_frame() {
        let frame = Frame::new(true, true, 1, 2, 3.0, 4, 5.0);
        assert_eq!(frame.auto_adjust_y, true);
        assert_eq!(frame.auto_move_x, true);
        assert_eq!(frame.shift, 1);
        assert_eq!(frame.min_x, 2);
        assert_eq!(frame.min_y, 3.0);
        assert_eq!(frame.max_x, 4);
        assert_eq!(frame.max_y, 5.0);
    }

    #[test]
    fn test_set_auto_adjust_y() {
        let mut frame = Frame::default();
        frame.set_auto_adjust_y(true);
        assert_eq!(frame.auto_adjust_y, true);
    }

    #[test]
    fn test_set_auto_move_x() {
        let mut frame = Frame::default();
        frame.set_auto_move_x(true);
        assert_eq!(frame.auto_move_x, true);
    }

    #[test]
    fn test_set_shift() {
        let mut frame = Frame::default();
        frame.set_shift(1);
        assert_eq!(frame.shift, 1);
    }

    #[test]
    fn test_vertical_zoom_in() {
        let mut frame = Frame::new(false, false, 0, 0, 1.0, 0, 2.0);
        frame.vertical_zoom_in(0.2);
        assert_eq!(frame.min_y, 1.2);
        assert_eq!(frame.max_y, 1.8);
    }

    #[test]
    fn test_vertical_zoom_out() {
        let mut frame = Frame::new(false, false, 0, 0, 1.0, 0, 2.0);
        frame.vertical_zoom_out(0.2);
        assert_eq!(frame.min_y, 0.8);
        assert_eq!(frame.max_y, 2.2);
    }

    #[test]
    fn test_horizontal_zoom_in() {
        let mut frame = Frame::new(false, false, 0, 0, 0.0, 20, 0.0);
        frame.horizontal_zoom_in(1);
        assert_eq!(frame.min_x, 1);
        assert_eq!(frame.max_x, 19);
    }

    #[test]
    fn test_horizontal_zoom_out() {
        let mut frame = Frame::new(false, false, 0, 0, 0.0, 20, 0.0);
        frame.horizontal_zoom_out(1);
        assert_eq!(frame.min_x, 0);
        assert_eq!(frame.max_x, 21);

        let mut frame = Frame::new(false, false, 0, 10, 0.0, 20, 0.0);
        frame.horizontal_zoom_out(1);
        assert_eq!(frame.min_x, 9);
        assert_eq!(frame.max_x, 21);
    }

    #[test]
    fn test_vertical_move() {
        let mut frame = Frame::new(false, false, 0, 0, 1.0, 0, 2.0);
        frame.vertical_move(0.2);
        assert_eq!(frame.min_y, 1.2);
        assert_eq!(frame.max_y, 2.2);

        let mut frame = Frame::new(false, false, 0, 0, 1.0, 0, 2.0);
        frame.vertical_move(-0.2);
        assert_eq!(frame.min_y, 0.8);
        assert_eq!(frame.max_y, 1.8);
    }

    #[test]
    fn test_horizontal_move() {
        let mut frame = Frame::new(false, false, 0, 0, 0.0, 20, 0.0);
        frame.horizontal_move(1);
        assert_eq!(frame.min_x, 1);
        assert_eq!(frame.max_x, 21);

        let mut frame = Frame::new(false, false, 0, 10, 0.0, 20, 0.0);
        frame.horizontal_move(-1);
        assert_eq!(frame.min_x, 9);
        assert_eq!(frame.max_x, 19);
    }

    #[test]
    fn test_move_to_most_recent_data() {
        let mut frame = Frame::new(false, false, 0, 0, 0.0, 20, 0.0);
        frame.move_to_most_recent_data(100);
        assert_eq!(frame.min_x, 80);
        assert_eq!(frame.max_x, 100);

        let mut frame = Frame::new(false, false, 0, 0, 0.0, 20, 0.0);
        frame.move_to_most_recent_data(10);
        assert_eq!(frame.min_x, 0);
        assert_eq!(frame.max_x, 10);
    }

    #[test]
    fn test_move_to_oldest_data() {
        let mut frame = Frame::new(false, false, 0, 0, 0.0, 50, 0.0);
        frame.move_to_oldest_data();
        assert_eq!(frame.min_x, 0);
        assert_eq!(frame.max_x, 20);
    }
}
