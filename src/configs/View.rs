use crate::traits::color::{RGBAColor, RGBAConvertible};

#[derive(Debug, Clone, Copy)]
pub struct ViewConfig {
    pub frame: ViewFrame,
    pub background_color: Option<RGBAColor>,
    pub foreground_color: Option<RGBAColor>,
    pub corner_radius: Option<i32>,
}

impl ViewConfig {
    pub fn new() -> Self {
        Self {
            frame: ViewFrame {
                width: ViewSize::Infinite,
                height: ViewSize::Infinite
            },
            background_color: None,
            foreground_color: None,
            corner_radius: None
        }
    }

    pub fn frame(&mut self, width: usize, height: usize) -> &mut Self {
        self.frame = ViewFrame {
            width: ViewSize::Finite(width),
            height: ViewSize::Finite(height)
        };
        self
    }

    pub fn background_color(&mut self, color: &dyn RGBAConvertible) -> &mut Self {
        self.background_color = Some(color.get_rgba());
        self
    }

    pub fn foreground_color(&mut self, color: &dyn RGBAConvertible) -> &mut Self {
        self.foreground_color = Some(color.get_rgba());
        self
    }

    pub fn corner_radius(&mut self, radius: &i32) -> &mut Self {
        self.corner_radius = Some(radius.clone());
        self
    }

    pub fn done(&mut self) -> Self {
        self.clone()
    }

    pub fn inherit(&mut self, _parent: &ViewConfig) {
        // inheritance is meant for things like theme, frame shouldn't be
        return
    }
}

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
