use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

use super::helpers::transpiler_ident;

#[derive(Default)]
pub struct RegisterServerFunctionVisitor {
    registry: Vec<ModuleItem>,
    current_class: Option<ClassDecl>,
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
                    obj: Box::new(Expr::Ident(transpiler_ident())),
                    prop: member_prop_ident("registry".into()),
                })),
                prop: MemberProp::Computed(ComputedPropName {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: box_ident_expr(n.ident.sym.clone()),
                        prop: member_prop_ident("hash".into()),
                    })),
                }),
            }))),
            right: box_ident_expr(n.ident.sym.clone()),
        })),
    }))
}

fn register_function(n: &ClassDecl, f: &Ident) -> ModuleItem {
    let invocation = format!(".{}", &f.sym);
    ModuleItem::Stmt(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            op: AssignOp::Assign,
            left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(transpiler_ident())),
                    prop: member_prop_ident("registry".into()),
                })),
                prop: MemberProp::Computed(ComputedPropName {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Tpl(Tpl {
                        span: DUMMY_SP,
                        exprs: vec![Box::new(Expr::Member(MemberExpr {
                            span: DUMMY_SP,
                            obj: box_ident_expr(n.ident.sym.clone()),
                            prop: member_prop_ident("hash".into()),
                        }))],
                        quasis: vec![tpl_element("".into()), tpl_element(invocation.into())],
                    })),
                }),
            }))),
            right: Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: box_ident_expr(n.ident.sym.clone()),
                prop: member_prop_ident(f.sym.clone()),
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
                obj: box_ident_expr(n.ident.sym.clone()),
                prop: member_prop_ident("bindStaticFunctions".into()),
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: box_ident_expr(n.ident.sym.clone()),
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
            if m.is_static && m.function.is_async {
                if let Some(decl) = self.current_class.clone() {
                    if let Some(ident) = m.key.clone().ident() {
                        self.registry.push(register_function(&decl, &ident));
                    }
                }
            }
        }
    }
}
