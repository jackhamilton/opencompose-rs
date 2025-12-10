#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewAlignment {
    pub vertical: Alignment,
    pub horizontal: Alignment,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
    Fill
}
