#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::inject_hash_to_class::InjectHashVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(InjectHashVisitor::new("/src/Application.njs".into(), true)),
    inject_dev_hash,
    r#"class Component extends Nullstack { works = true };"#,
    r#"class Component extends Nullstack { static hash = "__src__Application__njs"; works = true };"#
);

test!(
    Default::default(),
    |_| tr(InjectHashVisitor::new("/src/Application.njs".into(), false)),
    inject_prod_hash,
    r#"class Component extends Nullstack { works = true };"#,
    r#"class Component extends Nullstack { static hash = "e7eacfb84f0534dc757c0c4752385e2c"; works = true };"#
);
