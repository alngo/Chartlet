use super::composite::*;
use super::Drawables;

pub struct Builder {
    width: usize,
    height: usize,
}

impl Builder {
    pub fn new(width: usize, height: usize) -> Builder {
        Builder { width, height }
    }

    pub fn build_grid(&self, min_x: usize, max_x: usize) -> Drawables {
        let mut drawables = Drawables::new();
        for x in min_x..=max_x {
            // Every 20 candles, draw a vertical line...
            // Could be better, todo: get rid of this magic number
            if x % 20 == 0 {
                drawables.push(Box::new(line::Line::new(
                    super::Point(x as f64, 0.0),
                    super::Point(x as f64, self.height as f64),
                )));
            }
        }
        drawables
    }
}
