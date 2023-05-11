use async_trait::async_trait;
use gluesql::{
    core::{
        data::Schema,
        result::MutResult,
        store::{RowIter, Store, StoreMut},
    },
    prelude::{Key, Row},
};

use super::Storage;

#[async_trait(? Send)]
impl Store for Storage {
    #[tracing::instrument(skip(self))]
    async fn fetch_schema(
        &self,
        table_name: &str,
    ) -> gluesql::core::result::Result<Option<Schema>> {
        Ok(self
            .tables
            .iter()
            .find(|t| t.name() == table_name)
            .map(|t| t.schema()))
    }

    #[tracing::instrument(skip(self))]
    async fn fetch_all_schemas(&self) -> gluesql::core::result::Result<Vec<Schema>> {
        self.tables.iter().map(|t| Ok(t.schema())).collect()
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
        let table = self.tables.iter().find(|t| t.name() == table_name).unwrap();

        let rows = table.scan_data();

        Ok(Box::new(rows.into_iter().map(Ok)))
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
