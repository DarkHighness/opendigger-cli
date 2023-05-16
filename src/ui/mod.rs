mod report;
mod table;
mod util;

pub use report::RepoOverviewUI;
pub use table::TableUI;
pub use util::{format_human_readable_f64, render_csv, render_gluesql_rows};

#[derive(Debug, Clone, Copy)]
pub enum UIMode {
    Simple,
    Interactive,
}

#[derive(Debug, thiserror::Error)]
pub enum UIError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub trait UI {
    fn render(self) -> Result<(), UIError>;
}
