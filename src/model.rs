pub mod data;
pub mod frame;
pub mod list;
pub mod viewport;

use data::DataList;
use frame::Frame;
use viewport::Viewport;

#[derive(Default, Clone, Debug)]
pub struct Model {
    pub frame: Frame,
    pub viewport: Viewport,
    pub data_list: DataList,
    // IndicatorsList
    // ObjectsList
    // OrderList
}

impl Model {
    pub fn new() -> Model {
        Model {
            frame: Frame::default(),
            viewport: Viewport::default(),
            data_list: DataList::default(),
        }
    }
}

#[cfg(test)]
mod store_tests {
    use super::{data::Data, *};
    use list::List;

    #[test]
    fn test_data_list() {
        let mut model = Model::new();
        let data = Data::new(0, 1.0, 2.0, 3.0, 4.0, 5.0);
        model.data_list.push(data);
        assert_eq!(model.data_list.len(), 1);
    }
}
