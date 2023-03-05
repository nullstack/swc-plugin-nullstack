#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::inject_accept_to_module::InjectAcceptVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::new("/src/Application.njs".into())),
    inject_accept,
    r#"
        class Component {};
    "#,
    r#"
        class Component {};
        $runtime.accept(module, "/src/Application.njs", {klasses: [Component], dependencies: []})
    "#
);

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::new("/src/Application.njs".into())),
    inject_multiple_accept,
    r#"
        class Component {}; 
        class Component2 {};
    "#,
    r#"
        class Component {}; 
        class Component2 {}; 
        $runtime.accept(module, "/src/Application.njs", {klasses: [Component, Component2], dependencies: []})
    "#
);

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::new("/src/Application.njs".into())),
    inject_multiple_imports,
    r#"
        import Nullstack from 'nullstack'; 
        import Logo from 'nullstack/logo'; 
        class Component {}; 
        class Component2 {};
    "#,
    r#"
        import Nullstack from 'nullstack'; 
        import Logo from 'nullstack/logo'; 
        class Component {}; 
        class Component2 {}; 
        $runtime.accept(module, "/src/Application.njs", {klasses: [Component, Component2], dependencies: ["nullstack", "nullstack/logo"]})"#
);
