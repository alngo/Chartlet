pub mod data;
pub mod frame;
pub mod list;
pub mod viewport;

use std::{cell::RefCell, rc::Rc};

use data::DataList;
use frame::Frame;
use list::List;
use viewport::Viewport;

#[derive(Default, Clone, Debug)]
pub struct Model {
    pub frame: Rc<RefCell<Frame>>,
    pub viewport: Rc<RefCell<Viewport>>,
    pub data_list: Rc<RefCell<DataList>>,
    // IndicatorsList
    // ObjectsList
    // OrderList
}

impl Model {
    pub fn new() -> Model {
        Model {
            frame: Rc::new(RefCell::new(Frame::default())),
            viewport: Rc::new(RefCell::new(Viewport::default())),
            data_list: Rc::new(RefCell::new(DataList::new())),
        }
    }
}

#[cfg(test)]
mod store_tests {
    use super::{data::Data, *};

    #[test]
    fn test_data_list() {
        let model = Model::new();
        let data = Data::new(0, 1.0, 2.0, 3.0, 4.0, 5.0);
        let mut reference = model.data_list.borrow_mut();
        reference.push(data);
        assert_eq!(reference.len(), 1);
    }
}
