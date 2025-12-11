use crate::configs::view_subtypes::view_padding::{Axis, EdgeInsets, Padding, Side, ViewPadding};
use crate::configs::view_subtypes::view_alignment::{Alignment, ViewAlignment};
use crate::configs::view_subtypes::view_size::*;
use crate::{traits::color::{RGBAColor, RGBAConvertible}};

#[derive(Debug, Clone)]
pub struct ViewConfig {
    pub frame: ViewFrame,
    pub alignment: Option<ViewAlignment>,
    pub background_color: Option<RGBAColor>,
    pub foreground_color: Option<RGBAColor>,
    pub corner_radius: Option<i32>,
    pub padding: ViewPadding
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
            corner_radius: None,
            padding: ViewPadding::new()
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

    pub fn alignment(&mut self, vertical: Alignment, horizontal: Alignment) -> &mut Self {
        let alignment = ViewAlignment {
            vertical,
            horizontal
        };
        self.alignment = Some(alignment);
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

    pub fn padding(&mut self, border: i32) -> &mut Self {
        let mut border_padding = EdgeInsets {
            left_offset: border,
            right_offset: border,
            top_offset: border,
            bottom_offset: border
        }.to_view_padding();
        self.padding.padding.append(&mut border_padding.padding);
        self
    }

    pub fn side_padding(&mut self, side: Side, offset: i32) -> &mut Self {
        match side {
            Side::LEFT => {
                self.padding.padding.append(&mut vec![
                    Padding { side: Side::LEFT, offset: offset },
                ]);
            },
            Side::RIGHT => {
                self.padding.padding.append(&mut vec![
                    Padding { side: Side::RIGHT, offset: offset },
                ]);
            },
            Side::TOP => {
                self.padding.padding.append(&mut vec![
                    Padding { side: Side::TOP, offset: offset },
                ]);
            },
            Side::BOTTOM => {
                self.padding.padding.append(&mut vec![
                    Padding { side: Side::BOTTOM, offset: offset },
                ]);
            },
        }
        self
    }

    pub fn axis_padding(&mut self, axis: Axis, offset: i32) -> &mut Self {
        match axis {
            Axis::HORIZONTAL => {
                self.padding.padding.append(&mut vec![
                    Padding { side: Side::LEFT, offset: offset },
                    Padding { side: Side::RIGHT, offset: offset }
                ]);
            },
            Axis::VERTICAL => {
                self.padding.padding.append(&mut vec![
                    Padding { side: Side::TOP, offset: offset },
                    Padding { side: Side::BOTTOM, offset: offset }
                ]);
            },
        }
        self
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
