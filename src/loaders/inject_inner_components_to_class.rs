use std::borrow::BorrowMut;

use swc_common::{chain, Mark};
use swc_core::ecma::{
    ast::*,
    transforms::{base::resolver, testing::test},
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};
use swc_ecma_parser::{EsConfig, Syntax};

pub struct InjectInnerComponentVisitor {
    seeking_idents: bool,
    tags: Vec<Ident>,
    idents: Vec<Ident>,
    local_idents: Vec<Ident>,
}

impl Default for InjectInnerComponentVisitor {
    fn default() -> Self {
        Self {
            seeking_idents: true,
            tags: vec![],
            idents: vec![],
            local_idents: vec![],
        }
    }
}

fn inject_constant(ident: Ident) -> Stmt {
    let mut render_ident = ident.clone();
    render_ident.sym = format!("render{}", ident.sym).into();
    Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: ident.span.clone(),
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![VarDeclarator {
            span: ident.span.clone(),
            name: Pat::Ident(BindingIdent {
                id: ident.clone(),
                type_ann: None,
            }),
            init: Some(Box::new(Expr::Member(MemberExpr {
                span: ident.span.clone(),
                obj: Box::new(Expr::This(ThisExpr {
                    span: ident.span.clone(),
                })),
                prop: MemberProp::Ident(render_ident),
            }))),
            definite: false,
        }],
    })))
}

fn includes_ident(idents: Vec<Ident>, ident: Ident) -> bool {
    idents
        .clone()
        .into_iter()
        .find(|i| i.sym == ident.sym)
        .is_some()
}

impl VisitMut for InjectInnerComponentVisitor {
    noop_visit_mut_type!();

    fn visit_mut_ident(&mut self, n: &mut Ident) {
        if self.seeking_idents {
            self.idents.push(n.clone());
        }
    }

    fn visit_mut_class_method(&mut self, n: &mut ClassMethod) {
        self.seeking_idents = false;
        if let Some(key) = n.key.as_ident() {
            if key.sym.starts_with("render") {
                self.tags.clear();
                self.local_idents.clear();
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
        if !self.seeking_idents {
            if let JSXElementName::Ident(na) = n.name.clone() {
                if na
                    .to_string()
                    .chars()
                    .next()
                    .unwrap_or_else(|| 'n')
                    .is_uppercase()
                {
                    self.tags.push(na);
                }
            }
        }
    }

    fn visit_mut_var_declarator(&mut self, n: &mut VarDeclarator) {
        self.local_idents.push(n.name.clone().ident().unwrap().id)
    }

    fn visit_mut_fn_decl(&mut self, n: &mut FnDecl) {
        self.local_idents.push(n.ident.clone());
    }
}

#[allow(dead_code)]
fn tr() -> impl Fold {
    chain!(
        resolver(Mark::new(), Mark::new(), false),
        as_folder(InjectInnerComponentVisitor::default())
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
    skip_inject_outter_components,
    r#"
        import OutterComponent from 'oc';
        class Component {
            render() {
                return <OutterComponent />;
            }
        }
    "#,
    r#"
        import OutterComponent from 'oc';
        class Component {
            render() {
                return <OutterComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(),
    skip_inject_declared_variables,
    r#"
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(),
    skip_inject_declared_functions,
    r#"
        class Component {
            render() {
                function InnerComponent() { return false };
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                function InnerComponent() { return false };
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(),
    inject_inner_components,
    r#"
        class Component {
            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(),
    inject_inner_components_per_function,
    r#"
        class Component {
            renderSomethingElse() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }

            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            renderSomethingElse() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }

            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);

test!(
    syntax(),
    |_| tr(),
    inject_inner_components_all_functions,
    r#"
        class Component {
            renderSomethingElse() {
                return <InnerComponent />;
            }

            render() {
                return <InnerComponent />;
            }
        }
    "#,
    r#"
        class Component {
            renderSomethingElse() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }

            render() {
                const InnerComponent = this.renderInnerComponent;
                return <InnerComponent />;
            }
        }
    "#
);
