use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::DataType;
use lazy_static::lazy_static;

pub static ACTIVITY_TABLE_NAME: &str = "Activity";

lazy_static! {
    pub static ref ACTIVITY_TABLE_SCHEMA: Schema = Schema {
        table_name: ACTIVITY_TABLE_NAME.to_string(),
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
