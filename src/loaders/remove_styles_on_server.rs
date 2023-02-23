use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{noop_visit_mut_type, VisitMut},
};

pub struct RemoveStylesVisitor {}

impl Default for RemoveStylesVisitor {
    fn default() -> Self {
        RemoveStylesVisitor {}
    }
}

fn is_style(source: &JsWord) -> bool {
    !source.ends_with(".css") && !source.ends_with(".scss") && !source.ends_with(".sass")
}

impl VisitMut for RemoveStylesVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.body.retain(|item| {
            if let ModuleItem::ModuleDecl(decl) = item {
                if let ModuleDecl::Import(i) = decl {
                    return is_style(&i.src.value);
                }
            }
            true
        })
    }
}
