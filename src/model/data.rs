use super::list::List;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Data {
    pub fn new(timestamp: u64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Data {
        Data {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataList {
    data: Vec<Data>,
}

impl DataList {
    pub fn get_slice(&self, from: usize, to: usize) -> &[Data] {
        let to = if to > self.data.len() {
            self.data.len()
        } else {
            to
        };
        &self.data[from..to]
    }
}

impl List<Data> for DataList {
    fn new() -> DataList {
        DataList { data: Vec::new() }
    }

    fn get(&self, index: usize) -> Option<&Data> {
        self.data.get(index)
    }

    fn push(&mut self, data: Data) {
        self.data.push(data);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&self) -> std::slice::Iter<'_, Data> {
        self.data.iter()
    }
}

#[cfg(test)]
mod data_list_tests {
    use super::*;

    #[test]
    fn test_data_list() {
        let mut data_list = DataList::new();
        data_list.push(Data::new(1, 2.0, 3.0, 4.0, 5.0, 6.0));
        data_list.push(Data::new(2, 3.0, 4.0, 5.0, 6.0, 7.0));
        assert_eq!(data_list.len(), 2);
        assert_eq!(data_list.get(0).unwrap().timestamp, 1);
        assert_eq!(data_list.get(1).unwrap().timestamp, 2);
    }

    #[test]
    fn test_data_slice() {
        let mut data_list = DataList::new();
        data_list.push(Data::new(1, 2.0, 3.0, 4.0, 5.0, 6.0));
        data_list.push(Data::new(2, 3.0, 4.0, 5.0, 6.0, 7.0));
        data_list.push(Data::new(3, 4.0, 5.0, 6.0, 7.0, 8.0));
        let slice = data_list.get_slice(1, 3);
        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0].timestamp, 2);
        assert_eq!(slice[1].timestamp, 3);
    }
}

#[cfg(test)]
mod data_tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = Data::new(1, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(data.timestamp, 1);
        assert_eq!(data.open, 2.0);
        assert_eq!(data.high, 3.0);
        assert_eq!(data.low, 4.0);
        assert_eq!(data.close, 5.0);
        assert_eq!(data.volume, 6.0);
    }
}