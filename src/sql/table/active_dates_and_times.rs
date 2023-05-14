use std::collections::BTreeMap;

use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::{DataType, Key, Row, Value};
use lazy_static::lazy_static;

use crate::api::Metric;

use super::{DataFetchError, TableOwner};

pub static ACTIVE_DATES_AND_TIMES_TABLE_NAME: &'static str = "ActiveDatesAndTimes";

lazy_static! {
    pub static ref ACTIVE_DATES_AND_TIMES_TABLE_SCHEMA: Schema = Schema {
        table_name: ACTIVE_DATES_AND_TIMES_TABLE_NAME.to_string(),
        column_defs: vec![
            ColumnDef {
                name: "name".to_owned(),
                data_type: DataType::Text,
                options: vec![],
            },
            ColumnDef {
                name: "date".to_owned(),
                data_type: DataType::Date,
                options: vec![],
            },
            ColumnDef {
                name: "hour".to_owned(),
                data_type: DataType::Int,
                options: vec![],
            },
            ColumnDef {
                name: "value".to_string(),
                data_type: DataType::Int,
                options: vec![],
            },
        ],
        indexes: vec![],
    };
}

pub(crate) async fn fetch_data(
    owner: &TableOwner,
    metric: &Metric,
) -> Result<Vec<(Key, Row)>, DataFetchError> {
    let api = crate::api::get();
    let data = api
        .get::<BTreeMap<String, Vec<i64>>>(owner.to_string().as_str(), metric.clone())
        .await?;

    let items = data
        .iter()
        .flat_map(|(month, value)| {
            value
                .into_iter()
                .enumerate()
                .map(|(hour, value)| {
                    let day = hour / 24;
                    let hour = hour % 24;

                    let time = format!("{}-{}", month, day + 1);

                    let date = chrono::NaiveDate::parse_from_str(time.as_str(), "%Y-%m-%d")
                        .expect("Failed to parse date");

                    let row = Row(vec![
                        Value::Str(owner.to_string()),
                        Value::Date(date.clone()),
                        Value::I64(hour as i64),
                        Value::I64(value.clone()),
                    ]);

                    let key = Key::Date(date);

                    (key, row)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(items)
}
