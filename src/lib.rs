use swc_common::plugin::metadata::TransformPluginMetadataContextKind;
// use swc_common::{chain, plugin::metadata::TransformPluginMetadataContextKind};
use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

mod loaders;
use loaders::{
    inject_hash_to_class::InjectHashVisitor, inject_source_to_events::InjectSourceVisitor,
    replace_server_functions_on_client::ReplaceServerFunctionVisitor,
};

#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    let file_path = metadata
        .get_context(&TransformPluginMetadataContextKind::Filename)
        .unwrap_or_else(|| "/".into());

    let is_dev = true;
    let is_server = true;

    program.visit_mut_with(&mut InjectSourceVisitor::default());
    program.visit_mut_with(&mut InjectHashVisitor::new(file_path, is_dev));
    if !is_server {
        program.visit_mut_with(&mut ReplaceServerFunctionVisitor::default());
    }
    program
}
