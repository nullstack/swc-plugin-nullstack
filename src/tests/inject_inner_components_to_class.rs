#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::inject_inner_components_to_class::InjectInnerComponentVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_outter_components,
    r#"
        import OutterComponent from 'oc';
        class Component {
            render() {
                return <OutterComponent />;
            }
        }
    "#,
    r#"
        import OutterComponent from 'oc';
        class Component {
            render() {
                return <OutterComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_declared_variables,
    r#"
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_destructured_renamed_args,
    r#"
        class Component {
            render({ component: InnerComponent }) {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            render({ component: InnerComponent }) {
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_destructured_args,
    r#"
        class Component {
            render({ InnerComponent }) {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            render({ InnerComponent }) {
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_declared_functions,
    r#"
        class Component {
            render() {
                function InnerComponent() { return false };
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                function InnerComponent() { return false };
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_components,
    r#"
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_components_per_function,
    r#"
        class Component {
            renderSomethingElse() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }

            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            renderSomethingElse() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }

            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_components_all_functions,
    r#"
        class Component {
            renderSomethingElse() {
                return <InnerComponent />;
            }

            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            renderSomethingElse() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }

            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inner_components_with_top_level_const,
    r#"
        const InnerComponent = {};
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        const InnerComponent = {};
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_components_with_top_renamed_destructured_const,
    r#"
        const {InnerComponent: Nope} = {};
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        const {InnerComponent: Nope} = {};
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_components_with_top_renamed_destructured_params,
    r#"
        class Component {
            render({InnerComponent: Nope}) {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            render({InnerComponent: Nope}) {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);
