#[allow(unused_imports)]
use super::syntax;
use crate::loaders::replace_server_functions_on_client::ReplaceServerFunctionVisitor;
use swc_core::ecma::{
    transforms::testing::test,
    visit::{as_folder, Fold},
};

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(ReplaceServerFunctionVisitor::default())
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
