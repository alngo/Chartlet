use std::cmp;

pub enum Timeframe {
    M5,
    M10,
    M15,
    M30,
    H1,
}

// (open, high, low, close, volume)
type Data = (f32, f32, f32, f32, f32);

struct History {
    data: Vec<Data>,
    timeframe: Timeframe,
    start_date: u32,
}
impl History {
    pub fn new(timeframe: Timeframe, start_date: u32) -> History {
        History {
            data: Vec::new(),
            timeframe: timeframe,
            start_date: start_date,
        }
    }

    pub fn insert(&mut self, open: f32, high: f32, low: f32, close: f32, volume: f32) {
        self.data.push((open, high, low, close, volume));
    }

    pub fn get_data(&self, from: u32, to: u32) -> &[Data] {
        if from as usize > self.data.len() {
            return &[];
        }
        let end = cmp::min(self.data.len(), to as usize);
        self.data.get(from as usize..end).unwrap()
    }

    pub fn get_timeline(&self, from: u32, to: u32) -> Vec<u32> {
        const MINUTE: u32 = 60;
        let multiplier = match self.timeframe {
            Timeframe::M5 => 5,
            Timeframe::M10 => 10,
            Timeframe::M15 => 15,
            Timeframe::M30 => 30,
            Timeframe::H1 => 60,
        };
        let mut timeline = Vec::new();
        let mut current_time = self.start_date + from * multiplier * MINUTE;
        for _ in from..to {
            timeline.push(current_time);
            current_time += multiplier * MINUTE;
        }
        timeline
    }
}

#[cfg(test)]
pub mod history_tests {
    use super::*;

    fn create_history() -> History {
        let mut history = History::new(Timeframe::M5, 1000000000);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history.insert(1.0, 2.0, 3.0, 4.0, 5.0);
        history
    }

    #[test]
    fn test_get_data_on_empty_history() {
        let history = History::new(Timeframe::M5, 20210101);
        let data = history.get_data(0, 10);
        assert_eq!(data.len(), 0);

        let data = history.get_data(0, 5);
        assert_eq!(data.len(), 0);

        let data = history.get_data(2, 5);
        assert_eq!(data.len(), 0);

        let data = history.get_data(2, 10);
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_get_data() {
        let history = create_history();
        let data = history.get_data(0, 10);
        assert_eq!(data.len(), 10);

        let data = history.get_data(0, 5);
        assert_eq!(data.len(), 5);

        let data = history.get_data(2, 5);
        assert_eq!(data.len(), 3);

        let data = history.get_data(2, 10);
        assert_eq!(data.len(), 8);
    }

    #[test]
    fn test_get_timeline() {
        let history = create_history();
        let timeline = history.get_timeline(0, 10);
        assert_eq!(timeline.len(), 10);
        assert_eq!(timeline[0], 1000000000);
        assert_eq!(timeline[1], 1000000300);
        assert_eq!(timeline[2], 1000000600);
        assert_eq!(timeline[3], 1000000900);
        assert_eq!(timeline[4], 1000001200);
        assert_eq!(timeline[5], 1000001500);
        assert_eq!(timeline[6], 1000001800);
        assert_eq!(timeline[7], 1000002100);
        assert_eq!(timeline[8], 1000002400);
        assert_eq!(timeline[9], 1000002700);

        let timeline = history.get_timeline(2, 5);
        assert_eq!(timeline.len(), 3);
        assert_eq!(timeline[0], 1000000600);
        assert_eq!(timeline[1], 1000000900);
        assert_eq!(timeline[2], 1000001200);
    }
}
