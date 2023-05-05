use crate::sql::analyzer::{AnalysisOutput, AnalyzeError};
use gluesql::core::ast;

pub use storage::{Storage, StorageBuildError};
pub use table::{AggregateTableEntry, TableTypes, UserTables};

mod analyzer;
mod storage;
mod table;

pub fn analyse_statements(statements: &[ast::Statement]) -> Result<AnalysisOutput, AnalyzeError> {
    let analyzer = analyzer::Analyzer::new();
    let result = analyzer.analyze_statements(statements)?;

    Ok(result)
}
