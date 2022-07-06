use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorHandling {
    pub error: String,
}

impl ErrorHandling {
    pub fn new(text: String) -> Self {
        Self { error: text }
    }
}
