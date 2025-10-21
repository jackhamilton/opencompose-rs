pub enum OpenComposeAST {
    View(ViewNode),
    ViewListNode(Box<[ViewNode]>),
    Container(Box<ContainerNode>)
}

pub enum ViewNode {
    Image,
    Text,
    Button
}

pub enum ContainerNode {
    Row(crate::ast::OpenComposeAST),
    Column(crate::ast::OpenComposeAST),
    Box(crate::ast::OpenComposeAST)
}
