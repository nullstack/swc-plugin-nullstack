mod loaders;
use loaders::{
    inject_accept_to_module::InjectAcceptVisitor, inject_hash_to_class::InjectHashVisitor,
    inject_inner_components_to_class::InjectInnerComponentVisitor,
    inject_restart_to_module::InjectRestartVisitor, inject_runtime_to_module::InjectRuntimeVisitor,
    inject_source_to_events::InjectSourceVisitor,
    register_server_functions_on_server::RegisterServerFunctionVisitor,
    remove_styles_on_server::RemoveStylesVisitor, remove_unused_from_client::RemoveUnusedVisitor,
    replace_lazy_on_module::ReplaceLazyVisitor, replace_ref_on_attributes::ReplaceRefVisitor,
    replace_server_functions_on_client::ReplaceServerFunctionVisitor,
};
use swc_common::plugin::metadata::TransformPluginMetadataContextKind;
use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[cfg(test)]
mod tests;

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct NullstackPluginOptions {
    #[serde(default)]
    development: bool,
    #[serde(default)]
    client: bool,
    #[serde(default)]
    template: bool,
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
        .unwrap_or_else(|| "{}".into());
    let config = serde_json::from_str::<NullstackPluginOptions>(&config_string)
        .unwrap_or_else(|_| NullstackPluginOptions::default());
    let mut file_path = absolute_file_path.replace(&cwd, "");
    file_path.remove(0);

    if config.template {
        if config.development {
            program.visit_mut_with(&mut InjectAcceptVisitor::new(file_path.clone()));
            // for now its never used in prod
            program.visit_mut_with(&mut ReplaceLazyVisitor::new(
                file_path.clone(),
                config.development,
            ));
        }
        program.visit_mut_with(&mut ReplaceRefVisitor::default());
        program.visit_mut_with(&mut InjectSourceVisitor::default());
        program.visit_mut_with(&mut InjectHashVisitor::new(file_path, config.development));
        program.visit_mut_with(&mut InjectInnerComponentVisitor::default());
        if config.client {
            program.visit_mut_with(&mut ReplaceServerFunctionVisitor::default());
            program.visit_mut_with(&mut RemoveUnusedVisitor::default());
        } else {
            program.visit_mut_with(&mut RemoveStylesVisitor::default());
            program.visit_mut_with(&mut RegisterServerFunctionVisitor::default());
        }
    } else if config.development
        && (file_path.eq("server.js")
            || file_path.eq("server.ts")
            || file_path.eq("client.js")
            || file_path.eq("client.ts"))
    {
        program.visit_mut_with(&mut InjectRuntimeVisitor::default());
        program.visit_mut_with(&mut InjectRestartVisitor::default());
    }

    program
}
