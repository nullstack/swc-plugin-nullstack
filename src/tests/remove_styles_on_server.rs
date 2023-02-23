#[allow(unused_imports)]
use super::syntax;
use crate::loaders::remove_styles_on_server::RemoveStylesVisitor;
use swc_core::ecma::{
    transforms::testing::test,
    visit::{as_folder, Fold},
};

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(RemoveStylesVisitor::default())
}

test!(
    syntax(),
    |_| tr(),
    remove_css,
    r#"import "styles.css"; class Component { };"#,
    r#"class Component { };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_scss,
    r#"import "styles.scss"; class Component { };"#,
    r#"class Component { };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_sass,
    r#"import "styles.sass"; class Component { };"#,
    r#"class Component { };"#
);
