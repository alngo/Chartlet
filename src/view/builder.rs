pub struct BuilderError;

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BuilderError")
    }
}

pub struct Builder {
    width: u64,
    height: u64,
}

impl Builder {
    pub fn new(width: u64, height: u64) -> Builder {
        Builder { width, height }
    }

    pub fn build_grid(&self) -> Result<String, BuilderError> {
        todo!()
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
