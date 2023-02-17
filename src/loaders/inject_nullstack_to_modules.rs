use swc_core::ecma::{
    ast::*,
    transforms::testing::test,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_quote::quote;

pub struct InjectNullstackVisitor {
    imports_nullstack: bool,
    seeking_import: bool,
}

impl Default for InjectNullstackVisitor {
    fn default() -> Self {
        InjectNullstackVisitor {
            imports_nullstack: false,
            seeking_import: false,
        }
    }
}

impl VisitMut for InjectNullstackVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        let old_seeking_import = self.seeking_import;
        self.seeking_import = true;
        n.visit_mut_children_with(self);
        if !self.imports_nullstack {
            let nullstack_injection = quote!("import Nullstack from 'nullstack';" as ModuleItem);
            n.body.insert(0, nullstack_injection);
        }
        self.seeking_import = old_seeking_import;
    }

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        if n.src.value.eq_str_ignore_ascii_case("nullstack") {
            self.imports_nullstack = true;
        }
    }
}

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(InjectNullstackVisitor::default())
}

#[allow(dead_code)]
fn syntax() -> Syntax {
    let mut config = EsConfig::default();
    config.jsx = true;
    Syntax::Es(config)
}

test!(
    Default::default(),
    |_| tr(),
    inject_nullstack,
    r#"import Nutsack from 'nutsack'; class Component extends Nullstack { works = true };"#,
    r#"import Nullstack from 'nullstack'; import Nutsack from 'nutsack'; class Component extends Nullstack { works = true };"#
);

test!(
    Default::default(),
    |_| tr(),
    skip_inject_nullstack,
    r#"import Nullstack from 'nullstack'; class Component extends Nullstack { works = true };"#,
    r#"import Nullstack from 'nullstack'; class Component extends Nullstack { works = true };"#
);
