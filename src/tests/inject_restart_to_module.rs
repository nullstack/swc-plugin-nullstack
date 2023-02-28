#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::inject_restart_to_module::InjectRestartVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(InjectRestartVisitor::default()),
    inject_restart,
    r#"
        import Nullstack from 'nullstack';
        import Application from './src/Application';
        const context = Nullstack.start(Application);
    "#,
    r#"
        import Nullstack from 'nullstack';
        import Application from './src/Application';
        const context = Nullstack.start(Application);
        $runtime.restart(module, "./src/Application");
    "#
);

test!(
    syntax(),
    |_| tr(InjectRestartVisitor::default()),
    inject_restart_unusual_nullstack_ident,
    r#"
        import Nadegas from 'nullstack';
        import Application from './src/Application';
        const context = Nadegas.start(Application);
    "#,
    r#"
        import Nadegas from 'nullstack';
        import Application from './src/Application';
        const context = Nadegas.start(Application);
        $runtime.restart(module, "./src/Application");
    "#
);

test!(
    syntax(),
    |_| tr(InjectRestartVisitor::default()),
    inject_restart_unusual_application_ident,
    r#"
        import Nullstack from 'nullstack';
        import { Blog } from './src/Blog';
        const context = Nullstack.start(Blog);
    "#,
    r#"
        import Nullstack from 'nullstack';
        import { Blog } from './src/Blog';
        const context = Nullstack.start(Blog);
        $runtime.restart(module, "./src/Blog");
    "#
);
