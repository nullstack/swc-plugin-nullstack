pub mod inject_accept_to_module;
pub mod inject_hash_to_class;
pub mod inject_inner_components_to_class;
pub mod inject_restart_to_module;
pub mod inject_runtime_to_module;
pub mod inject_source_to_events;
pub mod register_server_functions_on_server;
pub mod remove_styles_on_server;
pub mod remove_unused_from_client;
pub mod replace_lazy_on_module;
pub mod replace_ref_on_attributes;
pub mod replace_server_functions_on_client;

use swc_common::{chain, Mark};
use swc_core::ecma::{
    transforms::base::resolver,
    visit::{as_folder, Fold, VisitMut},
};
use swc_ecma_parser::{Syntax, TsConfig};

#[allow(dead_code)]
pub fn syntax() -> Syntax {
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..TsConfig::default()
    })
}

#[allow(dead_code)]
pub fn tr<T: VisitMut>(visitor: T) -> impl Fold {
    chain!(
        resolver(Mark::new(), Mark::new(), false),
        as_folder(visitor)
    )
}
