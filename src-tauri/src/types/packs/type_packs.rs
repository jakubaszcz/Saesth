use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pack {
    name: String,
    pub(crate) id: String,
    description: String,
    pub(crate) icon: String,
}