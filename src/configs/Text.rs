#[derive(Debug, Clone, Copy)]
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

    pub fn font_size(&mut self, size: usize) -> &mut Self {
        self.font_size = size;
        self
    }

    pub fn done(&self) -> Self {
        self.clone()
    }
}
