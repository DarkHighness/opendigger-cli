use std::collections::BTreeMap;

use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::{DataType, Key, Row, Value};
use lazy_static::lazy_static;

use crate::api::Metric;

use super::{DataFetchError, TableOwner};

pub static CODE_CHANGE_LINES_TABLE_NAME: &'static str = "CodeChangeLines";
pub static CODE_CHANGE_LINES_ADD_TABLE_NAME: &'static str = "CodeChangeLinesAdd";
pub static CODE_CHANGE_LINES_REMOVE_TABLE_NAME: &'static str = "CodeChangeLinesRemove";
pub static CODE_CHANGE_LINES_SUM_TABLE_NAME: &'static str = "CodeChangeLinesSum";

lazy_static! {
    pub static ref CODE_CHANGE_LINE_TABLE_SCHEMA: Schema = Schema {
        table_name: CODE_CHANGE_LINES_TABLE_NAME.to_string(),
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
                name: "add".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
            ColumnDef {
                name: "remove".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
            ColumnDef {
                name: "sum".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
    pub static ref CODE_CHANGE_LINES_ADD_TABLE_SCHEMA: Schema = Schema {
        table_name: CODE_CHANGE_LINES_ADD_TABLE_NAME.to_string(),
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
    pub static ref CODE_CHANGE_LINES_REMOVE_TABLE_SCHEMA: Schema = Schema {
        table_name: CODE_CHANGE_LINES_REMOVE_TABLE_NAME.to_string(),
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
    pub static ref CODE_CHANGE_LINES_SUM_TABLE_SCHEMA: Schema = Schema {
        table_name: CODE_CHANGE_LINES_SUM_TABLE_NAME.to_string(),
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
}

pub(crate) async fn fetch_data(owner: &TableOwner) -> Result<Vec<(Key, Row)>, DataFetchError> {
    let api = crate::api::get();

    let (code_change_lines_add, code_change_lines_remove, code_change_lines_sum) =
        futures::future::join3(
            api.get::<BTreeMap<String, i64>>(
                owner.to_string().as_str(),
                Metric::Repo(crate::api::RepositoryMetric::CodeChangeLinesAdd),
            ),
            api.get::<BTreeMap<String, i64>>(
                owner.to_string().as_str(),
                Metric::Repo(crate::api::RepositoryMetric::CodeChangeLinesRemove),
            ),
            api.get::<BTreeMap<String, i64>>(
                owner.to_string().as_str(),
                Metric::Repo(crate::api::RepositoryMetric::CodeChangeLinesSum),
            ),
        )
        .await;

    match (
        code_change_lines_add,
        code_change_lines_remove,
        code_change_lines_sum,
    ) {
        (Ok(code_changes_line_add), Ok(code_changes_line_remove), Ok(code_changes_line_sum)) => {
            let items = code_changes_line_add
                .iter()
                .map(|(month, value)| {
                    let remove = code_changes_line_remove.get(month).unwrap_or(&0);
                    let sum = code_changes_line_sum.get(month).unwrap_or(&0);

                    let row = Row(vec![
                        Value::Str(owner.to_string()),
                        Value::Str(month.clone()),
                        Value::I64(value.clone()),
                        Value::I64(remove.clone()),
                        Value::I64(sum.clone()),
                    ]);

                    let key = Key::Str(owner.to_string());

                    (key, row)
                })
                .collect::<Vec<_>>();

            Ok(items)
        }
        _ => todo!("handle error"),
    }
}
