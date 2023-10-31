//! src/routes/mod.rs
mod get_animal_by_period;
mod health_check;
mod subscriptions;

pub use get_animal_by_period::*;
pub use health_check::*;
pub use subscriptions::*;
