#[derive(Debug, Clone)]
pub struct ViewPadding {
    pub padding: Vec<Padding>
}

impl ViewPadding {
    pub fn new() -> Self {
        ViewPadding {
            padding: vec![]
        }
    }

    pub fn to_edge_insets(&self) -> EdgeInsets {
        let mut insets = EdgeInsets {
            left_offset: 0,
            right_offset: 0,
            top_offset: 0,
            bottom_offset: 0
        };
        for padding_element in &self.padding {
            match padding_element.side {
                Side::TOP => insets.top_offset += padding_element.offset,
                Side::BOTTOM => insets.bottom_offset += padding_element.offset,
                Side::LEFT => insets.left_offset += padding_element.offset,
                Side::RIGHT => insets.right_offset += padding_element.offset,
            }
        }
        insets
    }
}

#[derive(Debug, Clone)]
pub struct EdgeInsets {
    pub left_offset: i32,
    pub right_offset: i32,
    pub top_offset: i32,
    pub bottom_offset: i32
}

impl EdgeInsets {
    pub fn to_view_padding(&self) -> ViewPadding {
        ViewPadding {
            padding: vec![
                Padding { side: Side:: LEFT, offset: self.left_offset },
                Padding { side: Side:: RIGHT, offset: self.right_offset },
                Padding { side: Side:: TOP, offset: self.top_offset },
                Padding { side: Side:: BOTTOM, offset: self.bottom_offset }
            ]
        }
    }
}

#[derive(Debug, Clone)]
pub struct Padding {
    pub side: Side,
    pub offset: i32
}

#[derive(Debug, Clone)]
pub enum Side {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT
}

#[derive(Debug, Clone)]
pub enum Axis {
    HORIZONTAL,
    VERTICAL
}

