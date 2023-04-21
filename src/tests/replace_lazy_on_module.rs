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
                <LazyComponent route="/" />
            }
         };
    "#,
    r#"
        const LazyComponent = $runtime.lazy("src__LazyComponent", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/" />
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
                <LazyComponent route="/" />
            }
         };
    "#,
    r#"
        import LazyComponent from './LazyComponent';
        LazyComponent.reused = true;
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/" />
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
                <LazyComponent route="/"> children </LazyComponent>
            }
         };
    "#,
    r#"
        const LazyComponent = $runtime.lazy("src__LazyComponent", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/"> children </LazyComponent>
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
                <LazyComponent route="/" />
            }
         };
    "#,
    r#"
        const LazyComponent = $runtime.lazy("c1a38acc", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/" />
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
                    <OtherComponent route="/" />
                    <LazyComponent route="/" />
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
                    <OtherComponent route="/" />
                    <LazyComponent route="/" />
                </div>
            }
        };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src\\Application.njs".into(), true)),
    replace_lazy_on_windows,
    r#"
        import LazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/" />
            }
         };
    "#,
    r#"
        const LazyComponent = $runtime.lazy("src__LazyComponent", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/" />
            }
         };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src/Application.njs".into(), true)),
    replace_lazy_when_skiping_non_jsx_import,
    r#"
        import Nullstack from "nullstack";
        import LazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/" />
            }
         };
    "#,
    r#"
        import Nullstack from "nullstack";
        const LazyComponent = $runtime.lazy("src__LazyComponent", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/" />
            }
         };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src/Application.njs".into(), true)),
    replace_lazy_when_has_named_import,
    r#"
        import Nullstack from "nullstack";
        import { helper } from "./helpers";
        import LazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/" />
            }
         };
    "#,
    r#"
        import Nullstack from "nullstack";
        import { helper } from "./helpers";
        const LazyComponent = $runtime.lazy("src__LazyComponent", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <LazyComponent route="/" />
            }
         };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src/Application.njs".into(), true)),
    skip_replace_lazy_when_lowercase,
    r#"
        import lazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <lazyComponent route="/" />
            }
         };
    "#,
    r#"
        import lazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <lazyComponent route="/" />
            }
         };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src/Application.njs".into(), true)),
    skip_replace_lazy_when_not_tag_name,
    r#"
        import LazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <button route="/" icon={LazyComponent} />
            }
         };
    "#,
    r#"
        import LazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <button route="/" icon={LazyComponent} />
            }
         };
    "#
);

test!(
    syntax(),
    |_| tr(ReplaceLazyVisitor::new("src/Application.njs".into(), true)),
    replace_lazy_when_not_route,
    r#"
        import SyncComponent from './SyncComponent';
        import LazyComponent from './LazyComponent';
        class Component extends Nullstack {
            render() {
                <div>
                    <SyncComponent />
                    <LazyComponent route="/" />
                </div>
            }
         };
    "#,
    r#"
        import SyncComponent from './SyncComponent';
        const LazyComponent = $runtime.lazy("src__LazyComponent", () => import("./LazyComponent"));
        class Component extends Nullstack {
            render() {
                <div>
                    <SyncComponent />
                    <LazyComponent route="/" />
                </div>
            }
         };
    "#
);
