#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::replace_server_functions_on_client::ReplaceServerFunctionVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(ReplaceServerFunctionVisitor::default()),
    inject_nullstack,
    r#"class Component { static async server() { console.log("server") } };"#,
    r#"class Component { static server = $runtime.invoke("server", this.hash) };"#
);

test!(
    syntax(),
    |_| tr(ReplaceServerFunctionVisitor::default()),
    skip_inject_nullstack_when_not_async,
    r#"class Component { static server() { console.log("isomorphic") } };"#,
    r#"class Component { static server() { console.log("isomorphic") } };"#
);

test!(
    syntax(),
    |_| tr(ReplaceServerFunctionVisitor::default()),
    skip_inject_nullstack_when_not_static,
    r#"class Component { async server() { console.log("client") } };"#,
    r#"class Component { async server() { console.log("client") } };"#
);

test!(
    syntax(),
    |_| tr(ReplaceServerFunctionVisitor::default()),
    remove_server_functions_starting_with_underline,
    r#"class Component { static async _server() { console.log("client") } };"#,
    r#"class Component { };"#
);
