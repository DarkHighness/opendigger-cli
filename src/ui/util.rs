use std::io;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use gluesql::{core::data::Interval, prelude::Value};
use itertools::Itertools;
use term_table::{row::Row, table_cell::TableCell};
use term_table::{Table, TableStyle};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

use super::UIError;

pub fn render_sql_value(value: &Value) -> String {
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
                .map(render_sql_value)
                .collect::<Vec<_>>()
                .join(", ");

            format!("[{}]", values)
        }
        Value::Map(map) => {
            let values = map
                .iter()
                .map(|(key, value)| format!("{}: {}", key, render_sql_value(value)))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{{{}}}", values)
        }
    }
}

pub fn render_rows(rows: &[gluesql::prelude::Row]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| row.iter().map(render_sql_value).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn render_csv(header: &[String], rows: &Vec<Vec<String>>) -> String {
    let header = header
        .iter()
        .map(|header: &String| header.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let rows = rows
        .iter()
        .map(|row| row.iter().map(|value| value.to_string()).join(", "))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}", header, rows)
}

pub fn render_table(header: &[String], rows: &Vec<Vec<String>>) -> String {
    let mut table = Table::new();

    table.max_column_width = 40;
    table.style = TableStyle::elegant();

    table.add_row(Row::new(
        header.iter().map(|header| TableCell::new(header.as_str())),
    ));

    for row in rows {
        table.add_row(Row::new(
            row.iter().map(|value| TableCell::new(value.as_str())),
        ));
    }

    table.render()
}

pub fn create_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, UIError> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn destory_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), UIError> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}
