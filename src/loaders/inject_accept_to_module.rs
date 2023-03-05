use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default)]
pub struct InjectAcceptVisitor {
    class_names: Vec<Ident>,
    import_paths: Vec<JsWord>,
}

fn runtime_accept(class_names: &Vec<Ident>, import_paths: &Vec<JsWord>) -> ModuleItem {
    ModuleItem::Stmt(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "$runtime".into(),
                    optional: false,
                })),
                prop: MemberProp::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "accept".into(),
                    optional: false,
                }),
            }))),
            args: vec![
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Ident(Ident {
                        span: DUMMY_SP,
                        sym: "module".into(),
                        optional: false,
                    })),
                },
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Object(ObjectLit {
                        span: DUMMY_SP,
                        props: vec![
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(Ident {
                                    span: DUMMY_SP,
                                    sym: "klasses".into(),
                                    optional: false,
                                }),
                                value: Box::new(Expr::Array(ArrayLit {
                                    span: DUMMY_SP,
                                    elems: class_names
                                        .iter()
                                        .map(|class_name| {
                                            Some(ExprOrSpread {
                                                spread: None,
                                                expr: Box::new(Expr::Ident(class_name.clone())),
                                            })
                                        })
                                        .collect(),
                                })),
                            }))),
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(Ident {
                                    span: DUMMY_SP,
                                    sym: "dependencies".into(),
                                    optional: false,
                                }),
                                value: Box::new(Expr::Array(ArrayLit {
                                    span: DUMMY_SP,
                                    elems: import_paths
                                        .iter()
                                        .map(|import_path| {
                                            Some(ExprOrSpread {
                                                spread: None,
                                                expr: Box::new(Expr::Lit(Lit::Str(Str {
                                                    span: DUMMY_SP,
                                                    value: import_path.clone(),
                                                    raw: None,
                                                }))),
                                            })
                                        })
                                        .collect(),
                                })),
                            }))),
                        ],
                    })),
                },
            ],
            type_args: None,
        })),
    }))
}

impl VisitMut for InjectAcceptVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.visit_mut_children_with(self);
        n.body
            .push(runtime_accept(&self.class_names, &self.import_paths));
    }

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        self.import_paths.push(n.src.value.clone());
    }

    fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) {
        self.class_names.push(n.ident.clone());
    }
}
