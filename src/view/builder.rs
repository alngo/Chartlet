use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::composite::*;
use super::Drawables;
use super::Point;

#[derive(Default)]
pub struct Builder;

impl Builder {
    pub fn build_grid(&self, min_x: usize, max_x: usize, min_y: f64, max_y: f64) -> Drawables {
        // Could be better, todo: get rid of magic numbers todo: improve big O
        // They should be configurable
        let mut drawables = Drawables::new();
        // Every 30 candles, draw a vertical line
        for x in min_x..=max_x {
            if x % 30 == 0 {
                drawables.push(Box::new(line::Line::new(
                    Point(x as f64, 0.0),
                    Point(x as f64, max_y),
                )));
            }
        }

        // Every 50 pips, draw a horizontal line
        let mut min_y = Decimal::from_f64(min_y).unwrap().round_dp(4);
        let max_y = Decimal::from_f64(max_y).unwrap().round_dp(4);
        let step = Decimal::from_f64(0.0001).unwrap();
        while min_y < max_y {
            if min_y % dec!(0.0050) == dec!(0.0) {
                drawables.push(Box::new(line::Line::new(
                    Point(min_x as f64, min_y.to_string().parse::<f64>().unwrap()),
                    Point(max_x as f64, min_y.to_string().parse::<f64>().unwrap()),
                )));
            }
            min_y += step;
        }
        drawables
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn test_build_grid() {
        let builder = Builder::default();
        let drawables = builder.build_grid(0, 100, 0.0, 1.0);
        // 4 horizontal lines, 200 vertical lines
        assert_eq!(drawables.len(), 204);
    }
}
