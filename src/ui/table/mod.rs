mod interactive;
mod simple;

use crate::engine::Value;

use self::{interactive::InteractiveTableUI, simple::SimpleTableUI};

use super::{UIError, UIMode, UI};

pub enum TableUI {
    Simple(simple::SimpleTableUI),
    Interactive(interactive::InteractiveTableUI),
}

impl TableUI {
    pub fn new(
        mode: UIMode,
        name: String,
        header: Vec<String>,
        rows: Vec<Vec<Value>>,
    ) -> Result<Self, UIError> {
        let ui = match mode {
            UIMode::Simple => TableUI::Simple(SimpleTableUI::new(name, header, rows)?),
            UIMode::Interactive => {
                TableUI::Interactive(InteractiveTableUI::new(name, header, rows)?)
            }
        };

        Ok(ui)
    }

    pub fn render(self) -> Result<(), UIError> {
        match self {
            TableUI::Simple(ui) => ui.render(),
            TableUI::Interactive(ui) => ui.render(),
        }
    }
}
