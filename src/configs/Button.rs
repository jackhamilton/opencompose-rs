#[derive(Debug, Clone, Copy)]
pub struct ButtonConfig {
    pub action: fn(),
}

impl ButtonConfig {
    pub fn new() -> Self {
        Self {
            action: Self::nullpt
        }
    }

    pub fn action(&mut self, action: fn()) -> &mut Self {
        self.action = action;
        self
    }

    pub fn done(&mut self) -> Self {
        *self
    }

    fn nullpt() {}
}
