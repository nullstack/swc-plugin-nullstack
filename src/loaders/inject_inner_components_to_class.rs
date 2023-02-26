use std::borrow::BorrowMut;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default)]
pub struct InjectInnerComponentVisitor {
    found_idents: bool,
    tags: Vec<Ident>,
    idents: Vec<Ident>,
    local_idents: Vec<Ident>,
}

fn inject_constant(ident: Ident) -> Stmt {
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

fn includes_ident(idents: Vec<Ident>, ident: Ident) -> bool {
    idents.iter().any(|i| i.sym == ident.sym)
}

impl VisitMut for InjectInnerComponentVisitor {
    noop_visit_mut_type!();

    fn visit_mut_ident(&mut self, n: &mut Ident) {
        if !self.found_idents {
            self.idents.push(n.clone());
        }
    }

    fn visit_mut_class_method(&mut self, n: &mut ClassMethod) {
        self.found_idents = true;
        if let Some(key) = n.key.as_ident() {
            if key.sym.starts_with("render") {
                self.tags.clear();
                self.local_idents.clear();
                let params = n.function.params.clone();
                if let Some(ctx) = params.first() {
                    if let Pat::Object(pat) = ctx.pat.clone() {
                        for prop in pat.props.into_iter() {
                            match prop {
                                ObjectPatProp::KeyValue(kv) => {
                                    if let Some(v) = kv.value.ident() {
                                        self.local_idents.push(v.id);
                                    }
                                }
                                ObjectPatProp::Assign(a) => {
                                    self.local_idents.push(a.key);
                                }
                                ObjectPatProp::Rest(_) => {}
                            }
                        }
                    }
                }
                n.visit_mut_children_with(self);
                if !self.tags.is_empty() {
                    for tag in self.tags.clone().into_iter() {
                        if !includes_ident(self.idents.clone(), tag.clone())
                            && !includes_ident(self.local_idents.clone(), tag.clone())
                        {
                            if let Some(body) = n.function.body.borrow_mut() {
                                body.stmts.insert(0, inject_constant(tag));
                            }
                        }
                    }
                }
            }
        }
    }

    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        n.visit_mut_children_with(self);
        if self.found_idents {
            if let JSXElementName::Ident(na) = n.name.clone() {
                if na.to_string().chars().next().unwrap_or('n').is_uppercase() {
                    self.tags.push(na);
                }
            }
        }
    }

    fn visit_mut_var_declarator(&mut self, n: &mut VarDeclarator) {
        if let Some(ident) = n.name.clone().ident() {
            self.local_idents.push(ident.id);
        }
    }

    fn visit_mut_fn_decl(&mut self, n: &mut FnDecl) {
        self.local_idents.push(n.ident.clone());
    }
}
