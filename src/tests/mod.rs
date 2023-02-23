pub mod helpers;
pub mod inject_hash_to_class;
pub mod inject_inner_components_to_class;
pub mod inject_source_to_events;
pub mod register_server_functions_on_server;
pub mod remove_styles_on_server;
pub mod remove_unused_from_client;
pub mod replace_server_functions_on_client;

use swc_ecma_parser::{EsConfig, Syntax};

#[allow(dead_code)]
pub fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..EsConfig::default()
    })
}
