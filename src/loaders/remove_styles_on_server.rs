use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    transforms::testing::test,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut},
};
use swc_ecma_parser::{EsConfig, Syntax};

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

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(RemoveStylesVisitor::default())
}

#[allow(dead_code)]
fn syntax() -> Syntax {
    let mut config = EsConfig::default();
    config.jsx = true;
    Syntax::Es(config)
}

test!(
    syntax(),
    |_| tr(),
    remove_css,
    r#"import "styles.css"; class Component { };"#,
    r#"class Component { };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_scss,
    r#"import "styles.scss"; class Component { };"#,
    r#"class Component { };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_sass,
    r#"import "styles.sass"; class Component { };"#,
    r#"class Component { };"#
);
