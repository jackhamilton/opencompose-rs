#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewAnchors {
    pub vertical_start: f32,
    pub vertical_end: f32,
    pub horizontal_start: f32,
    pub horizontal_end: f32
}

// These function as presets for ViewAnchor setups
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

pub trait ViewAnchorConvertible {
    fn to_anchors(&self) -> ViewAnchors;
}

impl ViewAnchorConvertible for ViewAlignment {
    fn to_anchors(&self) -> ViewAnchors {
        let (vstart, vend) = match self.vertical {
            Alignment::Start => (0.0, 0.0),
            Alignment::Center => (0.5, 0.5),
            Alignment::End => (1.0, 1.0),
            Alignment::Fill => (0.0, 1.0),
        };
        let (hstart, hend) = match self.horizontal {
            Alignment::Start => (0.0, 0.0),
            Alignment::Center => (0.5, 0.5),
            Alignment::End => (1.0, 1.0),
            Alignment::Fill => (0.0, 1.0),
        };
        ViewAnchors {
            vertical_start: vstart,
            vertical_end: vend,
            horizontal_start: hstart,
            horizontal_end: hend,
        }
    }
}
