mod report;
mod table;
mod util;

pub use table::TableUI;
pub use report::{RepoOverviewUI};
pub use util::{render_csv, render_rows, render_table};

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
