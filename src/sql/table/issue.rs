use std::collections::BTreeMap;

use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::{DataType, Key, Row, Value};
use lazy_static::lazy_static;

use crate::api::Metric;

use super::{DataError, TableOwner};

pub static ISSUES_TABLE_NAME: &str = "Issues";
pub static ISSUES_NEW_TABLE_NAME: &str = "IssuesNew";
pub static ISSUES_CLOSED_TABLE_NAME: &str = "IssuesClosed";
pub static ISSUE_COMMENTS_TABLE_NAME: &str = "IssueComments";
pub static ISSUE_RESPONSE_TIME_TABLE_NAME: &str = "IssueResponseTime";
pub static ISSUE_RESOLUTION_DURATION_TABLE_NAME: &str = "IssueResolutionDuration";
pub static ISSUE_AGE_TABLE_NAME: &str = "IssueAge";

lazy_static! {
    pub static ref ISSUES_TABLE_SCHEMA: Schema = Schema {
        table_name: ISSUES_TABLE_NAME.to_string(),
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
                name: "new".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
            ColumnDef {
                name: "closed".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
            ColumnDef {
                name: "comments".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
    pub static ref ISSUES_NEW_TABLE_SCHEMA: Schema = Schema {
        table_name: ISSUES_NEW_TABLE_NAME.to_string(),
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
    pub static ref ISSUES_CLOSED_TABLE_SCHEMA: Schema = Schema {
        table_name: ISSUES_CLOSED_TABLE_NAME.to_string(),
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
    pub static ref ISSUE_COMMENTS_TABLE_SCHEMA: Schema = Schema {
        table_name: ISSUE_COMMENTS_TABLE_NAME.to_string(),
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
    pub static ref ISSUE_RESPONSE_TIME_TABLE_SCHEMA: Schema = Schema {
        table_name: ISSUE_RESPONSE_TIME_TABLE_NAME.to_string(),
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
                name: "avg".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q0".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q1".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q2".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q3".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q4".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
    pub static ref ISSUE_RESOLUTION_DURATION_TABLE_SCHEMA: Schema = Schema {
        table_name: ISSUE_RESOLUTION_DURATION_TABLE_NAME.to_string(),
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
                name: "avg".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q0".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q1".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q2".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q3".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q4".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
    pub static ref ISSUE_AGE_TABLE_SCHEMA: Schema = Schema {
        table_name: ISSUE_AGE_TABLE_NAME.to_string(),
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
                name: "avg".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q0".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q1".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q2".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q3".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
            ColumnDef {
                name: "Q4".to_string(),
                data_type: DataType::Float,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
}

#[allow(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
struct Response {
    avg: BTreeMap<String, f64>,
    levels: BTreeMap<String, Vec<i64>>,
    quantile_0: BTreeMap<String, f64>,
    quantile_1: BTreeMap<String, f64>,
    quantile_2: BTreeMap<String, f64>,
    quantile_3: BTreeMap<String, f64>,
    quantile_4: BTreeMap<String, f64>,
}

pub(crate) async fn fetch_detail_data(
    owner: &TableOwner,
    metric: &Metric,
) -> Result<Vec<(Key, Row)>, DataError> {
    let api = crate::api::get();
    let data = api
        .get::<Response>(owner.to_string().as_str(), *metric)
        .await?;

    let items = data
        .avg
        .keys()
        .map(|key| {
            let avg = data.avg.get(key).unwrap();
            let q0 = data.quantile_0.get(key).unwrap();
            let q1 = data.quantile_1.get(key).unwrap();
            let q2 = data.quantile_2.get(key).unwrap();
            let q3 = data.quantile_3.get(key).unwrap();
            let q4 = data.quantile_4.get(key).unwrap();

            let row = Row(vec![
                Value::Str(owner.to_string()),
                Value::Str(key.clone()),
                Value::F64(*avg),
                Value::F64(*q0),
                Value::F64(*q1),
                Value::F64(*q2),
                Value::F64(*q3),
                Value::F64(*q4),
            ]);

            let key = Key::Str(key.clone());

            (key, row)
        })
        .collect::<Vec<_>>();

    Ok(items)
}

pub(crate) async fn fetch_combined_data(owner: &TableOwner) -> Result<Vec<(Key, Row)>, DataError> {
    let api = crate::api::get();
    let owner = owner.to_string();

    let (data_new, data_closed, data_comments) = futures::future::join3(
        api.get::<BTreeMap<String, i64>>(
            owner.as_str(),
            crate::api::RepositoryMetric::IssuesNew.into(),
        ),
        api.get::<BTreeMap<String, i64>>(
            owner.as_str(),
            crate::api::RepositoryMetric::IssuesClosed.into(),
        ),
        api.get::<BTreeMap<String, i64>>(
            owner.as_str(),
            crate::api::RepositoryMetric::IssueComments.into(),
        ),
    )
    .await;

    match (data_new, data_closed, data_comments) {
        (Ok(data_new), Ok(data_closed), Ok(data_comments)) => {
            let items = data_new
                .into_iter()
                .map(|(month, value)| {
                    let closed = data_closed.get(&month).cloned().unwrap_or(0);
                    let comments = data_comments.get(&month).cloned().unwrap_or(0);

                    let row = Row(vec![
                        Value::Str(owner.to_string()),
                        Value::Str(month),
                        Value::I64(value),
                        Value::I64(closed),
                        Value::I64(comments),
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
