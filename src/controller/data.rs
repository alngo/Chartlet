use crate::model::{
    data::{Data, DataList},
    list::List,
};

#[derive(Debug)]
pub enum DataControllerMessage {
    Push(Data),
}

#[derive(Default)]
pub(crate) struct DataController;

impl DataController {
    pub fn call(&mut self, method_name: DataControllerMessage, data_list: &mut DataList) {
        use self::DataControllerMessage::*;
        match method_name {
            Push(data) => data_list.push(data),
        }
    }
}

#[cfg(test)]
mod data_controller_tests {
    use super::*;
    use crate::model::Model;

    #[test]
    fn test_data_controller() {
        let mut model = Model::new();
        let mut data_controller = DataController::default();
        data_controller.call(
            DataControllerMessage::Push(Data::new(0, 1.0, 2.0, 3.0, 4.0, 5.0)),
            &mut model.data_list,
        );
        assert_eq!(model.data_list.len(), 1);
    }
}
