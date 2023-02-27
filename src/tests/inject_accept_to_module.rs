#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::inject_accept_to_module::InjectAcceptVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::default()),
    inject_accept,
    r#"class Component {};"#,
    r#"class Component {}; $runtime.accept(module, Component)"#
);

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::default()),
    inject_multiple_accept,
    r#"class Component {}; class Component2 {};"#,
    r#"class Component {}; class Component2 {}; $runtime.accept(module, Component, Component2)"#
);
