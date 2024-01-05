use std::{cell::RefCell, rc::Rc};

use crate::model::{
    data::{Data, DataList},
    list::List,
};

pub enum DataControllerMessage {
    Push(Data),
}

pub(crate) struct DataController {
    data_list: DataList,
}

impl DataController {
    pub fn new(data_list: DataList) -> DataController {
        DataController { data_list }
    }

    pub fn call(&mut self, method_name: DataControllerMessage) {
        use self::DataControllerMessage::*;
        match method_name {
            Push(data) => self.push_data(data),
        }
    }

    fn push_data(&mut self, data: Data) {
        self.data_list.push(data);
    }
}
