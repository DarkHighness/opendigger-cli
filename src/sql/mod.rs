pub use storage::{Storage, StorageError};
pub use strategy::{create_strategy_instance, StorageStrategy, StorageStrategyType};
pub use table::TableType;

mod storage;
mod strategy;
mod table;

pub use table::ALL_SCHEMAS;
