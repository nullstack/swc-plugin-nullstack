use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    transforms::testing::test,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut},
};
use swc_ecma_parser::{EsConfig, Syntax};

pub struct InjectSourceVisitor {}

impl Default for InjectSourceVisitor {
    fn default() -> Self {
        InjectSourceVisitor {}
    }
}

fn should_inject_source_attribute(n: &JSXOpeningElement) -> bool {
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

fn source_attribute() -> JSXAttrOrSpread {
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

impl VisitMut for InjectSourceVisitor {
    noop_visit_mut_type!();

    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        if should_inject_source_attribute(n) {
            n.attrs.push(source_attribute())
        }
    }
}

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(InjectSourceVisitor::default())
}

#[allow(dead_code)]
fn syntax() -> Syntax {
    let mut config = EsConfig::default();
    config.jsx = true;
    Syntax::Es(config)
}

test!(
    syntax(),
    |_| tr(),
    inject_source_to_node,
    r#"function Modal() { return <button onclick={close}> x </button> }"#,
    r#"function Modal() { return <button onclick={close} source={this}> x </button> }"#
);

test!(
    syntax(),
    |_| tr(),
    skip_inject_duplicated_source_to_node,
    r#"function Modal() { return <button source={this} onclick={close}> x </button> }"#,
    r#"function Modal() { return <button source={this} onclick={close}> x </button> }"#
);

test!(
    syntax(),
    |_| tr(),
    skip_inject_source_to_node,
    r#"function Modal() { return <button class="kawaii-desu"> x </button> }"#,
    r#"function Modal() { return <button class="kawaii-desu"> x </button> }"#
);
