use opencompose_rs::layout_compositor::Compositor;
use opencompose_rs::configs::View::{ViewConfig, ViewFrame, ViewSize};
use opencompose_rs::view_builder;

#[test]
fn it_works() {
    let mut dsl_ast = view_builder! {
        Column {
            Text(text: "Text")
                .frame(width: 100, height: 100)
            Text(text: "Text")
                .frame(width: 150, height: 120)
        }
    };
    Compositor::layout_ast(&mut dsl_ast);
    dbg!(&dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(220),
            width: ViewSize::Finite(150)
        }),
        _ => panic!("Non expected root type")
    }
}
