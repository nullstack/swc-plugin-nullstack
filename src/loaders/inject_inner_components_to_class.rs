use std::{borrow::BorrowMut, collections::HashMap};
use swc_common::Span;
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default)]
pub struct InjectInnerComponentVisitor {
    outter_idents: Vec<JsWord>,
    inner_idents: Vec<JsWord>,
    inner_tags: HashMap<JsWord, Span>,
    is_inside_class: bool,
    is_inside_method: bool,
    is_inside_tag: bool,
}

fn inject_constant(constant_name: &JsWord, span: &Span) -> Stmt {
    let function_name = format!("render{}", constant_name);
    Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: *span,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![VarDeclarator {
            span: *span,
            name: Pat::Ident(BindingIdent {
                id: Ident {
                    span: *span,
                    sym: constant_name.clone(),
                    optional: false,
                },
                type_ann: None,
            }),
            init: Some(Box::new(Expr::Member(MemberExpr {
                span: *span,
                obj: Box::new(Expr::This(ThisExpr { span: *span })),
                prop: MemberProp::Ident(Ident {
                    span: *span,
                    sym: JsWord::from(function_name),
                    optional: false,
                }),
            }))),
            definite: false,
        }],
    })))
}

fn push_if_uppercase(vec: &mut Vec<JsWord>, sym: &JsWord) {
    if sym.chars().next().unwrap_or_default().is_uppercase() && !vec.iter().any(|s| *s == *sym) {
        vec.push(sym.clone());
    }
}

impl VisitMut for InjectInnerComponentVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.body.retain(|item| {
            if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(decl))) = item {
                return !decl.declare
            }
            true
        });
        n.visit_mut_children_with(self);
    }

    fn visit_mut_module_item(&mut self, n: &mut ModuleItem) {
        match n {
            ModuleItem::ModuleDecl(decl) => match decl {
                ModuleDecl::Import(i) => {
                    for specifier in i.specifiers.iter() {
                        match specifier {
                            ImportSpecifier::Named(s) => {
                                push_if_uppercase(&mut self.outter_idents, &s.local.sym)
                            }
                            ImportSpecifier::Default(s) => {
                                push_if_uppercase(&mut self.outter_idents, &s.local.sym)
                            }
                            ImportSpecifier::Namespace(s) => {
                                push_if_uppercase(&mut self.outter_idents, &s.local.sym)
                            }
                        }
                    }
                }
                other => other.visit_mut_children_with(self),
            },
            ModuleItem::Stmt(stmt) => match stmt {
                Stmt::Decl(d) => match d {
                    Decl::Var(v) => {
                        for decl in v.decls.iter() {
                            if let Pat::Ident(ident) = &decl.name {
                                push_if_uppercase(&mut self.outter_idents, &ident.sym)
                            }
                        }
                    }
                    Decl::Fn(f) => {
                        if !f.declare {
                            push_if_uppercase(&mut self.outter_idents, &f.ident.sym)
                        } 
                    }
                    other => other.visit_mut_children_with(self),
                },
                other => other.visit_mut_children_with(self),
            },
        }
    }

    fn visit_mut_ident(&mut self, n: &mut Ident) {
        if !self.is_inside_class {
            push_if_uppercase(&mut self.outter_idents, &n.sym);
        } else if self.is_inside_tag && !self.outter_idents.iter().any(|s| *s == n.sym)
                && !self.inner_idents.iter().any(|s| *s == n.sym)
                && n.sym.chars().next().unwrap_or_default().is_uppercase() {
                    self.inner_tags.insert(n.sym.clone(), n.span);
        } else if self.is_inside_method {
            push_if_uppercase(&mut self.inner_idents, &n.sym);
        }
    }

    fn visit_mut_class_expr(&mut self, n: &mut ClassExpr) {
        if let Some(ident) = &n.ident {
            push_if_uppercase(&mut self.outter_idents, &ident.sym);
        }
        self.is_inside_class = true;
        n.visit_mut_children_with(self);
        self.is_inside_class = false;
    }

    fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) {
        push_if_uppercase(&mut self.outter_idents, &n.ident.sym);
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
                for (inner_tag, inner_span) in self.inner_tags.iter() {
                    if let Some(body) = n.function.body.borrow_mut() {
                        body.stmts.insert(0, inject_constant(inner_tag, inner_span));
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
            n.name.visit_mut_children_with(self);
            self.is_inside_tag = false;
            n.visit_mut_children_with(self);
        }
    }

    fn visit_mut_jsx_closing_element(&mut self, _: &mut JSXClosingElement) {}
}
