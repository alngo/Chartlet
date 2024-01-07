use std::{cell::RefCell, rc::Rc};

use crate::model::{
    data::{Data, DataList},
    list::List,
};

pub enum DataControllerMessage {
    Push(Data),
}

#[derive(Default)]
pub(crate) struct DataController {
    data_list: Rc<RefCell<DataList>>,
}

impl DataController {
    pub fn new(data_list: Rc<RefCell<DataList>>) -> DataController {
        DataController { data_list }
    }

    pub fn call(&mut self, method_name: DataControllerMessage) {
        use self::DataControllerMessage::*;
        match method_name {
            Push(data) => self.push_data(data),
        }
    }

    fn push_data(&mut self, data: Data) {
        let mut data_list = self.data_list.borrow_mut();
        data_list.push(data);
    }
}

#[cfg(test)]
mod data_controller_tests {
    use super::*;
    use crate::model::Model;

    #[test]
    fn test_data_controller() {
        let model = Model::new();
        let mut data_controller = DataController::new(model.data_list.clone());
        data_controller.call(DataControllerMessage::Push(Data::new(
            0, 1.0, 2.0, 3.0, 4.0, 5.0,
        )));
        assert_eq!(model.data_list.borrow().len(), 1);
    }
}
