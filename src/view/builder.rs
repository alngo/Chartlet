use super::composite::*;
use super::converter::Converter;
use super::Drawables;
use super::Point;

pub struct Builder {
    converter: Converter,
}

impl Builder {
    pub fn new(converter: Converter) -> Builder {
        Builder { converter }
    }

    pub fn build_grid(&self, min_x: usize, max_x: usize, min_y: f64, max_y: f64) -> Drawables {
        // Could be better, todo: get rid of magic numbers
        // They should be configurable
        let mut drawables = Drawables::new();
        // Every 30 candles, draw a vertical line
        for x in min_x..=max_x {
            if x % 30 == 0 {
                drawables.push(Box::new(line::Line::new(
                    self.converter.frame_to_viewport(Point(x as f64, 0.0)),
                    self.converter.frame_to_viewport(Point(x as f64, max_y)),
                )));
            }
        }

        // Every 50 pips, draw a horizontal line
        drawables
    }
}
