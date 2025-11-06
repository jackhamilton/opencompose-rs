pub trait RGBAConvertible {
    fn get_rgba(&self) -> RGBAColor;
}

#[derive(Clone, Debug)]
pub struct RGBAColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl RGBAConvertible for RGBAColor {
    fn get_rgba(&self) -> RGBAColor {
        return self.clone()
    }
}
