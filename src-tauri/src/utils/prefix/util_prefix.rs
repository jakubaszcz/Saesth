use crate::global::global::{PREFIX_FOR_SETTING, PREFIXES};

pub fn util_prefix_find_prefix(id: &str) -> String {
    for prefix in PREFIXES.iter() {
        if id.contains(prefix) {
            return prefix.to_string();
        }
    }

    String::new()
}
pub fn util_prefix_remove_prefix(id: &str) -> String {
    id.replace(format!("{}_", util_prefix_find_prefix(id)).as_str(), "")
}

pub fn util_prefix_add_prefix(prefix: &str, id: &str) -> String {
    format!("{}_{}", prefix, id)
}