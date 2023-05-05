use crate::sql::storage::StorageBuildError;
use futures::future;
use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::{DataType, Key, Row, Value};
use std::collections::BTreeMap;

use super::StorageTable;

#[derive(Debug)]
pub struct OpenRankTable {
    pub items: Vec<(String, String, f64)>,
}

impl StorageTable for OpenRankTable {
    fn name(&self) -> &str {
        "Openrank"
    }

    fn schema(&self) -> Schema {
        Schema {
            table_name: self.name().to_string(),
            column_defs: vec![
                ColumnDef {
                    name: "name".to_owned(),
                    data_type: DataType::Text,
                    options: vec![],
                },
                ColumnDef {
                    name: "time".to_owned(),
                    data_type: DataType::Text,
                    options: vec![],
                },
                ColumnDef {
                    name: "value".to_string(),
                    data_type: DataType::Float,
                    options: vec![],
                },
            ],
            indexes: vec![],
        }
    }

    fn scan_data(&self) -> Vec<(Key, Row)> {
        let items = self
            .items
            .iter()
            .map(|(name, time, value)| {
                let row = Row(vec![
                    Value::Str(name.clone()),
                    Value::Str(time.clone()),
                    Value::F64(*value),
                ]);

                let key = Key::Str(time.clone());

                (key, row)
            })
            .collect::<Vec<_>>();

        items
    }
}

impl OpenRankTable {
    pub async fn build(
        names: &Vec<String>,
        r#type: crate::api::Type,
    ) -> Result<Self, StorageBuildError> {
        let api = crate::api::get();
        let metric = match r#type {
            crate::api::Type::Repository => {
                crate::api::Metric::Repo(crate::api::RepoMetric::OpenRank)
            }
            crate::api::Type::User => crate::api::Metric::User(crate::api::UserMetric::OpenRank),
        };
        let data = future::join_all(names.iter().map(async move |owner| {
            (owner, api.get::<BTreeMap<String, f64>>(owner, metric).await)
        }))
        .await
        .into_iter();

        let mut items = vec![];

        for (owner, item) in data {
            let item = item?;
            for (time, value) in item.iter() {
                items.push((owner.clone(), time.clone(), value.clone()));
            }
        }

        Ok(Self { items })
    }
}
