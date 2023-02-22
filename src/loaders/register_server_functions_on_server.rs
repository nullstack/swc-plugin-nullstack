use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    transforms::testing::test,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};
use swc_ecma_parser::{EsConfig, Syntax};

pub struct RegisterServerFunctionVisitor {
    registry: Vec<ModuleItem>,
    current_class: Option<ClassDecl>,
}

impl Default for RegisterServerFunctionVisitor {
    fn default() -> Self {
        RegisterServerFunctionVisitor {
            registry: vec![],
            current_class: None,
        }
    }
}

fn member_prop_ident(sym: JsWord) -> MemberProp {
    MemberProp::Ident(Ident {
        span: DUMMY_SP,
        sym,
        optional: false,
    })
}

fn box_ident_expr(sym: JsWord) -> Box<Expr> {
    Box::new(Expr::Ident(Ident {
        span: DUMMY_SP,
        sym,
        optional: false,
    }))
}

fn tpl_element(sym: JsWord) -> TplElement {
    TplElement {
        span: DUMMY_SP,
        tail: false,
        cooked: Some(sym.clone().into()),
        raw: sym.into(),
    }
}

fn register_class(n: &ClassDecl) -> ModuleItem {
    ModuleItem::Stmt(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            op: AssignOp::Assign,
            left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: box_ident_expr(n.ident.sym.clone().into()),
                    prop: member_prop_ident("registry".into()),
                })),
                prop: MemberProp::Computed(ComputedPropName {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: box_ident_expr(n.ident.sym.clone().into()),
                        prop: member_prop_ident("hash".into()),
                    })),
                }),
            }))),
            right: box_ident_expr(n.ident.sym.clone().into()),
        })),
    }))
}

fn register_function(n: &ClassDecl, f: &ClassMethod) -> ModuleItem {
    let invocation = format!(".{}", f.key.clone().ident().unwrap().sym);
    ModuleItem::Stmt(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            op: AssignOp::Assign,
            left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: box_ident_expr(n.ident.sym.clone().into()),
                    prop: member_prop_ident("registry".into()),
                })),
                prop: MemberProp::Computed(ComputedPropName {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Tpl(Tpl {
                        span: DUMMY_SP,
                        exprs: vec![Box::new(Expr::Member(MemberExpr {
                            span: DUMMY_SP,
                            obj: box_ident_expr(n.ident.sym.clone().into()),
                            prop: member_prop_ident("hash".into()),
                        }))],
                        quasis: vec![
                            tpl_element("".into()),
                            tpl_element(invocation.clone().into()),
                        ],
                    })),
                }),
            }))),
            right: Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: box_ident_expr(n.ident.sym.clone().into()),
                prop: member_prop_ident(f.key.clone().ident().unwrap().sym),
            })),
        })),
    }))
}

fn register_bind(n: &ClassDecl) -> ModuleItem {
    ModuleItem::Stmt(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: box_ident_expr(n.ident.sym.clone().into()),
                prop: member_prop_ident("bindStaticFunctions".into()),
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: box_ident_expr(n.ident.sym.clone().into()),
            }],
            type_args: None,
        })),
    }))
}

impl VisitMut for RegisterServerFunctionVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.visit_mut_children_with(self);
        n.body.extend(self.registry.clone());
        // if (module.hot) { module.hot.accept(); }
    }

    fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) {
        self.current_class = Some(n.clone());
        let number_of_server_functions = self.registry.len();
        n.visit_mut_children_with(self);
        if self.registry.len() > number_of_server_functions {
            self.registry.push(register_class(n));
            self.registry.push(register_bind(n));
        }
        self.current_class = None;
    }

    fn visit_mut_class_member(&mut self, n: &mut ClassMember) {
        if let ClassMember::Method(m) = n {
            if m.is_static && m.function.is_async && m.key.clone().ident().is_some() {
                self.registry
                    .push(register_function(&self.current_class.clone().unwrap(), &m));
            }
        }
    }
}

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(RegisterServerFunctionVisitor::default())
}

#[allow(dead_code)]
fn syntax() -> Syntax {
    let mut config = EsConfig::default();
    config.jsx = true;
    Syntax::Es(config)
}

test!(
    Default::default(),
    |_| tr(),
    register_server_functions,
    r#"class Component { static async server() { console.log("server") } };"#,
    r#"
        class Component { static async server() { console.log("server") } };
        Component.registry[`${Component.hash}.server`] = Component.server;
        Component.registry[Component.hash] = Component;
        Component.bindStaticFunctions(Component);
    "#
);

test!(
    Default::default(),
    |_| tr(),
    register_server_functions_with_multiple_classes,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { static async server() { console.log("server") } };
    "#,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { static async server() { console.log("server") } };
        Component.registry[`${Component.hash}.server`] = Component.server;
        Component.registry[Component.hash] = Component;
        Component.bindStaticFunctions(Component);
        Component2.registry[`${Component2.hash}.server`] = Component2.server;
        Component2.registry[Component2.hash] = Component2;
        Component2.bindStaticFunctions(Component2);
    "#
);

test!(
    Default::default(),
    |_| tr(),
    skip_register_server_functions_with_multiple_classes,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { };
    "#,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { };
        Component.registry[`${Component.hash}.server`] = Component.server;
        Component.registry[Component.hash] = Component;
        Component.bindStaticFunctions(Component);
    "#
);
