use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default)]
pub struct ReplaceRefVisitor {
    current: Option<MemberExpr>,
}

impl VisitMut for ReplaceRefVisitor {
    noop_visit_mut_type!();

    fn visit_mut_jsx_attr_or_spread(&mut self, n: &mut JSXAttrOrSpread) {
        if let JSXAttrOrSpread::JSXAttr(attr) = n {
            if let JSXAttrName::Ident(ident) = &attr.name {
                if ident.sym.eq("ref") || ident.sym.eq("bind") {
                    if let Some(JSXAttrValue::JSXExprContainer(container)) = &attr.value {
                        if let JSXExpr::Expr(expr) = &container.expr {
                            if let Expr::Member(a) = &**expr {
                                self.current = Some(a.clone());
                                n.visit_mut_children_with(self);
                                self.current = None;
                            }
                        }
                    }
                }
            }
        }
    }

    fn visit_mut_jsx_expr(&mut self, n: &mut JSXExpr) {
        if let Some(a) = &self.current {
            let prop = match a.clone().prop {
                MemberProp::Ident(i) => Box::new(Expr::Lit(Lit::Str(i.sym.into()))),
                MemberProp::PrivateName(i) => Box::new(Expr::Ident(i.id)),
                MemberProp::Computed(i) => i.expr,
            };
            *n = JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                span: a.span,
                props: vec![
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(Ident {
                            span: a.clone().span,
                            sym: "object".into(),
                            optional: false,
                        }),
                        value: a.clone().obj,
                    }))),
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(Ident {
                            span: a.clone().span,
                            sym: "property".into(),
                            optional: false,
                        }),
                        value: prop,
                    }))),
                ],
            })));
        }
    }
}
