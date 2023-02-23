#[allow(unused_imports)]
use super::syntax;
use crate::loaders::inject_source_to_events::InjectSourceVisitor;
use swc_core::ecma::{
    transforms::testing::test,
    visit::{as_folder, Fold},
};

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(InjectSourceVisitor::default())
}

test!(
    syntax(),
    |_| tr(),
    inject_source_to_node,
    r#"function Modal() { return <button onclick={close}> x </button> }"#,
    r#"function Modal() { return <button onclick={close} source={this}> x </button> }"#
);

test!(
    syntax(),
    |_| tr(),
    skip_inject_duplicated_source_to_node,
    r#"function Modal() { return <button source={this} onclick={close}> x </button> }"#,
    r#"function Modal() { return <button source={this} onclick={close}> x </button> }"#
);

test!(
    syntax(),
    |_| tr(),
    skip_inject_source_to_node,
    r#"function Modal() { return <button class="kawaii-desu"> x </button> }"#,
    r#"function Modal() { return <button class="kawaii-desu"> x </button> }"#
);
