use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ImportIndex {
    pub lang: String,
    pub entries: HashMap<String, String>,
}
