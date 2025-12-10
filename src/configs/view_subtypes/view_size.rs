#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewFrame {
    pub width: ViewSize,
    pub height: ViewSize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewSize {
    Infinite,
    Finite(usize)
}

pub trait ViewSizeConvertible {
    fn as_view_size(&self) -> ViewSize;
}

impl ViewSizeConvertible for ViewSize {
    fn as_view_size(&self) -> ViewSize {
        return *self
    }
}

impl ViewSizeConvertible for usize {
    fn as_view_size(&self) -> ViewSize {
        return ViewSize::Finite(*self)
    }
}
