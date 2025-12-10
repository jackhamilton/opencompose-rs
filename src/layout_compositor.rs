use crate::ast::ContainerNode;
use crate::{ast::OpenComposeAST, configs::View::{ViewFrame, ViewSize}};

// Adds view frames
pub struct Compositor {}

impl Compositor {
    pub fn layout_ast(ast: &mut OpenComposeAST) {
        let root_frame = Compositor::layout_ast_recurse(ast, MergeStyle::Parent, MergeStyle::Parent);
        Compositor::modify_ast_frame(ast, root_frame);
    }

    fn layout_ast_recurse(ast: &mut OpenComposeAST, horizontal_merge_style: MergeStyle, vertical_merge_style: MergeStyle) -> ViewFrame {
        match ast {
            OpenComposeAST::View(view_config, view) => {
                let view_frame = match view {
                    crate::ast::ViewNode::Image(view_config, _image_config) => view_config.frame,
                    crate::ast::ViewNode::Text(view_config, _text_config) => view_config.frame,
                };
                // test print: println!("Reconciling view with frame {:?} with subview with frame {:?}", view_config.frame, view_frame);
                let reconciled_frame = Compositor::reconcile_frames(view_config.frame, view_frame, MergeStyle::Smallest, MergeStyle::Smallest);
                return reconciled_frame
            },
            OpenComposeAST::List(list_config, child_asts) => {
                // test print: println!("Parsing list");
                let mut combined_frame = ViewFrame {
                    width: ViewSize::Infinite,
                    height: ViewSize::Infinite
                };
                let child_ast_iter = child_asts.iter_mut();
                for child_ast in child_ast_iter {
                    let frame = Compositor::layout_ast_recurse(child_ast, MergeStyle::ParentUnlessInfinite, MergeStyle::ParentUnlessInfinite);
                    combined_frame = Compositor::reconcile_frames(combined_frame, frame, horizontal_merge_style, vertical_merge_style);
                }
                // println!("Detected combined frame: {:?}", combined_frame);
                for mut child_ast in child_asts {
                    let frame = Compositor::layout_ast_recurse(&mut child_ast, MergeStyle::Parent, MergeStyle::Parent);
                    let mut updated_frame = Compositor::reconcile_frames(combined_frame, frame, MergeStyle::Smallest, MergeStyle::Smallest);
                    updated_frame = Compositor::reconcile_frames(list_config.frame, updated_frame, MergeStyle::Smallest, MergeStyle::Smallest);
                    Compositor::modify_ast_frame(child_ast, updated_frame);
                }
                combined_frame
            },
            OpenComposeAST::Container(parent_config, container) => {
                // cloning a reference type, should still refer to the same underlying data
                // test print: println!("parsing container");
                match container.as_mut() {
                    ContainerNode::Row(view_config, open_compose_ast) => {
                        view_config.frame = parent_config.frame;
                        match open_compose_ast {
                            OpenComposeAST::List(list_config, _child_asts) => list_config.frame = view_config.frame,
                            _ => {}
                        }
                        // println!("Entering row recursion. Current AST:");
                        // dbg!(&open_compose_ast);
                        let frame = Compositor::layout_ast_recurse(open_compose_ast, MergeStyle::Additive, MergeStyle::LargestFinite);
                        // Row should expand ignoring width if children exceed frame, so it can be
                        // stuck in a scrollview
                        // test print: println!("Merging parent {:?} with child {:?}", parent_config.frame, frame);
                        let merged_parent_frame = Compositor::reconcile_frames(
                            parent_config.frame,
                            frame,
                            MergeStyle::ParentUnlessInfinite,
                            MergeStyle::ParentUnlessInfinite
                        );
                        // test print: println!("Updating parent frame to {:?}", merged_parent_frame);
                        view_config.frame = merged_parent_frame;
                        parent_config.frame = merged_parent_frame;
                        merged_parent_frame
                    },
                    ContainerNode::Column(view_config, open_compose_ast) => {
                        view_config.frame = parent_config.frame;
                        match open_compose_ast {
                            OpenComposeAST::List(list_config, _child_asts) => list_config.frame = view_config.frame,
                            _ => {}
                        }
                        // println!("Entering column recursion. Current AST:");
                        // dbg!(&open_compose_ast);
                        let frame = Compositor::layout_ast_recurse(open_compose_ast, MergeStyle::LargestFinite, MergeStyle::Additive);
                        // Column should expand ignoring height if children exceed frame, so it can be
                        // stuck in a scrollview
                        // test print: println!("Merging parent {:?} with child {:?}", parent_config.frame, frame);
                        let merged_parent_frame = Compositor::reconcile_frames(
                            parent_config.frame,
                            frame,
                            MergeStyle::ParentUnlessInfinite,
                            MergeStyle::ParentUnlessInfinite
                        );
                        // test print: println!("Updating parent frame to {:?}", merged_parent_frame);
                        view_config.frame = merged_parent_frame;
                        parent_config.frame = merged_parent_frame;
                        merged_parent_frame
                    },
                    ContainerNode::Box(view_config, _open_compose_ast) => {
                        // Boxes shouldn't expand
                        view_config.frame
                    },
                    ContainerNode::Button(view_config, _open_compose_ast) => {
                        // Buttons shouldn't expand
                        view_config.frame
                    }
                }
            },
        }
    }

    fn reconcile_frames(
        parent_frame: ViewFrame,
        child_frame: ViewFrame,
        horizontal_merge_style: MergeStyle,
        vertical_merge_style: MergeStyle
    ) -> ViewFrame {
        let width: ViewSize = match child_frame.width {
            ViewSize::Infinite => parent_frame.width,
            ViewSize::Finite(child_width) => {
                match parent_frame.width {
                    ViewSize::Infinite => {
                        match horizontal_merge_style {
                            MergeStyle::Largest => ViewSize::Infinite,
                            MergeStyle::Parent => ViewSize::Infinite,
                            _ => ViewSize::Finite(child_width),
                        }

                    }
                    ViewSize::Finite(parent_width) => {
                        match horizontal_merge_style {
                            MergeStyle::Additive => ViewSize::Finite(child_width + parent_width),
                            MergeStyle::Largest | MergeStyle::LargestFinite => {
                                ViewSize::Finite(
                                    if child_width > parent_width {
                                        child_width
                                    } else {
                                        parent_width
                                    }
                                )
                            },
                            MergeStyle::Smallest => {
                                ViewSize::Finite(
                                    if child_width < parent_width {
                                        child_width
                                    } else {
                                        parent_width
                                    }
                                )
                            },
                            MergeStyle::Parent | MergeStyle::ParentUnlessInfinite => {
                                ViewSize::Finite(parent_width)
                            }
                        }
                    },
                }
            },
        };

        let height: ViewSize = match child_frame.height {
            ViewSize::Infinite => parent_frame.height,
            ViewSize::Finite(child_height) => {
                match parent_frame.height {
                    ViewSize::Infinite => {
                        match vertical_merge_style {
                            MergeStyle::Largest => ViewSize::Infinite,
                            MergeStyle::Parent => ViewSize::Infinite,
                            _ => ViewSize::Finite(child_height)
                        }
                    },
                    ViewSize::Finite(parent_height) => {
                        match vertical_merge_style {
                            MergeStyle::Additive => ViewSize::Finite(child_height + parent_height),
                            MergeStyle::Largest | MergeStyle::LargestFinite => {
                                ViewSize::Finite(
                                    if child_height > parent_height {
                                        child_height
                                    } else {
                                        parent_height
                                    }
                                )
                            },
                            MergeStyle::Smallest => {
                                ViewSize::Finite(
                                    if child_height < parent_height {
                                        child_height
                                    } else {
                                        parent_height
                                    }
                                )
                            },
                            MergeStyle::Parent | MergeStyle::ParentUnlessInfinite => {
                                ViewSize::Finite(parent_height)
                            }
                        }
                    },
                }
            },
        };

        ViewFrame {
            width,
            height
        }
    }

    fn modify_ast_frame(ast: &mut OpenComposeAST, new_frame: ViewFrame) {
        // println!("Before modifying frame to {:?}:", new_frame);
        // dbg!(&ast);
        match ast {
            OpenComposeAST::View(view_config, node_ast) => {
                view_config.frame = new_frame;
                match node_ast {
                    crate::ast::ViewNode::Image(view_config, _) => view_config.frame = new_frame,
                    crate::ast::ViewNode::Text(view_config, _) => view_config.frame = new_frame,
                }
            },
            OpenComposeAST::List(view_config, _node_ast) => {
                view_config.frame = new_frame
            },
            OpenComposeAST::Container(view_config, node_ast) => {
                view_config.frame = new_frame;
                match node_ast.as_mut() {
                    ContainerNode::Row(view_config, _open_compose_ast) => view_config.frame = new_frame,
                    ContainerNode::Column(view_config, _open_compose_ast) => view_config.frame = new_frame,
                    ContainerNode::Box(view_config, _open_compose_ast) => view_config.frame = new_frame,
                    ContainerNode::Button(view_config, _open_compose_ast) => view_config.frame = new_frame,
                }
            },
        }
        // println!("After:");
        // dbg!(&ast);
    }
}

#[derive(Copy, Clone)]
enum MergeStyle {
    Additive,
    Largest,
    LargestFinite,
    Smallest,
    Parent,
    ParentUnlessInfinite
}
