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
}
