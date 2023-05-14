use std::collections::BTreeMap;

use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::{DataType, Key, Row, Value};
use lazy_static::lazy_static;

use super::{DataError, TableOwner};

pub static CODE_CHANGE_LINES_TABLE_NAME: &str = "CodeChangeLines";
pub static CODE_CHANGE_LINES_ADD_TABLE_NAME: &str = "CodeChangeLinesAdd";
pub static CODE_CHANGE_LINES_REMOVE_TABLE_NAME: &str = "CodeChangeLinesRemove";
pub static CODE_CHANGE_LINES_SUM_TABLE_NAME: &str = "CodeChangeLinesSum";

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

pub(crate) async fn fetch_combined_data(owner: &TableOwner) -> Result<Vec<(Key, Row)>, DataError> {
    let api = crate::api::get();
    let owner = owner.to_string();

    let (code_change_lines_add, code_change_lines_remove, code_change_lines_sum) =
        futures::future::join3(
            api.get::<BTreeMap<String, i64>>(
                owner.as_str(),
                crate::api::RepositoryMetric::CodeChangeLinesAdd.into(),
            ),
            api.get::<BTreeMap<String, i64>>(
                owner.as_str(),
                crate::api::RepositoryMetric::CodeChangeLinesRemove.into(),
            ),
            api.get::<BTreeMap<String, i64>>(
                owner.as_str(),
                crate::api::RepositoryMetric::CodeChangeLinesSum.into(),
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
                .into_iter()
                .map(|(month, value)| {
                    let remove = code_changes_line_remove.get(&month).unwrap_or(&0);
                    let sum = code_changes_line_sum.get(&month).unwrap_or(&0);

                    let row = Row(vec![
                        Value::Str(owner.to_string()),
                        Value::Str(month.clone()),
                        Value::I64(value),
                        Value::I64(*remove),
                        Value::I64(*sum),
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
