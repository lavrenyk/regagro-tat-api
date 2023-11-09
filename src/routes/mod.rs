//! src/routes/mod.rs
mod get_animal_count_by_district;
mod get_animal_count_by_kind_for_period;
mod health_check;

pub use get_animal_count_by_district::*;
pub use get_animal_count_by_kind_for_period::*;
pub use health_check::*;
