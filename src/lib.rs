use swc_common::plugin::metadata::TransformPluginMetadataContextKind;
use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

mod loaders;

use loaders::{
    inject_hash_to_class::InjectHashVisitor,
    inject_inner_components_to_class::InjectInnerComponentVisitor,
    inject_source_to_events::InjectSourceVisitor,
    register_server_functions_on_server::RegisterServerFunctionVisitor,
    remove_styles_on_server::RemoveStylesVisitor,
    replace_server_functions_on_client::ReplaceServerFunctionVisitor,
};

use crate::loaders::remove_unused_from_client::RemoveUnusedVisitor;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NullstackPluginOptions {
    #[serde(default)]
    development: bool,
    #[serde(default)]
    client: bool,
}

impl Default for NullstackPluginOptions {
    fn default() -> Self {
        NullstackPluginOptions {
            development: false,
            client: false,
        }
    }
}

#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    let absolute_file_path = metadata
        .get_context(&TransformPluginMetadataContextKind::Filename)
        .unwrap_or_else(|| "/".into());
    let cwd = metadata
        .get_context(&TransformPluginMetadataContextKind::Cwd)
        .unwrap_or_else(|| "/".into());
    let config_string = metadata
        .get_transform_plugin_config()
        .unwrap_or(String::from("{}"));
    let config = serde_json::from_str::<NullstackPluginOptions>(&config_string)
        .unwrap_or_else(|_| NullstackPluginOptions::default());
    let file_path = absolute_file_path.replace(&cwd, "");

    program.visit_mut_with(&mut InjectSourceVisitor::default());
    program.visit_mut_with(&mut InjectHashVisitor::new(file_path, config.development));
    program.visit_mut_with(&mut InjectInnerComponentVisitor::default());
    // config.client = true;

    if config.client {
        program.visit_mut_with(&mut ReplaceServerFunctionVisitor::default());
        program.visit_mut_with(&mut RemoveUnusedVisitor::default());
    } else {
        program.visit_mut_with(&mut RemoveStylesVisitor::default());
        program.visit_mut_with(&mut RegisterServerFunctionVisitor::default());
    }
    program
}

#[cfg(test)]
mod tests;
