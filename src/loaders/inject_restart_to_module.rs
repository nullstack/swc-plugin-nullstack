use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default)]
pub struct InjectRestartVisitor {
    nullstack: Option<Ident>,
    starter: Option<Ident>,
    starter_path: Option<String>,
}

fn runtime_restart(starter_path: String) -> ModuleItem {
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
                    sym: "restart".into(),
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
                    expr: Box::new(Expr::Lit(Lit::Str(starter_path.into()))),
                },
            ],
            type_args: None,
        })),
    }))
}

impl VisitMut for InjectRestartVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        // find nullstack ident
        n.visit_mut_children_with(self);
        // find starter ident
        n.visit_mut_children_with(self);
        // find application path
        n.visit_mut_children_with(self);
        if let Some(starter_path) = &self.starter_path {
            n.body.push(runtime_restart(starter_path.clone()));
        }
    }

    fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {
        if let Some(nullstack) = &self.nullstack {
            if let Callee::Expr(c) = &n.callee {
                if let Expr::Member(expr) = &**c {
                    if let Some(ident) = expr.clone().obj.ident() {
                        if ident.sym == nullstack.sym {
                            if let MemberProp::Ident(prop) = &expr.prop {
                                if prop.sym.eq("start") {
                                    if let Some(spread) = n.args.first() {
                                        if let Some(ident) = spread.expr.clone().ident() {
                                            self.starter = Some(ident)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        if self.nullstack.is_none() {
            if n.src.value.eq("nullstack") {
                for specifier in n.specifiers.iter() {
                    if let ImportSpecifier::Default(ident) = specifier {
                        self.nullstack = Some(ident.local.clone())
                    }
                }
            }
        } else if let Some(starter) = &self.starter {
            n.specifiers.iter().find(|specifier| {
                let local = match specifier {
                    ImportSpecifier::Named(s) => &s.local,
                    ImportSpecifier::Default(s) => &s.local,
                    ImportSpecifier::Namespace(s) => &s.local,
                };
                if local.sym.clone() == starter.sym {
                    self.starter_path = Some(n.src.value.to_string())
                }
                self.starter_path.is_some()
            });
        }
    }

    fn visit_mut_member_expr(&mut self, n: &mut MemberExpr) {
        if let Some(ident) = n.obj.clone().ident() {
            if ident.sym.eq("Nullstack") {
                if let MemberProp::Ident(prop) = &n.prop {
                    if prop.sym.eq("start") {
                        panic!("{:#?}", n);
                    }
                }
            }
        }
    }
}
