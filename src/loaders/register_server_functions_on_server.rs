use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default)]
pub struct RegisterServerFunctionVisitor {
    registry: Vec<ModuleItem>,
    current_class: Option<Ident>,
}

fn runtime_register_function(class_name: &Ident, function_name: &Ident) -> ModuleItem {
    runtime_register(vec![
        ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Ident(class_name.clone())),
        },
        ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: function_name.clone().sym,
                raw: None,
            }))),
        },
    ])
}

fn runtime_register_class(class_name: &Ident) -> ModuleItem {
    runtime_register(vec![ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Ident(class_name.clone())),
    }])
}

fn runtime_register(args: Vec<ExprOrSpread>) -> ModuleItem {
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
                    sym: "register".into(),
                    optional: false,
                }),
            }))),
            args,
            type_args: None,
        })),
    }))
}

impl VisitMut for RegisterServerFunctionVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.visit_mut_children_with(self);
        n.body.extend(self.registry.clone());
    }

    fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) {
        self.current_class = Some(n.ident.clone());
        n.visit_mut_children_with(self);
        self.registry.push(runtime_register_class(&n.ident));
        self.current_class = None;
    }

    fn visit_mut_class_expr(&mut self, n: &mut ClassExpr) {
        if let Some(ident) = &mut n.ident.clone() {
            self.current_class = Some(ident.clone());
            n.visit_mut_children_with(self);
            self.registry.push(runtime_register_class(ident));
            self.current_class = None;
        }
    }

    fn visit_mut_class_member(&mut self, n: &mut ClassMember) {
        if let ClassMember::Method(m) = n {
            if m.is_static && m.function.is_async {
                if let Some(class_name) = &self.current_class {
                    if let Some(function_name) = m.key.clone().ident() {
                        if !function_name.sym.starts_with('_') {
                            self.registry
                                .push(runtime_register_function(class_name, &function_name));
                        }
                    }
                }
            }
        }
    }
}
