use opencompose_rs::layout_compositor::Compositor;
use opencompose_rs::configs::View::ViewConfig;
use opencompose_rs::configs::view_subtypes::view_size::*;
use opencompose_rs::view_builder;

#[test]
fn test_basic_column() {
    let mut dsl_ast = view_builder! {
        Column {
            Text(text: "Text")
                .frame(width: 100, height: 100)
            Text(text: "Text")
                .frame(width: 150, height: 120)
        }
    };
    Compositor::layout_ast(&mut dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(220),
            width: ViewSize::Finite(150)
        }),
        _ => panic!("Non expected root type")
    }
}

#[test]
fn test_basic_row() {
    let mut dsl_ast = view_builder! {
        Row {
            Text(text: "Text")
                .frame(width: 100, height: 100)
            Text(text: "Text")
                .frame(width: 150, height: 120)
        }
    };
    Compositor::layout_ast(&mut dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(120),
            width: ViewSize::Finite(250)
        }),
        _ => panic!("Non expected root type")
    }
}

#[test]
fn test_embedded_row_column() {
    let mut dsl_ast = view_builder! {
        Column {
            Row {
                Text(text: "Text")
                    .frame(width: 100, height: 100)
                Text(text: "Text")
                    .frame(width: 150, height: 120)
            }
            Text(text: "Sample")
                .frame(width: 80, height: 90)
        }
    };
    Compositor::layout_ast(&mut dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(210),
            width: ViewSize::Finite(250)
        }),
        _ => panic!("Non expected root type")
    }
}

#[test]
fn test_embedded_row_column_2() {
    let mut dsl_ast = view_builder! {
        Column {
            Row {
                Text(text: "Text")
                    .frame(width: 100, height: 100)
                Text(text: "Text")
                    .frame(width: 150, height: 120)
            }
            Text(text: "Sample")
                .frame(width: 300, height: 90)
        }
    };
    Compositor::layout_ast(&mut dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(210),
            width: ViewSize::Finite(300)
        }),
        _ => panic!("Non expected root type")
    }
}


#[test]
fn test_embedded_column_row() {
    let mut dsl_ast = view_builder! {
        Row {
            Column {
                Text(text: "Text")
                    .frame(width: 100, height: 100)
                Text(text: "Text")
                    .frame(width: 150, height: 120)
            }
            Text(text: "Sample")
                .frame(width: 130, height: 90)
        }
    };
    Compositor::layout_ast(&mut dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(220),
            width: ViewSize::Finite(280)
        }),
        _ => panic!("Non expected root type")
    }
}

#[test]
fn test_column_forced_size() {
    let mut dsl_ast = view_builder! {
        Column {
            Text(text: "Text")
                .frame(width: 100, height: 100)
            Text(text: "Text")
                .frame(width: 150, height: 120)
        }
        .frame(width: 120, height: 110)
    };
    Compositor::layout_ast(&mut dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(110),
            width: ViewSize::Finite(120)
        }),
        _ => panic!("Non expected root type")
    }
}

#[test]
fn test_row_forced_size() {
    let mut dsl_ast = view_builder! {
        Row {
            Text(text: "Text")
                .frame(width: 100, height: 100)
            Text(text: "Text")
                .frame(width: 150, height: 120)
        }
        .frame(width: 120, height: 110)
    };
    Compositor::layout_ast(&mut dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(110),
            width: ViewSize::Finite(120)
        }),
        _ => panic!("Non expected root type")
    }
}

#[test]
fn test_row_forced_fractional_size() {
    let mut dsl_ast = view_builder! {
        Row {
            Text(text: "Text")
                .frame(width: 100, height: 100)
            Text(text: "Text")
                .frame(width: 150, height: 120)
        }
        .frame(width: 10, height: 15)
    };
    Compositor::layout_ast(&mut dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(15),
            width: ViewSize::Finite(10)
        }),
        _ => panic!("Non expected root type")
    }
}

#[test]
fn test_column_forced_fractional_size() {
    let mut dsl_ast = view_builder! {
        Row {
            Text(text: "Text")
                .frame(width: 100, height: 100)
            Text(text: "Text")
                .frame(width: 150, height: 120)
        }
        .frame(width: 10, height: 15)
    };
    Compositor::layout_ast(&mut dsl_ast);
    match dsl_ast {
        opencompose_rs::ast::OpenComposeAST::Container(view_config, _) =>
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(15),
            width: ViewSize::Finite(10)
        }),
        _ => panic!("Non expected root type")
    }
}
