use opencompose_rs::ast::{ContainerNode, OpenComposeAST, ViewNode};
use opencompose_rs::layout_compositor::Compositor;
use opencompose_rs::configs::View::ViewConfig;
use opencompose_rs::configs::view_subtypes::view_size::*;
use opencompose_rs::view_builder;

#[test]
fn test_forced_row_size_forces_child_size() {
    let mut dsl_ast = view_builder! {
        Row {
            Text(text: "Text")
                .frame(width: 100, height: ViewSize::Infinite)
            Text(text: "Text")
                .frame(width: 100, height: ViewSize::Infinite)
        }
        .frame(width: ViewSize::Infinite, height: 10)
    };
    Compositor::layout_ast(&mut dsl_ast);
    dbg!(&dsl_ast);
    if let OpenComposeAST::Container(view_config, asts) = dsl_ast {
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(10),
            width: ViewSize::Finite(200)
        });
        if let ContainerNode::Row(row_config, row_ast) = *asts {
            assert_eq!(row_config.frame, ViewFrame {
                height: ViewSize::Finite(10),
                width: ViewSize::Finite(200)
            });
            if let OpenComposeAST::List(list_config, list_ast) = row_ast {
                assert_eq!(list_config.frame, ViewFrame {
                    height: ViewSize::Finite(10),
                    width: ViewSize::Finite(200)
                });
                for child in list_ast {
                    if let OpenComposeAST::View(subview_config, view_node) = child {
                        assert_eq!(subview_config.frame, ViewFrame {
                            height: ViewSize::Finite(10),
                            width: ViewSize::Finite(100)
                        });
                        if let ViewNode::Text(text_config, _text_node) = view_node {
                            assert_eq!(text_config.frame, ViewFrame {
                                height: ViewSize::Finite(10),
                                width: ViewSize::Finite(100)
                            });
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_forced_column_size_forces_child_size() {
    let mut dsl_ast = view_builder! {
        Column {
            Text(text: "Text")
                .frame(width: ViewSize::Infinite, height: 25)
            Text(text: "Text")
                .frame(width: ViewSize::Infinite, height: 25)
        }
        .frame(width: 60, height: ViewSize::Infinite)
    };
    Compositor::layout_ast(&mut dsl_ast);
    dbg!(&dsl_ast);
    if let OpenComposeAST::Container(view_config, asts) = dsl_ast {
        assert_eq!(view_config.frame, ViewFrame {
            height: ViewSize::Finite(50),
            width: ViewSize::Finite(60)
        });
        if let ContainerNode::Row(row_config, row_ast) = *asts {
            assert_eq!(row_config.frame, ViewFrame {
                height: ViewSize::Finite(50),
                width: ViewSize::Finite(60)
            });
            if let OpenComposeAST::List(list_config, list_ast) = row_ast {
                assert_eq!(list_config.frame, ViewFrame {
                    height: ViewSize::Finite(50),
                    width: ViewSize::Finite(60)
                });
                for child in list_ast {
                    if let OpenComposeAST::View(subview_config, view_node) = child {
                        assert_eq!(subview_config.frame, ViewFrame {
                            height: ViewSize::Finite(25),
                            width: ViewSize::Finite(60)
                        });
                        if let ViewNode::Text(text_config, _text_node) = view_node {
                            assert_eq!(text_config.frame, ViewFrame {
                                height: ViewSize::Finite(25),
                                width: ViewSize::Finite(60)
                            });
                        }
                    }
                }
            }
        }
    }
}
