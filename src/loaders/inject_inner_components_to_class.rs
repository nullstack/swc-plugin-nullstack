use std::borrow::BorrowMut;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};
use tracing::info;

#[derive(Default)]
pub struct InjectInnerComponentVisitor {
    outter_idents: Vec<Ident>,
    inner_idents: Vec<Ident>,
    inner_tags: Vec<Ident>,
    is_inside_class: bool,
    is_inside_method: bool,
    is_inside_tag: bool,
}

fn inject_constant(ident: &Ident) -> Stmt {
    let mut render_ident = ident.clone();
    render_ident.sym = format!("render{}", ident.sym).into();
    Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: ident.span,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![VarDeclarator {
            span: ident.span,
            name: Pat::Ident(BindingIdent {
                id: ident.clone(),
                type_ann: None,
            }),
            init: Some(Box::new(Expr::Member(MemberExpr {
                span: ident.span,
                obj: Box::new(Expr::This(ThisExpr { span: ident.span })),
                prop: MemberProp::Ident(render_ident),
            }))),
            definite: false,
        }],
    })))
}

fn push_if_uppercase(vec: &mut Vec<Ident>, ident: &Ident) {
    if ident.sym.chars().next().unwrap_or_default().is_uppercase() {
        vec.push(ident.clone());
    }
}

impl VisitMut for InjectInnerComponentVisitor {
    noop_visit_mut_type!();

    fn visit_mut_ident(&mut self, n: &mut Ident) {
        if !self.is_inside_class {
            push_if_uppercase(&mut self.outter_idents, &n);
        } else if self.is_inside_tag {
            push_if_uppercase(&mut self.inner_tags, &n);
        } else if self.is_inside_method {
            push_if_uppercase(&mut self.inner_idents, &n);
        }
    }

    fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) {
        self.is_inside_class = true;
        n.visit_mut_children_with(self);
        self.is_inside_class = false;
    }

    fn visit_mut_class_method(&mut self, n: &mut ClassMethod) {
        if let Some(key) = n.key.as_ident() {
            if key.sym.starts_with("render") {
                self.is_inside_method = true;
                n.function.visit_mut_children_with(self);
                self.is_inside_method = false;
                for inner_tag in self.inner_tags.iter() {
                    info!(
                        "\n\n IN {:#?} - OUT {:#?} - TAGS {:#?}\n\n",
                        self.inner_idents, self.outter_idents, self.inner_tags
                    );
                    if !self.outter_idents.iter().any(|i| i.sym == inner_tag.sym)
                        && !self.inner_idents.iter().any(|i| i.sym == inner_tag.sym)
                    {
                        if let Some(body) = n.function.body.borrow_mut() {
                            body.stmts.insert(0, inject_constant(inner_tag));
                        }
                    }
                }
                self.inner_idents.clear();
                self.inner_tags.clear();
            }
        }
    }

    fn visit_mut_key_value_pat_prop(&mut self, n: &mut KeyValuePatProp) {
        n.value.visit_mut_children_with(self);
    }

    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        if self.is_inside_method {
            self.is_inside_tag = true;
            n.visit_mut_children_with(self);
            self.is_inside_tag = false;
        }
    }

    fn visit_mut_jsx_closing_element(&mut self, _: &mut JSXClosingElement) {}
}
