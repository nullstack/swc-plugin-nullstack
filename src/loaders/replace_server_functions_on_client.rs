use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    atoms::{Atom, JsWord},
    transforms::testing::test,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut},
};
use swc_ecma_parser::{EsConfig, Syntax};

pub struct ReplaceServerFunctionVisitor {}

impl Default for ReplaceServerFunctionVisitor {
    fn default() -> Self {
        ReplaceServerFunctionVisitor {}
    }
}

fn invoke_arg(value: Atom) -> ExprOrSpread {
    ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Lit(Lit::Str(Str {
            span: DUMMY_SP,
            value: JsWord::from(&*value),
            raw: Some(format!("'{}'", value).into()),
        }))),
    }
}

fn invoke_calle() -> Callee {
    Callee::Expr(Box::new(Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident {
            span: DUMMY_SP,
            sym: "Nullstack".into(),
            optional: false,
        })),
        prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: "_invoke".into(),
            optional: false,
        }),
    })))
}

fn invoke_value(function_name: Atom, class_hash: Atom) -> Option<Box<Expr>> {
    Option::Some(Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: invoke_calle(),
        args: vec![invoke_arg(function_name), invoke_arg(class_hash)],
        type_args: None,
    })))
}

fn invoke_prop(key: PropName) -> ClassMember {
    let function_name = key.clone().ident().unwrap().sym;
    let class_hash = "HASH";
    ClassMember::ClassProp(ClassProp {
        span: DUMMY_SP,
        key,
        value: invoke_value(function_name.into(), class_hash.into()),
        type_ann: None,
        is_static: false,
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

    fn visit_mut_class_member(&mut self, n: &mut ClassMember) {
        if let ClassMember::Method(m) = n {
            if m.is_static && m.function.is_async && m.key.clone().ident().is_some() {
                *n = invoke_prop(m.key.clone());
            }
        }
    }
}

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(ReplaceServerFunctionVisitor::default())
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
    inject_nullstack,
    r#"class Component { static async server() { console.log("server") } };"#,
    r#"class Component { server = Nullstack._invoke('server', 'HASH') };"#
);

test!(
    Default::default(),
    |_| tr(),
    skip_inject_nullstack_when_not_async,
    r#"class Component { static server() { console.log("isomorphic") } };"#,
    r#"class Component { static server() { console.log("isomorphic") } };"#
);

test!(
    Default::default(),
    |_| tr(),
    skip_inject_nullstack_when_not_static,
    r#"class Component { async server() { console.log("client") } };"#,
    r#"class Component { async server() { console.log("client") } };"#
);
