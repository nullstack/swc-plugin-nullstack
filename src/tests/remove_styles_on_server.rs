#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::remove_styles_on_server::RemoveStylesVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(RemoveStylesVisitor::default()),
    remove_css,
    r#"import "styles.css"; class Component { };"#,
    r#"class Component { };"#
);

test!(
    syntax(),
    |_| tr(RemoveStylesVisitor::default()),
    remove_scss,
    r#"import "styles.scss"; class Component { };"#,
    r#"class Component { };"#
);

test!(
    syntax(),
    |_| tr(RemoveStylesVisitor::default()),
    remove_sass,
    r#"import "styles.sass"; class Component { };"#,
    r#"class Component { };"#
);
