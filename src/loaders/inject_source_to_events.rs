use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut},
};

#[derive(Default)]
pub struct InjectSourceVisitor {}

impl InjectSourceVisitor {
    fn should_inject_source_attribute(&self, n: &JSXOpeningElement) -> bool {
        let mut has_event = false;
        for attr in n.attrs.clone().into_iter() {
            if let JSXAttrOrSpread::JSXAttr(b) = attr {
                if let JSXAttrName::Ident(c) = b.clone().name {
                    if c.sym.eq_str_ignore_ascii_case("source") {
                        return false;
                    }
                    if c.sym.starts_with("on") {
                        has_event = true;
                    }
                }
            }
        }
        has_event
    }

    fn source_attribute(&self) -> JSXAttrOrSpread {
        JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: DUMMY_SP,
            name: JSXAttrName::Ident(Ident {
                span: DUMMY_SP,
                sym: "source".into(),
                optional: false,
            }),
            value: Option::Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                span: DUMMY_SP,
                expr: JSXExpr::Expr(Box::new(Expr::This(ThisExpr { span: DUMMY_SP }))),
            })),
        })
    }
}

impl VisitMut for InjectSourceVisitor {
    noop_visit_mut_type!();

    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        if self.should_inject_source_attribute(n) {
            n.attrs.push(self.source_attribute())
        }
    }
}
