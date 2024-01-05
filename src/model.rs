pub mod data;
pub mod list;

use data::DataList;
use list::List;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    pub data_list: DataList,
    // IndicatorsList
    // ObjectsList
    // OrderList
    // Frame
}

impl Model {
    pub fn new() -> Model {
        Model {
            data_list: DataList::new(),
        }
    }
}

#[cfg(test)]
mod store_tests {
    use super::{data::Data, *};

    #[test]
    fn test_data_store() {
        let mut model = Model::new();
        let data = Data::new(0, 1.0, 2.0, 3.0, 4.0, 5.0);
        model.data_list.push(data);
        assert_eq!(model.data_list.len(), 1);
    }
}
