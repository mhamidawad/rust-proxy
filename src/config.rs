#[derive(Clone)]
pub struct Config {
    pub backends: Vec<String>,
}

impl Config {
    pub fn new(backends: Vec<String>) -> Self {
        Self { backends }
    }
}
