use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default)]
pub struct InjectTranspilerVisitor {
    has_runtime: bool,
}

fn runtime_import() -> ModuleItem {
    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        src: Box::new(Str::from("nullstack/runtime")),
        type_only: false,
        asserts: None,
        specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
            span: DUMMY_SP,
            local: Ident {
                sym: "$runtime".into(),
                span: DUMMY_SP,
                optional: false,
            },
        })],
    }))
}

impl VisitMut for InjectTranspilerVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.visit_mut_children_with(self);
        if !self.has_runtime {
            n.body.insert(0, runtime_import());
        }
    }

    fn visit_mut_import_default_specifier(&mut self,n: &mut ImportDefaultSpecifier) {
        if n.local.sym.eq("$runtime") {
            self.has_runtime = true
        }
    }
}
