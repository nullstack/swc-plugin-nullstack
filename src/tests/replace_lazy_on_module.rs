#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::replace_lazy_on_module::ReplaceLazyVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src/Application.njs".into(), true)),
    replace_lazy_when_only_jsx,
    r#"
        import LazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <LazyComponent />
            }
         };
    "#,
    r#"
        const LazyComponent = $runtime.lazy("src__LazyComponent", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <LazyComponent />
            }
         };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src/Application.njs".into(), true)),
    skip_replace_lazy_when_reused,
    r#"
        import LazyComponent from './LazyComponent';
        LazyComponent.reused = true;
        class Component extends Nullstack {
            render() {
                <LazyComponent />
            }
         };
    "#,
    r#"
        import LazyComponent from './LazyComponent';
        LazyComponent.reused = true;
        class Component extends Nullstack {
            render() {
                <LazyComponent />
            }
         };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src/Application.njs".into(), true)),
    replace_lazy_when_closing_jsx,
    r#"
        import LazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <LazyComponent> children </LazyComponent>
            }
         };
    "#,
    r#"
        const LazyComponent = $runtime.lazy("src__LazyComponent", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <LazyComponent> children </LazyComponent>
            }
         };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src/Application.njs".into(), false)),
    replace_lazy_when_only_jsx_in_production,
    r#"
        import LazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <LazyComponent />
            }
         };
    "#,
    r#"
        const LazyComponent = $runtime.lazy("57ad1c52", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <LazyComponent />
            }
         };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new(
        "src/nested/Application.njs".into(),
        true
    )),
    replace_lazy_when_nested_urls,
    r#"
        import LazyComponent from './LazyComponent';
        import OtherComponent from './OtherComponent';
        class Component extends Nullstack {
            render() {
                <div>
                    <OtherComponent />
                    <LazyComponent />
                </div>
            }
         };
    "#,
    r#"
        const LazyComponent = $runtime.lazy("src__nested__LazyComponent", () => import("./LazyComponent"));
        const OtherComponent = $runtime.lazy("src__nested__OtherComponent", () => import("./OtherComponent"));
        class Component extends Nullstack {
            render() {
                <div>
                    <OtherComponent />
                    <LazyComponent />
                </div>
            }
        };
    "#
);
