use gluesql::{core::data::Interval, prelude::Value};
use itertools::Itertools;
use term_table::{row::Row, table_cell::TableCell};
use term_table::{Table, TableStyle};

pub fn render_value(value: &Value) -> String {
    match value {
        Value::Null => "NULL".to_string(),
        Value::I8(value) => value.to_string(),
        Value::U8(value) => value.to_string(),
        Value::I16(value) => value.to_string(),
        Value::I32(value) => value.to_string(),
        Value::I128(value) => value.to_string(),
        Value::I64(value) => value.to_string(),
        Value::F64(value) => value.to_string(),
        Value::Str(value) => value.to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Date(value) => value.to_string(),
        Value::Timestamp(value) => value.to_string(),
        Value::Bytea(value) => value.iter().map(|byte| format!("{:02X}", byte)).join(""),
        Value::Decimal(value) => value.to_string(),
        Value::Time(value) => value.to_string(),
        Value::Uuid(value) => value.to_string(),
        Value::Interval(value) => match value {
            Interval::Month(value) => format!("{} months", value),
            Interval::Microsecond(value) => format!("{} ms", value),
        },
        Value::List(value) => {
            let values = value
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join(", ");

            format!("[{}]", values)
        }
        Value::Map(map) => {
            let values = map
                .iter()
                .map(|(key, value)| format!("{}: {}", key, render_value(value)))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{{{}}}", values)
        }
    }
}

pub fn render_csv(header: &[String], rows: &[gluesql::prelude::Row]) -> String {
    let header = header
        .iter()
        .map(|header: &String| header.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let rows = rows
        .iter()
        .map(|row| row.iter().map(render_value).collect::<Vec<_>>().join(", "))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}", header, rows)
}

pub fn render_table(header: &[String], rows: &[gluesql::prelude::Row]) -> String {
    let mut table = Table::new();

    table.max_column_width = 40;
    table.style = TableStyle::elegant();

    table.add_row(Row::new(
        header.iter().map(|header| TableCell::new(header.as_str())),
    ));

    for row in rows {
        table.add_row(Row::new(
            row.iter().map(|value| TableCell::new(render_value(value))),
        ));
    }

    table.render()
}
