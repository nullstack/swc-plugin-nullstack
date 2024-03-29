use std::path::PathBuf;
use swc_common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

use super::hash;

fn lazy_import(constant_name: &JsWord, file_hash: &JsWord, import_path: &JsWord) -> ModuleItem {
    ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![VarDeclarator {
            span: DUMMY_SP,
            name: Pat::Ident(BindingIdent {
                id: Ident {
                    span: DUMMY_SP,
                    sym: constant_name.clone(),
                    optional: false,
                },
                type_ann: None,
            }),
            init: Some(Box::new(Expr::Call(CallExpr {
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
                        sym: "lazy".into(),
                        optional: false,
                    }),
                }))),
                args: vec![
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            span: DUMMY_SP,
                            value: file_hash.clone(),
                            raw: None,
                        }))),
                    },
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Arrow(ArrowExpr {
                            span: DUMMY_SP,
                            params: vec![],
                            body: BlockStmtOrExpr::Expr(Box::new(Expr::Call(CallExpr {
                                span: DUMMY_SP,
                                callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                                    span: DUMMY_SP,
                                    sym: "import".into(),
                                    optional: false,
                                }))),
                                args: vec![ExprOrSpread {
                                    spread: None,
                                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                                        span: DUMMY_SP,
                                        value: import_path.clone(),
                                        raw: None,
                                    }))),
                                }],
                                type_args: None,
                            }))),
                            is_async: false,
                            is_generator: false,
                            type_params: None,
                            return_type: None,
                        })),
                    },
                ],
                type_args: None,
            }))),
            definite: false,
        }],
    }))))
}

#[derive(Default, Debug)]
pub struct ReplaceLazyVisitor {
    module_statements: Vec<Option<JsWord>>,
    is_dev: bool,
    file_path: String,
    completed_lookup: bool,
}

// impl ReplaceLazyVisitor {
//     pub fn new(file_path: String, is_dev: bool) -> Self {
//         ReplaceLazyVisitor {
//             module_statements: vec![],
//             is_dev,
//             file_path,
//             completed_lookup: false,
//         }
//     }
// }

fn resolve_path<'a>(current_path: &'a str, target_path: &'a str) -> String {
    let mut resolved_path = PathBuf::from(current_path.replace('\\', "/"));
    for component in target_path.replace('\\', "/").split('/') {
        if component == "." {
            resolved_path.pop();
        } else if component == ".." {
            resolved_path.pop();
            resolved_path.pop();
        } else {
            resolved_path.push(component);
        }
    }
    resolved_path.display().to_string()
}

impl VisitMut for ReplaceLazyVisitor {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        for item in n.body.iter() {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = &item {
                if import.specifiers.len() == 1 {
                    for specifier in import.specifiers.clone().iter_mut() {
                        if let ImportSpecifier::Default(default) = specifier {
                            // check if first character of sym is uppercase
                            if let Some(char) = default.local.sym.to_string().chars().next() {
                                if char.is_uppercase() {
                                    self.module_statements.push(Some(default.local.sym.clone()));
                                } else {
                                    self.module_statements.push(None);
                                }
                            } else {
                                self.module_statements.push(None);
                            }
                        } else {
                            self.module_statements.push(None);
                        }
                    }
                } else {
                    self.module_statements.push(None);
                }
            } else {
                break;
            }
        }
        n.visit_mut_children_with(self);
        self.completed_lookup = true;
        let mut insert_index = self.module_statements.len();
        for (index, statement) in self.module_statements.iter_mut().enumerate() {
            if let Some(constant_name) = &statement {
                if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = n.body[index].clone() {
                    let resolved_path = resolve_path(&self.file_path, &import.src.value);
                    let file_hash = hash(&resolved_path, self.is_dev);
                    n.body.insert(
                        insert_index,
                        lazy_import(constant_name, &file_hash.into(), &import.src.value),
                    );
                    insert_index += 1;
                }
            }
        }
        let mut index = 0;
        n.body.retain(|_| {
            let should_retain =
                index >= self.module_statements.len() || self.module_statements[index].is_none();
            index += 1;
            should_retain
        });
        n.visit_mut_children_with(self);
    }

    fn visit_mut_import_decl(&mut self, _n: &mut ImportDecl) {}
    fn visit_mut_jsx_closing_element(&mut self, n: &mut JSXClosingElement) {
        if self.completed_lookup {
            n.visit_mut_children_with(self);
        }
    }

    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        if self.completed_lookup {
            n.visit_mut_children_with(self);
        } else {
            let mut is_route = false;
            for attr in n.attrs.iter_mut() {
                attr.visit_mut_children_with(self);
                if let JSXAttrOrSpread::JSXAttr(jsxattr) = &attr {
                    if let JSXAttrName::Ident(ident) = &jsxattr.name {
                        if ident.sym.eq("route") {
                            is_route = true
                        }
                    }
                }
            }
            if !is_route {
                n.name.visit_mut_children_with(self);
            }
        }
    }

    fn visit_mut_ident(&mut self, n: &mut Ident) {
        for statement in self.module_statements.iter_mut() {
            if let Some(sym) = &statement {
                if n.sym == *sym {
                    if self.completed_lookup {
                        n.span = DUMMY_SP;
                    } else {
                        *statement = None;
                        return;
                    }
                }
            }
        }
    }
}
