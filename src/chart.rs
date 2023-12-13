#[derive(PartialEq, Debug)]
enum ChartKind {
    Line,
    Bar,
    Candle,
}

#[derive(PartialEq, Debug)]
pub struct Ohlc {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}

impl Ohlc {
    pub fn new(open: f32, high: f32, low: f32, close: f32) -> Self {
        Ohlc {
            open,
            high,
            low,
            close,
        }
    }
}

pub struct Chart {
    kind: ChartKind,
    pub data: Vec<Ohlc>,
}

impl Chart {
    pub fn new() -> Self {
        Chart {
            kind: ChartKind::Candle,
            data: Vec::new(),
        }
    }

    fn new_with_kind(kind: ChartKind) -> Self {
        Chart {
            kind,
            data: Vec::new(),
        }
    }

    pub fn add(&mut self, ohlc: Ohlc) {
        self.data.push(ohlc);
    }

    fn update(&mut self, index: usize, ohlc: Ohlc) {
        self.data[index] = ohlc;
    }

    pub fn get_data_at(&self, index: usize) -> Option<&Ohlc> {
        self.data.get(index)
    }

    fn set_kind(&mut self, kind: ChartKind) {
        self.kind = kind;
    }

    fn draw(&self) {
        match self.kind {
            ChartKind::Line => {
                todo!();
            }
            ChartKind::Bar => {
                todo!();
            }
            ChartKind::Candle => {
                todo!();
            }
        }
    }
}

#[cfg(test)]
mod chart_tests {
    use super::*;

    #[test]
    fn test_new() {
        let chart = Chart::new();
        assert_eq!(chart.kind, ChartKind::Candle);
        assert_eq!(chart.data.len(), 0);
    }

    #[test]
    fn test_new_with_kind() {
        let chart = Chart::new_with_kind(ChartKind::Line);
        assert_eq!(chart.kind, ChartKind::Line);
        assert_eq!(chart.data.len(), 0);
    }

    #[test]
    fn test_add() {
        let mut chart = Chart::new();
        chart.add(Ohlc::new(10.0, 20.0, 5.0, 15.0));
        assert_eq!(chart.data.len(), 1);
    }

    #[test]
    fn test_update() {
        let mut chart = Chart::new();
        chart.add(Ohlc::new(10.0, 20.0, 5.0, 15.0));
        chart.add(Ohlc::new(15.0, 25.0, 10.0, 20.0));
        chart.update(0, Ohlc::new(20.0, 30.0, 15.0, 25.0));
        assert_eq!(
            chart.get_data_at(0),
            Some(&Ohlc::new(20.0, 30.0, 15.0, 25.0))
        );
    }

    #[test]
    fn test_set_kind() {
        let mut chart = Chart::new();
        chart.set_kind(ChartKind::Line);
        assert_eq!(chart.kind, ChartKind::Line);
    }

    #[test]
    fn test_get_data_at() {
        let mut chart = Chart::new();
        chart.add(Ohlc::new(10.0, 20.0, 5.0, 15.0));
        chart.add(Ohlc::new(15.0, 25.0, 10.0, 20.0));
        assert_eq!(
            chart.get_data_at(0),
            Some(&Ohlc::new(10.0, 20.0, 5.0, 15.0))
        );
        assert_eq!(
            chart.get_data_at(1),
            Some(&Ohlc::new(15.0, 25.0, 10.0, 20.0))
        );
        assert_eq!(chart.get_data_at(2), None);
    }
}
