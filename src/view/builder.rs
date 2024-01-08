#[derive(Default)]
pub struct Builder {
    width: usize,
    height: usize,
}

impl Builder {
    pub fn new(width: usize, height: usize) -> Builder {
        Builder { width, height }
    }

    pub fn build_grid(&self, data: &[Data], min_y: f64, max_y: f64) -> String {
        // x Every hours
        // y Every round number 0.0050
    }

    // fn build_label(&self) -> Result<String, BuilderError> {
    //     todo!()
    // }

    // fn build_data(&self) -> Result<String, BuilderError> {
    //     todo!()
    // }

    // fn build_indicators(&self) -> Result<String, BuilderError> {
    //     todo!()
    // }

    // fn build_objects(&self) -> Result<String, BuilderError> {
    //     todo!()
    // }

    // fn build_orders(&self) -> Result<String, BuilderError> {
    //     todo!()
    // }
}
