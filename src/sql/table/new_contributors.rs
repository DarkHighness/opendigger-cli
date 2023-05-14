use std::collections::BTreeMap;

use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::{DataType, Key, Row, Value};
use lazy_static::lazy_static;

use crate::api::Metric;

use super::{DataError, TableOwner};

pub static NEW_CONTRIBUTORS_TABLE_NAME: &str = "NewContributors";
pub static NEW_CONTRIBUTORS_DETAIL_TABLE_NAME: &str = "NewContributorsDetail";

lazy_static! {
    pub static ref NEW_CONTRIBUTORS_TABLE_SCHEMA: Schema = Schema {
        table_name: NEW_CONTRIBUTORS_TABLE_NAME.to_string(),
        column_defs: vec![
            ColumnDef {
                name: "name".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "month".to_owned(),
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
    };
    pub static ref NEW_CONTRIBUTORS_DETAIL_TABLE_SCHEMA: Schema = Schema {
        table_name: NEW_CONTRIBUTORS_DETAIL_TABLE_NAME.to_string(),
        column_defs: vec![
            ColumnDef {
                name: "name".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "month".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "user".to_string(),
                data_type: DataType::Text,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
}

pub(crate) async fn fetch_detail_data(
    owner: &TableOwner,
    metric: &Metric,
) -> Result<Vec<(Key, Row)>, DataError> {
    let api = crate::api::get();
    let data = api
        .get::<BTreeMap<String, Vec<String>>>(owner.to_string().as_str(), *metric)
        .await?;

    let items = data
        .into_iter()
        .flat_map(|(month, value)| {
            value
                .into_iter()
                .map(|user| {
                    let row = Row(vec![
                        Value::Str(owner.to_string()),
                        Value::Str(month.clone()),
                        Value::Str(user),
                    ]);

                    let key = Key::Str(owner.to_string());

                    (key, row)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(items)
}
