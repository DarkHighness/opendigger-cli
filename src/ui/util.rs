use std::io;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use gluesql::core::data::Interval;
use itertools::Itertools;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use super::UIError;

pub fn render_gluesql_value(value: &gluesql::prelude::Value) -> String {
    match value {
        gluesql::prelude::Value::Null => "NULL".to_string(),
        gluesql::prelude::Value::I8(value) => value.to_string(),
        gluesql::prelude::Value::U8(value) => value.to_string(),
        gluesql::prelude::Value::I16(value) => value.to_string(),
        gluesql::prelude::Value::I32(value) => value.to_string(),
        gluesql::prelude::Value::I128(value) => value.to_string(),
        gluesql::prelude::Value::I64(value) => value.to_string(),
        gluesql::prelude::Value::F64(value) => value.to_string(),
        gluesql::prelude::Value::Str(value) => value.to_string(),
        gluesql::prelude::Value::Bool(value) => value.to_string(),
        gluesql::prelude::Value::Date(value) => value.to_string(),
        gluesql::prelude::Value::Timestamp(value) => value.to_string(),
        gluesql::prelude::Value::Bytea(value) => {
            value.iter().map(|byte| format!("{:02X}", byte)).join("")
        }
        gluesql::prelude::Value::Decimal(value) => value.to_string(),
        gluesql::prelude::Value::Time(value) => value.to_string(),
        gluesql::prelude::Value::Uuid(value) => value.to_string(),
        gluesql::prelude::Value::Interval(value) => match value {
            Interval::Month(value) => format!("{} months", value),
            Interval::Microsecond(value) => format!("{} ms", value),
        },
        gluesql::prelude::Value::List(value) => {
            let values = value
                .iter()
                .map(render_gluesql_value)
                .collect::<Vec<_>>()
                .join(", ");

            format!("[{}]", values)
        }
        gluesql::prelude::Value::Map(map) => {
            let values = map
                .iter()
                .map(|(key, value)| format!("{}: {}", key, render_gluesql_value(value)))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{{{}}}", values)
        }
    }
}

pub fn render_gluesql_rows(rows: &[gluesql::prelude::Row]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| row.iter().map(render_gluesql_value).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn render_csv(header: &[String], rows: &[Vec<String>]) -> String {
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

pub fn format_human_readable_f64(value: f64) -> String {
    if value < 1_000.0 {
        format!("{:.2}", value)
    } else if value < 1_000_000.0 {
        format!("{:.2}K", value / 1_000.0)
    } else if value < 1_000_000_000.0 {
        format!("{:.2}M", value / 1_000_000.0)
    } else if value < 1_000_000_000_000.0 {
        format!("{:.2}B", value / 1_000_000_000.0)
    } else {
        format!("{:.2}T", value / 1_000_000_000_000.0)
    }
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
