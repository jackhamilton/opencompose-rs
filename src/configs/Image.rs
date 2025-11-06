#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub url: String,
}
impl ImageConfig {
    fn new(url: String) -> Self {
        Self {
            url
        }
    }

    pub fn done(&mut self) -> Self {
        self.clone()
    }
}
