use swc_core::{
    ecma::{
        ast::Program,
        visit::{as_folder, FoldWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

mod loaders;
use loaders::inject_nullstack_to_modules::InjectNullstackVisitor;

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(InjectNullstackVisitor::default()))
}
