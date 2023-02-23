#[allow(unused_imports)]
use super::syntax;
use crate::loaders::inject_hash_to_class::InjectHashVisitor;
use swc_core::ecma::transforms::testing::test;
use swc_core::ecma::visit::{as_folder, Fold};

#[allow(dead_code)]
fn tr(is_dev: bool) -> impl Fold {
    as_folder(InjectHashVisitor::new(
        "/src/Application.njs".into(),
        is_dev,
    ))
}

test!(
    syntax(),
    |_| tr(true),
    inject_dev_hash,
    r#"class Component extends Nullstack { works = true };"#,
    r#"class Component extends Nullstack { static hash = "__src__Application__njs"; works = true };"#
);

test!(
    Default::default(),
    |_| tr(false),
    inject_prod_hash,
    r#"class Component extends Nullstack { works = true };"#,
    r#"class Component extends Nullstack { static hash = "e7eacfb84f0534dc757c0c4752385e2c"; works = true };"#
);
