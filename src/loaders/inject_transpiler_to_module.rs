use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

use super::helpers::transpiler_ident;

#[derive(Default)]
pub struct InjectTranspilerVisitor {
    has_import: bool,
}

fn import_specifier() -> ImportSpecifier {
    ImportSpecifier::Named(ImportNamedSpecifier {
        span: DUMMY_SP,
        local: transpiler_ident(),
        imported: None,
        is_type_only: false,
    })
}

impl VisitMut for InjectTranspilerVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.visit_mut_children_with(self);
        if !self.has_import {
            n.body.insert(
                0,
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    span: n.span,
                    specifiers: vec![import_specifier()],
                    src: Box::new("nullstack".into()),
                    type_only: false,
                    asserts: None,
                })),
            );
        }
    }

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        if !self.has_import && n.src.value.eq_ignore_ascii_case(&"nullstack".into()) {
            if let Some(ImportSpecifier::Default(_)) = n.specifiers.first() {
                self.has_import = true;
                n.specifiers.push(import_specifier());
            }
        }
    }
}
