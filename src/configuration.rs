pub struct Configuration {
    pub width: u32,
    pub height: u32,
    pub shift: u32,
}

impl Configuration {
    pub fn new(width: u32, height: u32) -> Configuration {
        Configuration {
            width,
            height,
            shift: 0,
        }
    }

    pub fn set_shift(&mut self, shift: u32) {
        self.shift = shift;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

#[cfg(test)]
mod configuration_tests {
    use super::*;

    #[test]
    fn test_new() {
        let config = Configuration::new(100, 100);
        assert_eq!(config.width, 100);
        assert_eq!(config.height, 100);
        assert_eq!(config.shift, 0);
        assert_eq!(config.scale, 10);
    }

    #[test]
    fn test_set_shift() {
        let mut config = Configuration::new(100, 100);
        config.set_shift(10);
        assert_eq!(config.shift, 10);
    }

    #[test]
    fn test_set_scale() {
        let mut config = Configuration::new(100, 100);
        config.set_scale(10);
        assert_eq!(config.scale, 10);
    }

    #[test]
    fn test_set_width() {
        let mut config = Configuration::new(100, 100);
        config.set_width(1000);
        assert_eq!(config.width, 1000);
    }

    #[test]
    fn test_set_height() {
        let mut config = Configuration::new(100, 100);
        config.set_height(1000);
        assert_eq!(config.height, 1000);
    }
}
