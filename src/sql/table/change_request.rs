use std::collections::BTreeMap;

use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::{DataType, Key, Row, Value};
use lazy_static::lazy_static;

use crate::api::Metric;

use super::{DataError, TableOwner};

pub static CHANGE_REQUESTS_TABLE_NAME: &str = "ChangeRequests";
pub static CHANGE_REQUESTS_OPEN_TABLE_NAME: &str = "ChangeRequestsOpen";
pub static CHANGE_REQUESTS_ACCEPTED_TABLE_NAME: &str = "ChangeRequestsAccepted";
pub static CHANGE_REQUESTS_REVIEWS_TABLE_NAME: &str = "ChangeRequestsReviews";

pub static CHANGE_REQUEST_RESPONSE_TIME_TABLE_NAME: &str = "ChangeRequestResponseTime";
pub static CHANGE_REQUEST_RESOLUTION_DURATION_TABLE_NAME: &str = "ChangeRequestResolutionDuration";
pub static CHANGE_REQUEST_AGE_TABLE_NAME: &str = "ChangeRequestAge";

lazy_static! {
    pub static ref CHANGE_REQUESTS_TABLE_SCHEMA: Schema = Schema {
        table_name: CHANGE_REQUESTS_TABLE_NAME.to_string(),
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
                name: "requests".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
            ColumnDef {
                name: "accepted".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
            ColumnDef {
                name: "reviews".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
    pub static ref CHANGE_REQUESTS_OPEN_TABLE_SCHEMA: Schema = Schema {
        table_name: CHANGE_REQUESTS_OPEN_TABLE_NAME.to_string(),
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
                data_type: DataType::Int,
                options: vec![],
            }
        ],
        indexes: vec![],
    };
    pub static ref CHANGE_REQUESTS_ACCEPTED_TABLE_SCHEMA: Schema = Schema {
        table_name: CHANGE_REQUESTS_ACCEPTED_TABLE_NAME.to_string(),
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
                data_type: DataType::Int,
                options: vec![],
            }
        ],
        indexes: vec![],
    };
    pub static ref CHANGE_REQUESTS_REVIEWS_TABLE_SCHEMA: Schema = Schema {
        table_name: CHANGE_REQUESTS_REVIEWS_TABLE_NAME.to_string(),
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
                data_type: DataType::Int,
                options: vec![],
            }
        ],
        indexes: vec![],
    };
    pub static ref CHANGE_REQUEST_RESPONSE_TIME_TABLE_SCHEMA: Schema = Schema {
        table_name: CHANGE_REQUEST_RESPONSE_TIME_TABLE_NAME.to_string(),
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
    pub static ref CHANGE_REQUEST_RESOLUTION_DURATION_TABLE_SCHEMA: Schema = Schema {
        table_name: CHANGE_REQUEST_RESOLUTION_DURATION_TABLE_NAME.to_string(),
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
    pub static ref CHANGE_REQUEST_AGE_TABLE_SCHEMA: Schema = Schema {
        table_name: CHANGE_REQUEST_AGE_TABLE_NAME.to_string(),
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

pub(crate) async fn fetch_combined_data(owner: &TableOwner) -> Result<Vec<(Key, Row)>, DataError> {
    let api = crate::api::get();

    let (change_requests_open, change_requests_accepted, change_requests_reviews) =
        futures::future::join3(
            api.get::<BTreeMap<String, i64>>(
                owner.to_string().as_str(),
                Metric::Repo(crate::api::RepositoryMetric::ChangeRequests),
            ),
            api.get::<BTreeMap<String, i64>>(
                owner.to_string().as_str(),
                Metric::Repo(crate::api::RepositoryMetric::ChangeRequestsAccepted),
            ),
            api.get::<BTreeMap<String, i64>>(
                owner.to_string().as_str(),
                Metric::Repo(crate::api::RepositoryMetric::ChangeRequestsReviews),
            ),
        )
        .await;

    match (
        change_requests_open,
        change_requests_accepted,
        change_requests_reviews,
    ) {
        (Ok(change_requests_open), Ok(change_requests_accepted), Ok(change_requests_reviews)) => {
            let items = change_requests_open
                .iter()
                .map(|(month, requests)| {
                    let accepted = change_requests_accepted.get(month).unwrap_or(&0);
                    let reviews = change_requests_reviews.get(month).unwrap_or(&0);
                    let row = Row(vec![
                        Value::Str(owner.to_string()),
                        Value::Str(month.to_string()),
                        Value::I64(*requests),
                        Value::I64(*accepted),
                        Value::I64(*reviews),
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
