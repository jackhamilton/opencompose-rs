#[derive(Debug, Clone)]
pub struct ViewConfig {
    pub frame: ViewFrame,
}
impl ViewConfig {
    pub fn new() -> Self {
        Self {
            frame: ViewFrame {
                width: ViewSize::Infinite,
                height: ViewSize::Infinite
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewFrame {
    pub width: ViewSize,
    pub height: ViewSize
}

#[derive(Debug, Clone)]
pub enum ViewSize {
    Infinite,
    Finite(usize)
}
