use std::borrow::Borrow;

use swc_common::{chain, Mark};
use swc_core::ecma::{
    ast::*,
    transforms::{base::resolver, testing::test},
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};
use swc_ecma_parser::{EsConfig, Syntax};

pub struct RemoveUnusedVisitor {
    reject_list: Vec<Ident>,
    ident_list: Vec<Ident>,
}

impl Default for RemoveUnusedVisitor {
    fn default() -> Self {
        RemoveUnusedVisitor {
            reject_list: vec![],
            ident_list: vec![],
        }
    }
}

fn specifier_ident(specifier: ImportSpecifier) -> Ident {
    match specifier {
        ImportSpecifier::Default(s) => s.local,
        ImportSpecifier::Named(s) => s.local,
        ImportSpecifier::Namespace(s) => s.local,
    }
}

fn is_equal(a: &Ident, b: &Ident) -> bool {
    if a.sym != b.sym {
        return false;
    }
    a.span.ctxt.outer() == b.span.ctxt.outer()
        && a.span.hi.0 == b.span.hi.0
        && a.span.lo.0 == b.span.lo.0
}

fn allow_list(list: Vec<Ident>, reject: Vec<Ident>) -> Vec<Ident> {
    let mut allow_list = list.clone();
    allow_list.retain(|item| {
        reject
            .clone()
            .into_iter()
            .find(|i| is_equal(item.borrow(), i.borrow()))
            .is_none()
    });
    allow_list
}

fn should_retain(allow_list: Vec<Ident>, item: Ident) -> bool {
    allow_list
        .into_iter()
        .find(|i| item.to_id() == i.to_id())
        .is_some()
}

impl VisitMut for RemoveUnusedVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.visit_mut_children_with(self);
        let allow = allow_list(self.ident_list.clone(), self.reject_list.clone());
        n.body.retain(|item| match item.clone() {
            ModuleItem::ModuleDecl(ModuleDecl::Import(a)) => {
                for specifier in a.specifiers.clone().into_iter() {
                    let ident = specifier_ident(specifier);
                    return should_retain(allow.clone(), ident);
                }
                true
            }
            _ => true,
        });
    }

    fn visit_mut_import_specifier(&mut self, n: &mut ImportSpecifier) {
        n.visit_mut_children_with(self);
        let ident = specifier_ident(n.clone());
        self.reject_list.push(ident);
    }

    fn visit_mut_class_member(&mut self, n: &mut ClassMember) {
        n.visit_mut_children_with(self);
        if let ClassMember::Method(m) = n {
            if let Some(ident) = m.key.clone().ident() {
                self.reject_list.push(ident);
            }
        }
    }

    fn visit_mut_member_expr(&mut self, n: &mut MemberExpr) {
        n.visit_mut_children_with(self);
        if let Some(ident) = n.prop.clone().ident() {
            self.reject_list.push(ident);
        }
    }

    fn visit_mut_ident(&mut self, n: &mut Ident) {
        n.visit_mut_children_with(self);
        self.ident_list.push(n.clone());
    }

    fn visit_mut_var_declarator(&mut self, n: &mut VarDeclarator) {
        n.visit_mut_children_with(self);
        if let Some(ident) = n.name.clone().ident() {
            self.reject_list.push(ident.id);
        }
    }
}

#[allow(dead_code)]
fn tr() -> impl Fold {
    chain!(
        resolver(Mark::new(), Mark::new(), false),
        as_folder(RemoveUnusedVisitor::default())
    )
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
    remove_unused_import,
    r#"import fs from 'fs'; class Component { };"#,
    r#"class Component { };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_member_conflict,
    r#"import fs from 'fs'; class Component { fs() {} };"#,
    r#"class Component { fs() {} };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_this_conflict,
    r#"import fs from 'fs'; class Component { prepare() { this.fs = 69 } };"#,
    r#"class Component { prepare() { this.fs = 69 } };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_redeclared,
    r#"import fs from 'fs'; class Component { prepare() { const fs = 0 } };"#,
    r#"class Component { prepare() { const fs = 0 } };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_redeclared_and_reused,
    r#"import fs from 'fs'; class Component { prepare() { let fs = 0; fs = 1; } };"#,
    r#"class Component { prepare() { let fs = 0; fs = 1; } };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_used_as_key,
    r#"import key from 'key'; class Component { prepare() { const obj = {}; return obj[key] } };"#,
    r#"import key from 'key'; class Component { prepare() { const obj = {}; return obj[key] } };"#
);

test!(
    syntax(),
    |_| tr(),
    keep_used_when_jsx_conflict,
    r#"import Tag from 'tag'; class Component { render() { return <Tag />} };"#,
    r#"import Tag from 'tag'; class Component { render() { return <Tag />} };"#
);

test!(
    syntax(),
    |_| tr(),
    keep_used_imports,
    r#"import fs from 'fs'; class Component { fs3() { this.fs2 = true; fs4.readFileSync(); const fs5 = 0; fs.existsSync("yourmom.txt") } };"#,
    r#"import fs from 'fs'; class Component { fs3() { this.fs2 = true; fs4.readFileSync(); const fs5 = 0; fs.existsSync("yourmom.txt") } };"#
);
