use opencompose_rs::configs::View::ViewConfig;
use opencompose_rs_macros::view_builder;

#[test]
fn it_works() {
    let dsl_ast = view_builder! {
        Column {
            Text(text: "Text")
                .font_size(12)
                .frame(width: 100, height: 100)
        }
        .frame(width: 100, height: 100)
    };
    dbg!(dsl_ast);
}
