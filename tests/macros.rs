use opencompose_rs_macros::view_builder;

#[test]
fn it_works() {
    let dsl_ast = view_builder! {
        Column {
            Text(text: "Text")
                .font_size(12)
        }
    };
    dbg!(dsl_ast);
}
