use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone)]
pub struct ButtonConfig {
    pub associated_action: Rc<dyn Fn()>,
}

impl ButtonConfig {
    pub fn new<F>(action: F) -> Self
    where
        F: Fn() + 'static {
        Self {
            associated_action: Rc::new(action),
        }
    }

    pub fn done(self) -> Self {
        self
    }
}

impl Debug for ButtonConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonConfig").finish()
    }
}
