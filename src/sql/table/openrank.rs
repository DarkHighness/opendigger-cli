use gluesql::core::ast::ColumnDef;
use gluesql::core::data::Schema;
use gluesql::prelude::DataType;
use once_cell::sync::Lazy;

pub static OPENRANK_TABLE_NAME: &'static str = "Openrank";
pub static OPENRANK_TABLE_SCHEMA: Lazy<Schema> = Lazy::new(|| Schema {
    table_name: OPENRANK_TABLE_NAME.to_string(),
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
});
