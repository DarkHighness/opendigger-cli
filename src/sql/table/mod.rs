mod activity;
mod attention;
mod openrank;
mod types;

use std::{ops::Deref, sync::Arc};

pub use activity::{ACTIVITY_TABLE_NAME, ACTIVITY_TABLE_SCHEMA};
pub use attention::{ATTENTION_TABLE_NAME, ATTENTION_TABLE_SCHEMA};
use gluesql::{
    core::data::Schema,
    prelude::{Key, Row},
};

pub use openrank::{OPENRANK_TABLE_NAME, OPENRANK_TABLE_SCHEMA};
pub use types::{TableEntry, TableType};

#[derive(Debug)]
pub struct InnerStorageTable {
    r#type: TableType,
    items: Vec<(Key, Row)>,
}

#[derive(Debug, Clone)]
pub struct StorageTable {
    inner: Arc<InnerStorageTable>,
}

impl Deref for StorageTable {
    type Target = InnerStorageTable;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl InnerStorageTable {
    pub fn name(&self) -> &str {
        self.r#type.as_ref()
    }

    pub fn schema(&self) -> Schema {
        match self.r#type {
            TableType::OpenRank => OPENRANK_TABLE_SCHEMA.clone(),
            TableType::Activity => ACTIVITY_TABLE_SCHEMA.clone(),
            TableType::Attention => ATTENTION_TABLE_SCHEMA.clone(),
        }
    }

    pub fn items(&self) -> Vec<(Key, Row)> {
        self.items.clone()
    }
}

impl StorageTable {
    pub fn new(r#type: TableType, items: Vec<(Key, Row)>) -> Self {
        Self {
            inner: Arc::new(InnerStorageTable { r#type, items }),
        }
    }
}
