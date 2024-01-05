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
        assert_eq!(json, "{\"data_list\":{\"data\":[]}}");
    }
}
