pub use error::{Error, Result};

pub mod commands;
mod components;
mod error;
mod family_manager;
mod relationships;

pub use family_manager::FamilyManager;
pub use family_manager::FamilyRow;
