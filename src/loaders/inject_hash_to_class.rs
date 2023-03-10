use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut},
};

pub struct InjectHashVisitor {
    file_path: String,
    is_dev: bool,
}

impl InjectHashVisitor {
    pub fn new(file_path: String, is_dev: bool) -> Self {
        InjectHashVisitor { file_path, is_dev }
    }

    fn hash(&self) -> String {
        if self.is_dev {
            let separator = "__";
            self.file_path
                .replace('/', separator)
                .replace('\\', separator)
                .replace('.', separator)
        } else {
            let digest = md5::compute(self.file_path.clone());
            format!("{:x}", digest)
        }
    }

    fn hash_value(&self) -> Option<Box<Expr>> {
        Option::Some(Box::new(Expr::Lit(Lit::Str(Str {
            value: self.hash().into(),
            raw: None,
            span: DUMMY_SP,
        }))))
    }

    fn hash_prop(&self) -> ClassMember {
        ClassMember::ClassProp(ClassProp {
            span: DUMMY_SP,
            key: PropName::Ident(Ident {
                span: DUMMY_SP,
                sym: "hash".into(),
                optional: false,
            }),
            value: self.hash_value(),
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

    fn visit_mut_class(&mut self, n: &mut Class) {
        n.body.insert(0, self.hash_prop());
    }
}
