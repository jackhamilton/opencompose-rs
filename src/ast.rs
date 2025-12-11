use crate::configs::Button::ButtonConfig;
use crate::configs::View::ViewConfig;
use crate::configs::Text::TextConfig;
use crate::configs::Image::ImageConfig;

#[derive(Debug, Clone)]
pub enum OpenComposeAST {
    View(ViewConfig, ViewNode),
    // No direct use
    List(ViewConfig, Box<[OpenComposeAST]>),
    Container(ViewConfig, Box<ContainerNode>),
}

#[derive(Debug, Clone)]
pub enum ViewNode {
    Image(ViewConfig, ImageConfig),
    Text(ViewConfig, TextConfig),
}

#[derive(Debug, Clone)]
pub enum ContainerNode {
    Row(ViewConfig, crate::ast::OpenComposeAST),
    Column(ViewConfig, crate::ast::OpenComposeAST),
    Box(ViewConfig, crate::ast::OpenComposeAST),
    Button(ViewConfig, ButtonConfig, crate::ast::OpenComposeAST)
}
