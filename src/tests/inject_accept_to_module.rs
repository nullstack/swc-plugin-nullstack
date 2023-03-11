#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::inject_accept_to_module::InjectAcceptVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::new("/src/Application.njs".into())),
    inject_accept,
    r#"
        class Component {};
    "#,
    r#"
        class Component {};
        $runtime.accept(module, "/src/Application.njs", {klasses: [Component], dependencies: [], initiate: ""})
    "#
);

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::new("/src/Application.njs".into())),
    inject_multiple_accept,
    r#"
        class Component {};
        class Component2 {};
    "#,
    r#"
        class Component {};
        class Component2 {};
        $runtime.accept(module, "/src/Application.njs", {klasses: [Component, Component2], dependencies: [], initiate: ""})
    "#
);

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::new("/src/Application.njs".into())),
    inject_multiple_imports,
    r#"
        import Nullstack from 'nullstack';
        import Logo from 'nullstack/logo';
        class Component {};
        class Component2 {};
    "#,
    r#"
        import Nullstack from 'nullstack';
        import Logo from 'nullstack/logo';
        class Component {};
        class Component2 {};
        $runtime.accept(module, "/src/Application.njs", {klasses: [Component, Component2], dependencies: ["nullstack", "nullstack/logo"], initiate: ""})"#
);

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::new("/src/Application.njs".into())),
    inject_accept_when_exporting_as_named,
    r#"
        export class Component {};
    "#,
    r#"
        export class Component {};
        $runtime.accept(module, "/src/Application.njs", {klasses: [Component], dependencies: [], initiate: ""})
    "#
);

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::new("/src/Application.njs".into())),
    inject_accept_when_exporting_as_default,
    r#"
        export default class Component {};
    "#,
    r#"
        export default class Component {};
        $runtime.accept(module, "/src/Application.njs", {klasses: [Component], dependencies: [], initiate: ""})
    "#
);

test!(
    syntax(),
    |_| tr(InjectAcceptVisitor::new("/src/Application.njs".into())),
    inject_accept_with_initiate_hash,
    r#"
        class Component {
            static async initiateDep({ lorem }) {
                if (lorem) {
                    return "ipsum"
                }
                return false
            }

            initiate() {
                this.initiateDep({ lorem: true })
            }
        };
    "#,
    r#"
        class Component {
            static async initiateDep({ lorem }) {
                if (lorem) {
                    return "ipsum"
                }
                return false
            }

            initiate() {
                this.initiateDep({ lorem: true })
            }
        };
        $runtime.accept(module, "/src/Application.njs", {klasses: [Component], dependencies: [], initiate: "56c841b87474b72cb79a6a32922f5de8"})
    "#
);
