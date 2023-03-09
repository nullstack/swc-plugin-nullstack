#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::register_server_functions_on_server::RegisterServerFunctionVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    Default::default(),
    |_| tr(RegisterServerFunctionVisitor::default()),
    register_server_functions,
    r#"class Component { static async server() { console.log("server") } };"#,
    r#"
        class Component { static async server() { console.log("server") } };
        $runtime.register(Component, "server");
        $runtime.register(Component);
    "#
);

test!(
    Default::default(),
    |_| tr(RegisterServerFunctionVisitor::default()),
    register_server_functions_with_multiple_classes,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { static async server() { console.log("server") } };
    "#,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { static async server() { console.log("server") } };
        $runtime.register(Component, "server");
        $runtime.register(Component);
        $runtime.register(Component2, "server");
        $runtime.register(Component2);
    "#
);

test!(
    Default::default(),
    |_| tr(RegisterServerFunctionVisitor::default()),
    skip_register_server_functions_with_multiple_classes,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { };
    "#,
    r#"
        class Component { static async server() { console.log("server") } };
        class Component2 { };
        $runtime.register(Component, "server");
        $runtime.register(Component);
    "#
);

test!(
    Default::default(),
    |_| tr(RegisterServerFunctionVisitor::default()),
    skip_register_server_functions_starting_with_underline,
    r#"
        class Component { 
            static async server() { console.log("server") }
            static async _private() { console.log("server") } 
        };
    "#,
    r#"
        class Component { 
            static async server() { console.log("server") }
            static async _private() { console.log("server") } 
        };
        $runtime.register(Component, "server");
        $runtime.register(Component);
    "#
);

test!(
    Default::default(),
    |_| tr(RegisterServerFunctionVisitor::default()),
    register_server_functions_when_exported_as_named,
    r#"export class Component { static async server() { console.log("server") } };"#,
    r#"
        export class Component { static async server() { console.log("server") } };
        $runtime.register(Component, "server");
        $runtime.register(Component);
    "#
);

test!(
    Default::default(),
    |_| tr(RegisterServerFunctionVisitor::default()),
    register_server_functions_when_exported_as_default,
    r#"export default class Component { static async server() { console.log("server") } };"#,
    r#"
        export default class Component { static async server() { console.log("server") } };
        $runtime.register(Component, "server");
        $runtime.register(Component);
    "#
);
