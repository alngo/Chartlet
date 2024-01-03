use super::model::data::Data;
use super::model::Model;
use super::{View, ViewError};

struct JsonView;

impl View<String> for JsonView {
    fn render(&self, model: &Model) -> Result<String, ViewError> {
        let json = serde_json::to_string(&model).map_err(|_| ViewError)?;
        Ok(json)
    }
}

#[cfg(test)]
mod json_view_tests {
    use super::*;
    use crate::model::Model;

    #[test]
    fn test_json_view() {
        let model = Model::new();
        let view = JsonView;

        let json = view.render(&model).unwrap();
        assert_eq!(json, "{\"data\":{\"data\":[]}}");
    }

    #[test]
    fn test_json_view_with_data() {
        let mut model = Model::new();
        model.push_data(Data::new(1, 2.0, 3.0, 4.0, 5.0, 6.0));
        let view = JsonView;

        let json = view.render(&model).unwrap();
        assert_eq!(
            json,
            "{\"data\":{\"data\":[{\"timestamp\":1,\"open\":2.0,\"high\":3.0,\"low\":4.0,\"close\":5.0,\"volume\":6.0}]}}"
        );
    }
}
