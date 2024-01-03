pub mod data;
mod list;

use data::DataList;
use list::List;

pub struct Model {
    data: DataList,
    // IndicatorsList
    // ObjectsList
}

impl Model {
    pub fn new() -> Model {
        Model {
            data: DataList::new(),
        }
    }

    pub fn get_data(&self, index: usize) -> Option<&data::Data> {
        self.data.get(index)
    }

    pub fn push_data(&mut self, data: data::Data) {
        self.data.push(data);
    }

    pub fn len_data(&self) -> usize {
        self.data.len()
    }

    pub fn iter_data(&self) -> std::slice::Iter<'_, data::Data> {
        self.data.iter()
    }
}

#[cfg(test)]
mod store_tests {
    use super::*;

    #[test]
    fn test_data_store() {
        let mut model = Model::new();
        model.push_data(data::Data::new(1, 2.0, 3.0, 4.0, 5.0, 6.0));
        assert_eq!(model.len_data(), 1);
        assert_eq!(model.get_data(0).unwrap().timestamp, 1);
        assert_eq!(model.get_data(0).unwrap().open, 2.0);
        assert_eq!(model.get_data(0).unwrap().high, 3.0);
        assert_eq!(model.get_data(0).unwrap().low, 4.0);
        assert_eq!(model.get_data(0).unwrap().close, 5.0);
        assert_eq!(model.get_data(0).unwrap().volume, 6.0);
    }
}
