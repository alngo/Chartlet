pub mod data;
pub mod list;

use std::{cell::RefCell, rc::Rc};

use data::DataList;
use list::List;

#[derive(Default, Clone, Debug)]
pub struct Model {
    pub data_list: Rc<RefCell<DataList>>,
    // IndicatorsList
    // ObjectsList
    // OrderList
    // Frame
}

impl Model {
    pub fn new() -> Model {
        Model {
            data_list: Rc::new(RefCell::new(DataList::new())),
        }
    }
}

#[cfg(test)]
mod store_tests {
    use super::{data::Data, *};

    #[test]
    fn test_data_store() {
        let model = Model::new();
        let data = Data::new(0, 1.0, 2.0, 3.0, 4.0, 5.0);
        let mut reference = model.data_list.borrow_mut();
        reference.push(data);
        assert_eq!(reference.len(), 1);
    }
}
