use crate::configs::Text::TextConfig;
use crate::configs::Image::ImageConfig;

#[derive(Debug)]
pub enum OpenComposeAST {
    View(ViewNode),
    List(Box<[OpenComposeAST]>),
    Container(Box<ContainerNode>),
}

#[derive(Debug)]
pub enum ViewNode {
    Image(ImageConfig),
    Text(TextConfig),
}

#[derive(Debug)]
pub enum ContainerNode {
    Row(crate::ast::OpenComposeAST),
    Column(crate::ast::OpenComposeAST),
    Box(crate::ast::OpenComposeAST),
    Button(crate::ast::OpenComposeAST)
}

pub trait ViewConfiguration {
    fn argument_config() -> &'static [ArgumentConfiguration];
    fn modifier_keys() -> &'static [&'static str];
}

pub struct ArgumentConfiguration {
    pub key: &'static str,
    pub required: bool
}
