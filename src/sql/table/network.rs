use std::collections::HashMap;

use crate::api::Metric;
use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::{DataType, Key, Row, Value};
use lazy_static::lazy_static;

use super::TableOwner;

pub static REPO_NETWORK_TABLE_NAME: &str = "RepoNetwork";
pub static DEVELOPER_NETWORK_TABLE_NAME: &str = "DeveloperNetwork";

lazy_static! {
    pub static ref REPO_NETWORK_TABLE_SCHEMA: Schema = Schema {
        table_name: REPO_NETWORK_TABLE_NAME.to_string(),
        column_defs: vec![
            ColumnDef {
                name: "owner".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "from".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "from_weight".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "to".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "to_weight".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "weight".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
    pub static ref DEVELOPER_NETWORK_TABLE_SCHEMA: Schema = Schema {
        table_name: DEVELOPER_NETWORK_TABLE_NAME.to_string(),
        column_defs: vec![
            ColumnDef {
                name: "owner".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "from".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "from_weight".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "to".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "to_weight".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "weight".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Network {
    nodes: Vec<(String, f64)>,
    edges: Vec<(String, String, f64)>,
}

pub(crate) async fn fetch_network_data(
    owner: &TableOwner,
    metric: &Metric,
) -> Result<Vec<(gluesql::prelude::Key, gluesql::prelude::Row)>, crate::sql::table::DataError> {
    let api = crate::api::get();
    let data = api
        .get::<Network>(owner.to_string().as_str(), *metric)
        .await?;

    let nodes = data.nodes.into_iter().collect::<HashMap<String, f64>>();

    let items = data
        .edges
        .into_iter()
        .map(|(from, to, weight)| {
            let from_weight = nodes.get(&from).cloned().unwrap_or(0.0);
            let to_weight = nodes.get(&to).cloned().unwrap_or(0.0);

            let row = Row(vec![
                Value::Str(owner.to_string()),
                Value::Str(from),
                Value::F64(from_weight),
                Value::Str(to),
                Value::F64(to_weight),
                Value::F64(weight),
            ]);

            let key = Key::Str(owner.to_string());

            (key, row)
        })
        .collect::<Vec<_>>();

    Ok(items)
}
