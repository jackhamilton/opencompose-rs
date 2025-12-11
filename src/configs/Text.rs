use crate::configs::view_subtypes::view_alignment::Alignment;

#[derive(Debug, Clone)]
pub struct TextConfig {
    pub text: &'static str,
    pub font_size: usize,
    pub horizontal_text_alignment: Alignment,
    pub vertical_text_alignment: Alignment
}
impl TextConfig {
    pub fn new(text: &'static str) -> Self {
        Self {
            text,
            font_size: 12,
            horizontal_text_alignment: Alignment::Start,
            vertical_text_alignment: Alignment::Start
        }
    }

    pub fn font_size(&mut self, size: usize) -> &mut Self {
        self.font_size = size;
        self
    }

    pub fn horizontal_text_alignment(&mut self, alignment: Alignment) -> &mut Self {
        self.horizontal_text_alignment = alignment;
        self
    }

    pub fn vertical_text_alignment(&mut self, alignment: Alignment) -> &mut Self {
        self.vertical_text_alignment = alignment;
        self
    }

    pub fn done(&self) -> Self {
        self.clone()
    }
}
