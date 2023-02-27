use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default)]
pub struct InjectAcceptVisitor {
    class_names: Vec<Ident>,
}

fn runtime_accept(class_names: &[Ident]) -> ModuleItem {
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
            args: class_names
                .iter()
                .map(|class_name| ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Ident(class_name.clone())),
                })
                .collect(),
            type_args: None,
        })),
    }))
}

impl VisitMut for InjectAcceptVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        self.class_names.push(Ident {
            span: DUMMY_SP,
            sym: "module".into(),
            optional: false,
        });
        n.visit_mut_children_with(self);
        n.body.push(runtime_accept(&self.class_names));
    }

    fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) {
        self.class_names.push(n.ident.clone());
    }
}
