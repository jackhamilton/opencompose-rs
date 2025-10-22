#[derive(Debug, Clone)]
pub struct TextConfig {
    pub text: &'static str,
    pub font_size: usize
}
impl TextConfig {
    pub fn new(text: &'static str) -> Self {
        Self {
            text,
            font_size: 12
        }
    }

    pub fn font_size(&mut self, size: usize) -> Self {
        self.font_size = size;
        self.clone()
    }
}
