use swc_core::{
    ecma::{
        ast::*,
        transforms::testing::test,
        visit::{as_folder, noop_visit_mut_type, Fold, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use swc_ecma_quote::quote;

pub struct TransformVisitor {
    imports_nullstack: bool,
    seeking_import: bool,
}

impl Default for TransformVisitor {
    fn default() -> Self {
        TransformVisitor {
            imports_nullstack: false,
            seeking_import: false,
        }
    }
}

impl VisitMut for TransformVisitor {
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

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor::default()))
}

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(TransformVisitor::default())
}

test!(
    Default::default(),
    |_| tr(),
    inject_nullstack,
    r#"import Nutsack from 'nutsack'; class Component extends Nullstack { works = true };"#,
    r#"import Nullstack from 'nullstack';import Nutsack from 'nutsack'; class Component extends Nullstack { works = true };"#
);

test!(
    Default::default(),
    |_| tr(),
    skip_inject_nullstack,
    r#"import Nullstack from 'nullstack'; class Component extends Nullstack { works = true };"#,
    r#"import Nullstack from 'nullstack'; class Component extends Nullstack { works = true };"#
);
