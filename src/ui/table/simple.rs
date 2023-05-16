use term_table::{row::Row, table_cell::TableCell, Table, TableStyle};

use crate::{
    engine::Value,
    ui::{UIError, UI},
};

#[allow(dead_code)]
pub struct SimpleTableUI {
    name: String,
    header: Vec<String>,
    rows: Vec<Vec<Value>>,
}

impl SimpleTableUI {
    pub fn new(name: String, header: Vec<String>, rows: Vec<Vec<Value>>) -> Result<Self, UIError> {
        Ok(SimpleTableUI { name, header, rows })
    }

    pub fn render_table(header: &[String], rows: &Vec<Vec<Value>>) -> String {
        let mut table = Table::new();

        table.max_column_width = 40;
        table.style = TableStyle::elegant();

        table.add_row(Row::new(
            header.iter().map(|header| TableCell::new(header.as_str())),
        ));

        for row in rows {
            table.add_row(Row::new(
                row.iter().map(|value| TableCell::new(value.to_string())),
            ));
        }

        table.render()
    }
}

impl UI for SimpleTableUI {
    fn render(self) -> Result<(), UIError> {
        let output = SimpleTableUI::render_table(&self.header, &self.rows);

        println!("{}", output);

        Ok(())
    }
}
