#[allow(unused_imports)]
use super::syntax;
use crate::loaders::register_server_functions_on_server::RegisterServerFunctionVisitor;
use swc_core::ecma::{
    transforms::testing::test,
    visit::{as_folder, Fold},
};

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(RegisterServerFunctionVisitor::default())
}

test!(
    Default::default(),
    |_| tr(),
    register_server_functions,
    r#"class Component { static async server() { console.log("server") } };"#,
    r#"
        class Component { static async server() { console.log("server") } };
        $transpiler.registry[`${Component.hash}.server`] = Component.server;
        $transpiler.registry[Component.hash] = Component;
        Component.bindStaticFunctions(Component);
    "#
);

test!(
    Default::default(),
    |_| tr(),
    register_server_functions_with_multiple_classes,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { static async server() { console.log("server") } };
    "#,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { static async server() { console.log("server") } };
        $transpiler.registry[`${Component.hash}.server`] = Component.server;
        $transpiler.registry[Component.hash] = Component;
        Component.bindStaticFunctions(Component);
        $transpiler.registry[`${Component2.hash}.server`] = Component2.server;
        $transpiler.registry[Component2.hash] = Component2;
        Component2.bindStaticFunctions(Component2);
    "#
);

test!(
    Default::default(),
    |_| tr(),
    skip_register_server_functions_with_multiple_classes,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { };
    "#,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { };
        $transpiler.registry[`${Component.hash}.server`] = Component.server;
        $transpiler.registry[Component.hash] = Component;
        Component.bindStaticFunctions(Component);
    "#
);
