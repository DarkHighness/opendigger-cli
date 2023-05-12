use std::str::FromStr;

use async_trait::async_trait;
use gluesql::{
    core::{
        data::Schema,
        result::MutResult,
        store::{RowIter, Store, StoreMut},
    },
    prelude::{Key, Row},
};

use crate::sql::TableType;

use super::Storage;

#[async_trait(? Send)]
impl Store for Storage {
    #[tracing::instrument(skip(self))]
    async fn fetch_schema(
        &self,
        table_name: &str,
    ) -> gluesql::core::result::Result<Option<Schema>> {
        let schema = self.strategy.fetch_all_schemas().await.map_err(|err| {
            gluesql::core::result::Error::StorageMsg(format!(
                "Failed to fetch all schemas: {}",
                err
            ))
        })?;

        let schema = schema
            .into_iter()
            .find(|schema| schema.table_name == table_name)
            .map(|schema| schema.clone());

        Ok(schema)
    }

    #[tracing::instrument(skip(self))]
    async fn fetch_all_schemas(&self) -> gluesql::core::result::Result<Vec<Schema>> {
        let schema = self.strategy.fetch_all_schemas().await.map_err(|err| {
            gluesql::core::result::Error::StorageMsg(format!(
                "Failed to fetch all schemas: {}",
                err
            ))
        })?;

        Ok(schema)
    }

    #[tracing::instrument(skip(self))]
    async fn fetch_data(
        &self,
        _table_name: &str,
        _key: &Key,
    ) -> gluesql::core::result::Result<Option<Row>> {
        unimplemented!()
    }

    #[tracing::instrument(skip(self))]
    async fn scan_data(&self, table_name: &str) -> gluesql::core::result::Result<RowIter> {
        let table_type = TableType::from_str(table_name)
            .map_err(|err| gluesql::core::result::Error::StorageMsg(err.to_string()))?;

        let table = self
            .fetch_table(table_type)
            .await
            .map_err(|err| gluesql::core::result::Error::StorageMsg(err.to_string()))?
            .map(|table| table.items().into_iter().map(|(key, row)| Ok((key, row))));

        match table {
            Some(table) => Ok(Box::new(table)),
            None => Err(gluesql::core::result::Error::StorageMsg(format!(
                "Table not found: {}",
                table_name
            ))),
        }
    }
}

#[async_trait(? Send)]
impl StoreMut for Storage {
    async fn insert_schema(self, _schema: &Schema) -> MutResult<Self, ()> {
        unimplemented!()
    }

    async fn delete_schema(self, _table_name: &str) -> MutResult<Self, ()> {
        unimplemented!()
    }

    async fn append_data(self, _table_name: &str, _rows: Vec<Row>) -> MutResult<Self, ()> {
        unimplemented!()
    }

    async fn insert_data(self, _table_name: &str, _rows: Vec<(Key, Row)>) -> MutResult<Self, ()> {
        unimplemented!()
    }

    async fn delete_data(self, _table_name: &str, _keys: Vec<Key>) -> MutResult<Self, ()> {
        unimplemented!()
    }
}
