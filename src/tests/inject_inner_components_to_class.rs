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
    skip_inject_import_default,
    r#"
        import InnerComponent from 'inner'
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        import InnerComponent from 'inner'
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
    skip_inject_import_named,
    r#"
        import { InnerComponent } from 'inner'
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        import { InnerComponent } from 'inner'
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
    skip_inject_import_named_as_local,
    r#"
        import { other as InnerComponent } from 'inner'
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        import { other as InnerComponent } from 'inner'
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
    skip_inject_import_namespace,
    r#"
        import * as InnerComponent from 'inner'
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        import * as InnerComponent from 'inner'
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

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_for_regular_tags,
    r#"
        class Component {
            render() {
                return <div />;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                return <div />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_components_with_nested_tags,
    r#"
        class Component {
            render() {
                return <InnerWrapper><InnerComponent /></InnerWrapper>;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                const InnerWrapper = this.renderInnerWrapper;
                return <InnerWrapper><InnerComponent /></InnerWrapper>;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_non_jsx_constants,
    r#"
        class Component {
            render() {
                return <InnerComponent text={String(1)} />;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent text={String(1)} />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_class_declr,
    r#"
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
        class Component2 {
            render() {
                return <Component />;
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
        class Component2 {
            render() {
                return <Component />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_component_without_repeating,
    r#"
        class Component {
            render() {
                return <div><InnerComponent /><InnerComponent /></div>;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <div><InnerComponent /><InnerComponent /></div>;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    injects_inner_component_when_exporting_as_named,
    r#"
        export class Component {
            render() {
                return <div><InnerComponent /><InnerComponent /></div>;
            }
        }
    "#,
    r#"
        export class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <div><InnerComponent /><InnerComponent /></div>;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    injects_inner_component_when_exporting_as_default,
    r#"
        export default class Component {
            render() {
                return <div><InnerComponent /><InnerComponent /></div>;
            }
        }
    "#,
    r#"
        export default class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <div><InnerComponent /><InnerComponent /></div>;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_components_when_mixed_scopes,
    r#"
        import OutterComponent from 'outter'
        class Component {
            render() {
                return <><OutterComponent /><InnerComponent /><OutterComponent /></>
            }
        }
    "#,
    r#"
        import OutterComponent from 'outter'
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent
                return <><OutterComponent /><InnerComponent /><OutterComponent /></>
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_components_when_declare_conflict,
    r#"
        declare function Link(context: HomeLinkProps): NullstackNode
        class Component {
            render() {
                return <><Link /></>
            }
        }
    "#,
    r#"
        class Component {
            render() {
                const Link = this.renderLink
                return <><Link /></>
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_inner_components_when_define_top_function,
    r#"
        function InnerComponent() {}
        class Component {
            render() {
                return <><InnerComponent /></>
            }
        }
    "#,
    r#"
        function InnerComponent() {}
        class Component {
            render() {
                return <><InnerComponent /></>
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    skip_inject_inner_components_when_define_top_function_constant,
    r#"
        const InnerComponent = () => {}
        class Component {
            render() {
                return <><InnerComponent /></>
            }
        }
    "#,
    r#"
        const InnerComponent = () => {}
        class Component {
            render() {
                return <><InnerComponent /></>
            }
        }
    "#
);


test!(
    syntax(),
    |_| tr(InjectInnerComponentVisitor::default()),
    inject_inner_components_by_param,
    r#"
        class ExternalComponent {
            render({test}) {
                return <div>{test}</div>;
            }
        }
        class Component {
            renderInnerComponent() {
                return <span />;
            }
            renderAnotherInnerComponent() {
                return <InnerComponent />;
            }

            render() {
                return <ExternalComponent tt="here" test={<AnotherInnerComponent />} />;
            }
        }
    "#,
    r#"
        class ExternalComponent {
            render({test}) {
                return <div>{test}</div>;
            }
        }
        class Component {
            renderInnerComponent() {
                return <span />;
            }
            renderAnotherInnerComponent() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }

            render() {
                const AnotherInnerComponent = this.renderAnotherInnerComponent;
                return <ExternalComponent tt="here" test={<AnotherInnerComponent/>} />;
            }
        }
    "#
);