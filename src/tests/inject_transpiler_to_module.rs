#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::inject_transpiler_to_module::InjectTranspilerVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(InjectTranspilerVisitor::default()),
    inject_transpiler,
    r#"
        import Nullstack from "nullstack"; 
        class Component {
            static async server() {}
            render() { return <div>hello</div> }
        };
    "#,
    r#"
        import Nullstack, { $transpiler } from "nullstack"; 
        class Component {
            static async server() {}
            render() { return <div>hello</div> }
        }; 
    "#
);

test!(
    syntax(),
    |_| tr(InjectTranspilerVisitor::default()),
    inject_transpiler_in_same_context,
    r#"function Component() { return <div>hello</div> }"#,
    r#"import { $transpiler } from "nullstack"; function Component() { return <div>hello</div> }"#
);
