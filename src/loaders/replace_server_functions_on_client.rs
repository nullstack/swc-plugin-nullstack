use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default)]
pub struct ReplaceServerFunctionVisitor {}

fn runtime_invoke(function_name: &JsWord) -> ClassMember {
    ClassMember::ClassProp(ClassProp {
        span: DUMMY_SP,
        key: PropName::Ident(Ident {
            span: DUMMY_SP,
            sym: function_name.clone(),
            optional: false,
        }),
        value: Some(Box::new(Expr::Call(CallExpr {
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
                    sym: "invoke".into(),
                    optional: false,
                }),
            }))),
            args: vec![
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: function_name.clone(),
                        raw: None,
                    }))),
                },
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: Box::new(Expr::This(ThisExpr { span: DUMMY_SP })),
                        prop: MemberProp::Ident(Ident {
                            span: DUMMY_SP,
                            sym: "hash".into(),
                            optional: false,
                        }),
                    })),
                },
            ],
            type_args: None,
        }))),
        type_ann: None,
        is_static: true,
        decorators: vec![],
        accessibility: None,
        is_abstract: false,
        is_optional: false,
        is_override: false,
        readonly: false,
        declare: false,
        definite: false,
    })
}

impl VisitMut for ReplaceServerFunctionVisitor {
    noop_visit_mut_type!();

    fn visit_mut_class_members(&mut self, n: &mut Vec<ClassMember>) {
        n.retain(|member| {
            if let ClassMember::Method(m) = member {
                if m.is_static && m.function.is_async {
                    if let Some(function_name) = &m.key.clone().ident() {
                        if function_name.sym.starts_with('_') {
                            return false;
                        }
                    }
                }
            }
            true
        });
        n.visit_mut_children_with(self);
    }

    fn visit_mut_class_member(&mut self, n: &mut ClassMember) {
        if let ClassMember::Method(m) = n {
            if m.is_static && m.function.is_async {
                if let Some(function_name) = &m.key.clone().ident() {
                    *n = runtime_invoke(&function_name.sym);
                }
            }
        }
    }
}
