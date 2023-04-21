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

pub fn hash(text: &str, is_dev: bool) -> String {
    let mut replaced = text.replace('\\', "/");
    if replaced.ends_with("/index.njs")
        || text.ends_with("/index.jsx")
        || text.ends_with("/index.nts")
        || text.ends_with("/index.tsx")
    {
        replaced.drain(replaced.len() - 10..);
    }
    let fragments: Vec<&str> = replaced.split('.').collect();
    if let Some(file_name) = fragments.first() {
        if is_dev {
            return file_name.replace('/', "__");
        } else {
            let checksum = crc32fast::hash(file_name.as_bytes());
            return format!("{:x}", checksum);
        }
    }
    "".to_string()
}

pub fn combine_hash(file_hash: &str, class_hash: &str, is_dev: bool) -> String {
    if is_dev {
        format!("{}___{}", file_hash, class_hash)
    } else {
        format!("{}{}", file_hash, class_hash)
    }
}
