use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub region_id: Option<u32>,
    pub date_reg_from: Option<String>,
    pub date_from: Option<String>,
    pub date_reg_to: Option<String>,
    pub date_to: Option<String>,
    pub kinds: Option<String>,
    pub kind_ids: Option<String>,
    pub districts: Option<String>,
    pub enterprise_districts: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistrictData {
    pub id: i64,
    pub object_id: i64,
    pub object_guid: String,
    pub change_id: i64,
    pub name: String,
    pub type_name: String,
    pub level: i64,
    pub oper_type_id: i64,
    pub prev_id: i64,
    pub next_id: i64,
    pub update_date: String,
    pub start_date: String,
    pub end_date: String,
    pub is_actual: u8,
    pub is_active: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrDistrictData {
    pub id: i64,
    pub name: String,
    pub view: String,
    pub guid: String,
    pub region_id: u16,
    pub regagro_id: u16,
    pub regagro_code: String,
}
