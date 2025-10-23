use crate::configs::View::ViewConfig;
use crate::configs::Text::TextConfig;
use crate::configs::Image::ImageConfig;

#[derive(Debug)]
pub enum OpenComposeAST {
    View(ViewConfig, ViewNode),
    // No direct use
    List(ViewConfig, Box<[OpenComposeAST]>),
    Container(ViewConfig, Box<ContainerNode>),
}

#[derive(Debug)]
pub enum ViewNode {
    Image(ViewConfig, ImageConfig),
    Text(ViewConfig, TextConfig),
}

#[derive(Debug)]
pub enum ContainerNode {
    Row(crate::ast::OpenComposeAST),
    Column(crate::ast::OpenComposeAST),
    Box(crate::ast::OpenComposeAST),
    Button(crate::ast::OpenComposeAST)
}
