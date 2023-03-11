use std::{collections::HashMap, vec};

use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

#[derive(Default, Debug)]
pub struct InjectAcceptVisitor {
    class_names: Vec<JsWord>,
    import_paths: Vec<JsWord>,
    file_path: String,
    inside_initiate: bool,
    initiate_dependencies: Vec<JsWord>,
    server_function_bytes: HashMap<JsWord, Vec<u8>>,
    current_server_function: Option<JsWord>,
}

impl InjectAcceptVisitor {
    pub fn new(file_path: String) -> Self {
        InjectAcceptVisitor {
            file_path,
            ..Default::default()
        }
    }
}

fn runtime_accept(
    class_names: &[JsWord],
    import_paths: &[JsWord],
    file_path: &str,
    initiate: &str,
) -> ModuleItem {
    ModuleItem::Stmt(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "$runtime".into(),
                    optional: false,
                })),
                prop: MemberProp::Ident(Ident {
                    span: DUMMY_SP,
                    sym: "accept".into(),
                    optional: false,
                }),
            }))),
            args: vec![
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Ident(Ident {
                        span: DUMMY_SP,
                        sym: "module".into(),
                        optional: false,
                    })),
                },
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: file_path.into(),
                        raw: None,
                    }))),
                },
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Object(ObjectLit {
                        span: DUMMY_SP,
                        props: vec![
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(Ident {
                                    span: DUMMY_SP,
                                    sym: "klasses".into(),
                                    optional: false,
                                }),
                                value: Box::new(Expr::Array(ArrayLit {
                                    span: DUMMY_SP,
                                    elems: class_names
                                        .iter()
                                        .map(|class_name| {
                                            Some(ExprOrSpread {
                                                spread: None,
                                                expr: Box::new(Expr::Ident(Ident {
                                                    span: DUMMY_SP,
                                                    sym: class_name.clone(),
                                                    optional: false,
                                                })),
                                            })
                                        })
                                        .collect(),
                                })),
                            }))),
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(Ident {
                                    span: DUMMY_SP,
                                    sym: "dependencies".into(),
                                    optional: false,
                                }),
                                value: Box::new(Expr::Array(ArrayLit {
                                    span: DUMMY_SP,
                                    elems: import_paths
                                        .iter()
                                        .map(|import_path| {
                                            Some(ExprOrSpread {
                                                spread: None,
                                                expr: Box::new(Expr::Lit(Lit::Str(Str {
                                                    span: DUMMY_SP,
                                                    value: import_path.clone(),
                                                    raw: None,
                                                }))),
                                            })
                                        })
                                        .collect(),
                                })),
                            }))),
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(Ident {
                                    span: DUMMY_SP,
                                    sym: "initiate".into(),
                                    optional: false,
                                }),
                                value: Box::new(Expr::Lit(Lit::Str(Str {
                                    span: DUMMY_SP,
                                    value: initiate.into(),
                                    raw: None,
                                }))),
                            }))),
                        ],
                    })),
                },
            ],
            type_args: None,
        })),
    }))
}

impl VisitMut for InjectAcceptVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.visit_mut_children_with(self);
        let mut initiate_hash = vec![];
        for (key, value) in self.server_function_bytes.iter_mut() {
            if self.initiate_dependencies.iter().any(|dep| dep == key) {
                initiate_hash.append(value);
            }
        }
        let hash = if initiate_hash.is_empty() {
            String::new()
        } else {
            format!("{:?}", md5::compute(initiate_hash.clone()))
        };
        n.body.push(runtime_accept(
            &self.class_names,
            &self.import_paths,
            &self.file_path,
            &hash,
        ));
    }

    fn visit_mut_class_member(&mut self, n: &mut ClassMember) {
        n.visit_mut_children_with(self);
        if let ClassMember::Method(m) = n {
            if let Some(ident) = m.key.clone().ident() {
                if m.is_static && m.function.is_async {
                    self.server_function_bytes.insert(ident.sym.clone(), vec![]);
                    self.current_server_function = Some(ident.sym);
                    n.visit_mut_children_with(self);
                    self.current_server_function = None;
                } else if ident.sym.eq("initiate") {
                    self.inside_initiate = true;
                    n.visit_mut_children_with(self);
                    self.inside_initiate = false;
                }
            }
        }
    }

    fn visit_mut_ident(&mut self, n: &mut Ident) {
        if self.inside_initiate {
            self.initiate_dependencies.push(n.sym.clone())
        } else if let Some(server_function) = &self.current_server_function {
            if let Some(identities) = self.server_function_bytes.get_mut(server_function) {
                identities.append(&mut n.sym.to_string().into_bytes());
            }
        }
    }

    fn visit_mut_expr(&mut self, n: &mut Expr) {
        if let Some(server_function) = &self.current_server_function {
            if let Some(identities) = self.server_function_bytes.get_mut(server_function) {
                match n {
                    Expr::This(_) => identities.push(1),
                    Expr::Array(_) => identities.push(2),
                    Expr::Object(_) => identities.push(3),
                    Expr::Fn(_) => identities.push(4),
                    Expr::Unary(_) => identities.push(5),
                    Expr::Update(_) => identities.push(6),
                    Expr::Bin(_) => identities.push(7),
                    Expr::Assign(_) => identities.push(8),
                    Expr::Member(_) => identities.push(9),
                    Expr::SuperProp(_) => identities.push(10),
                    Expr::Cond(_) => identities.push(11),
                    Expr::Call(_) => identities.push(12),
                    Expr::New(_) => identities.push(13),
                    Expr::Seq(_) => identities.push(14),
                    Expr::Ident(_) => identities.push(15),
                    Expr::Lit(_) => identities.push(16),
                    Expr::Tpl(_) => identities.push(17),
                    Expr::TaggedTpl(_) => identities.push(18),
                    Expr::Arrow(_) => identities.push(19),
                    Expr::Class(_) => identities.push(20),
                    Expr::Yield(_) => identities.push(21),
                    Expr::MetaProp(_) => identities.push(22),
                    Expr::Await(_) => identities.push(23),
                    Expr::Paren(_) => identities.push(24),
                    Expr::JSXMember(_) => identities.push(25),
                    Expr::JSXNamespacedName(_) => identities.push(26),
                    Expr::JSXEmpty(_) => identities.push(27),
                    Expr::JSXElement(_) => identities.push(28),
                    Expr::JSXFragment(_) => identities.push(29),
                    Expr::TsTypeAssertion(_) => identities.push(30),
                    Expr::TsConstAssertion(_) => identities.push(31),
                    Expr::TsNonNull(_) => identities.push(32),
                    Expr::TsAs(_) => identities.push(33),
                    Expr::TsInstantiation(_) => identities.push(34),
                    Expr::TsSatisfies(_) => identities.push(35),
                    Expr::PrivateName(_) => identities.push(36),
                    Expr::OptChain(_) => identities.push(37),
                    Expr::Invalid(_) => identities.push(38),
                }
            }
        }
        n.visit_mut_children_with(self);
    }

    fn visit_mut_lit(&mut self, n: &mut Lit) {
        if let Some(server_function) = &self.current_server_function {
            if let Some(identities) = self.server_function_bytes.get_mut(server_function) {
                match n {
                    Lit::Str(s) => identities.append(&mut s.value.to_string().into_bytes()),
                    Lit::Bool(b) => {
                        if b.value {
                            identities.push(1);
                        } else {
                            identities.push(0);
                        }
                    }
                    Lit::Null(_) => {
                        identities.push(0);
                    }
                    Lit::Num(num) => identities.append(&mut num.value.to_string().into_bytes()),
                    Lit::BigInt(bi) => identities.append(&mut bi.value.to_string().into_bytes()),
                    Lit::Regex(r) => identities.append(&mut r.exp.to_string().into_bytes()),
                    _ => identities.push(0),
                }
            }
        }
    }

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        self.import_paths.push(n.src.value.clone());
    }

    fn visit_mut_class_expr(&mut self, n: &mut ClassExpr) {
        if let Some(ident) = &n.ident {
            self.class_names.push(ident.sym.clone());
            n.visit_mut_children_with(self);
        }
    }

    fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) {
        self.class_names.push(n.ident.sym.clone());
        n.visit_mut_children_with(self);
    }
}
