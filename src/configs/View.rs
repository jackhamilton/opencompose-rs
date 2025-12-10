use crate::configs::view_subtypes::view_size::*;
use crate::{configs::view_subtypes::view_alignment::ViewAnchors, traits::color::{RGBAColor, RGBAConvertible}};

#[derive(Debug, Clone, Copy)]
pub struct ViewConfig {
    pub frame: ViewFrame,
    pub alignment: Option<ViewAnchors>,
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
            alignment: None,
            background_color: None,
            foreground_color: None,
            corner_radius: None
        }
    }

    pub fn frame<W, H>(&mut self, width: W, height: H) -> &mut Self
    where
        W: ViewSizeConvertible,
        H: ViewSizeConvertible
    {
        self.frame = ViewFrame {
            width: width.as_view_size(),
            height: height.as_view_size()
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
        *self
    }

    pub fn inherit(&mut self, parent: &ViewConfig) {
        // inheritance is meant for things like theme, frame shouldn't be
        if self.foreground_color.is_none() {
            self.foreground_color = parent.foreground_color;
        }
        if self.background_color.is_none() {
            self.background_color = parent.background_color;
        }
        return
    }
}
