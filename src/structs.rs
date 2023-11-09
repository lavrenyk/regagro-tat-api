use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub region_id: Option<u32>,
    pub date_reg_from: Option<String>,
    pub date_reg_to: Option<String>,
    pub kinds: Option<String>,
    pub districts: Option<String>,
}
