use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
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

impl ReplaceServerFunctionVisitor {
    fn invoke_arg_hash(&self) -> ExprOrSpread {
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
        }
    }

    fn invoke_arg_function_name(&self, function_name: JsWord) -> ExprOrSpread {
        ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: JsWord::from(&*function_name),
                raw: Some(format!("'{}'", function_name).into()),
            }))),
        }
    }

    fn invoke_value(&self, key: PropName) -> Option<Box<Expr>> {
        match key.clone().ident() {
            Some(function_name) => Option::Some(Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: invoke_calle(),
                args: vec![
                    self.invoke_arg_function_name(function_name.sym),
                    self.invoke_arg_hash(),
                ],
                type_args: None,
            }))),
            _ => None,
        }
    }

    fn invoke_prop(&self, key: PropName) -> ClassMember {
        ClassMember::ClassProp(ClassProp {
            span: DUMMY_SP,
            key: key.clone(),
            value: self.invoke_value(key),
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
}

fn invoke_calle() -> Callee {
    Callee::Expr(Box::new(Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident {
            span: DUMMY_SP,
            sym: "$transpiler".into(),
            optional: false,
        })),
        prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: "invoke".into(),
            optional: false,
        }),
    })))
}

impl VisitMut for ReplaceServerFunctionVisitor {
    noop_visit_mut_type!();

    fn visit_mut_class_member(&mut self, n: &mut ClassMember) {
        if let ClassMember::Method(m) = n {
            if m.is_static && m.function.is_async && m.key.clone().ident().is_some() {
                *n = self.invoke_prop(m.key.clone());
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
    syntax(),
    |_| tr(),
    inject_nullstack,
    r#"class Component { static async server() { console.log("server") } };"#,
    r#"class Component { static server = $transpiler.invoke('server', this.hash) };"#
);

test!(
    syntax(),
    |_| tr(),
    skip_inject_nullstack_when_not_async,
    r#"class Component { static server() { console.log("isomorphic") } };"#,
    r#"class Component { static server() { console.log("isomorphic") } };"#
);

test!(
    syntax(),
    |_| tr(),
    skip_inject_nullstack_when_not_static,
    r#"class Component { async server() { console.log("client") } };"#,
    r#"class Component { async server() { console.log("client") } };"#
);
