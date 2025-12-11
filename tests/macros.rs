use opencompose_rs::configs::Button::ButtonConfig;
use opencompose_rs::configs::view_subtypes::view_alignment::Alignment;
use opencompose_rs::configs::View::ViewConfig;
use opencompose_rs_macros::view_builder;

#[test]
fn basic_macro_compiles() {
    let dsl_ast = view_builder! {
        Column {
            Text(text: "Text")
                .font_size(12)
                .frame(width: 100, height: 100)
            Button(action: {}) {
                Text("Label")
                    .alignment(horizontal: Alignment::Center, vertical: Alignment::Center)
            }
        }
        .frame(width: 100, height: 100)
        .alignment(vertical: Alignment::Center, horizontal: Alignment::Center)
    };
    dbg!(dsl_ast);
}
