#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::inject_runtime_to_module::InjectRuntimeVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(InjectRuntimeVisitor::default()),
    inject_runtime_to_classes,
    r#"
        import Nullstack from "nullstack"; 
        class Component {
            static async server() {}
            render() { return <div>hello</div> }
        };
    "#,
    r#"
        import $runtime from "nullstack/runtime";
        import Nullstack from "nullstack"; 
        class Component {
            static async server() {}
            render() { return <div>hello</div> }
        }; 
    "#
);

test!(
    syntax(),
    |_| tr(InjectRuntimeVisitor::default()),
    inject_runtime_to_functions,
    r#"function Component() { return <div>hello</div> }"#,
    r#"import $runtime from "nullstack/runtime"; function Component() { return <div>hello</div> }"#
);
