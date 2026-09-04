use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pack {
    name: String,
    description: String,
    pub(crate) icon: String,
}