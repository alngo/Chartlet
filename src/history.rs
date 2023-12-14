pub enum Timeframe {
    M5,
    M10,
    M15,
    M30,
    H1,
    H4,
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

    pub fn get_data_for(&self, window: Window) -> &Vec<&Data> {
        todo!()
    }

    pub fn get_timeline_for(&self, window: Window) -> &Vec<u32>{
        todo!()
    }
}
