use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{noop_visit_mut_type, VisitMut},
};

use super::{combine_hash, hash};

pub struct InjectHashVisitor {
    file_hash: String,
    is_dev: bool,
}

impl InjectHashVisitor {
    pub fn new(file_path: String, is_dev: bool) -> Self {
        InjectHashVisitor {
            file_hash: hash(&file_path, is_dev),
            is_dev,
        }
    }

    fn hash_prop(&self, class_hash: JsWord) -> ClassMember {
        ClassMember::ClassProp(ClassProp {
            span: DUMMY_SP,
            key: PropName::Ident(Ident {
                span: DUMMY_SP,
                sym: "hash".into(),
                optional: false,
            }),
            value: Option::Some(Box::new(Expr::Lit(Lit::Str(Str {
                value: class_hash,
                raw: None,
                span: DUMMY_SP,
            })))),
            type_ann: None,
            is_static: true,
            decorators: vec![],
            accessibility: None,
            is_abstract: false,
            is_optional: false,
            is_override: false,
            readonly: false,
            declare: false,
            definite: false,
        })
    }
}

impl VisitMut for InjectHashVisitor {
    noop_visit_mut_type!();

    fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) {
        let class_hash = hash(&n.ident.sym, self.is_dev);
        let combined_hash = combine_hash(&self.file_hash, &class_hash, self.is_dev);
        n.class.body.insert(0, self.hash_prop(combined_hash.into()));
    }
}
