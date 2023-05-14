use gluesql::core::ast;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::sql::table::TableEntry;

#[derive(Debug)]
pub struct Analyzer {
    aliases: HashMap<Box<str>, String>,
    entries: HashSet<TableEntry>,
}

#[derive(Debug, thiserror::Error)]
pub enum AnalyzeError {
    #[error("Duplicate alias {0} for table {1} and {2}")]
    DuplicateAlias(String, String, String),
    #[error("Bad Query")]
    BadQuery,
}

#[derive(Debug)]
pub struct AnalysisOutput {
    pub tables: Vec<TableEntry>,
}

impl AnalysisOutput {
    pub fn build_from_table_entries(tables: HashSet<TableEntry>) -> Self {
        Self {
            tables: tables.into_iter().collect_vec(),
        }
    }
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            aliases: Default::default(),
            entries: Default::default(),
        }
    }

    fn lookup_alias(&self, alias: &str) -> Option<&str> {
        self.aliases.get(alias).map(|e| e.as_str())
    }

    fn collect_tables_and_alias(&mut self, body: &ast::SetExpr) -> Result<(), AnalyzeError> {
        match body {
            ast::SetExpr::Select(body) => {
                let relation = &body.from.relation;
                if let ast::TableFactor::Table {
                    name,
                    alias,
                    index: _index,
                } = relation
                {
                    let object_name = name.to_string();
                    let table_alias = alias
                        .as_ref()
                        .map(|e| e.name.clone())
                        .unwrap_or("<unnamed>".to_string());
                    let old_alias = self.lookup_alias(&table_alias);

                    if let Some(old_alias) = old_alias {
                        return Err(AnalyzeError::DuplicateAlias(
                            table_alias,
                            object_name,
                            old_alias.to_string(),
                        ));
                    }

                    self.aliases.insert(table_alias.into(), object_name);
                }
            }
            _ => return Err(AnalyzeError::BadQuery),
        };

        Ok(())
    }

    fn try_collect_table_info(
        &mut self,
        left: &ast::Expr,
        op: &ast::BinaryOperator,
        right: &ast::Expr,
    ) {
        if matches!(op, ast::BinaryOperator::Eq) {
            let (alias, condition) = match left {
                ast::Expr::CompoundIdentifier { alias, ident } => (alias.as_str(), ident.as_str()),
                ast::Expr::Identifier(id) => ("<unnamed>", id.as_str()),
                _ => return,
            };

            if condition != "name" {
                return;
            }

            let owner = match right {
                ast::Expr::Literal(ast::AstLiteral::QuotedString(target)) => target.as_str(),
                _ => return,
            };

            let table = self.lookup_alias(alias);

            if let Some(table) = table &&
                let Some(entry) = TableEntry::parse(table, owner){
                    tracing::debug!("Found table entry: {:?}", entry);

                    self.entries.insert(entry);
            }
        }
    }

    fn analyze_expr(&mut self, expr: &ast::Expr) -> Result<(), AnalyzeError> {
        match expr {
            ast::Expr::IsNull(expr) => self.analyze_expr(expr)?,
            ast::Expr::IsNotNull(expr) => self.analyze_expr(expr)?,
            ast::Expr::InList {
                expr,
                list,
                negated: _,
            } => {
                self.analyze_expr(expr)?;
                for expr in list {
                    self.analyze_expr(expr)?;
                }
            }
            ast::Expr::InSubquery {
                expr,
                subquery,
                negated: _,
            } => {
                self.analyze_expr(expr)?;
                self.analyze_query(subquery)?;
            }
            ast::Expr::Between {
                expr,
                negated: _,
                low,
                high,
            } => {
                self.analyze_expr(expr)?;
                self.analyze_expr(low)?;
                self.analyze_expr(high)?;
            }
            ast::Expr::Like {
                expr,
                negated: _,
                pattern,
            } => {
                self.analyze_expr(expr)?;
                self.analyze_expr(pattern)?;
            }
            ast::Expr::ILike {
                expr,
                negated: _,
                pattern,
            } => {
                self.analyze_expr(expr)?;
                self.analyze_expr(pattern)?;
            }
            ast::Expr::BinaryOp { left, op, right } => {
                self.try_collect_table_info(left, op, right);

                self.analyze_expr(left)?;
                self.analyze_expr(right)?;
            }
            ast::Expr::UnaryOp { op: _, expr } => {
                self.analyze_expr(expr)?;
            }
            ast::Expr::Nested(expr) => self.analyze_expr(expr)?,
            ast::Expr::Exists {
                subquery,
                negated: _,
            } => self.analyze_query(subquery)?,
            ast::Expr::Subquery(subquery) => self.analyze_query(subquery)?,
            ast::Expr::Case {
                operand,
                when_then,
                else_result,
            } => {
                if let Some(operand) = operand {
                    self.analyze_expr(operand)?;
                }

                for (when, then) in when_then {
                    self.analyze_expr(when)?;
                    self.analyze_expr(then)?;
                }

                if let Some(else_result) = else_result {
                    self.analyze_expr(else_result)?;
                }
            }
            ast::Expr::ArrayIndex { obj, indexes } => {
                self.analyze_expr(obj)?;
                for index in indexes {
                    self.analyze_expr(index)?;
                }
            }
            ast::Expr::Interval {
                expr,
                leading_field: _,
                last_field: _,
            } => {
                self.analyze_expr(expr)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn analyze_query(&mut self, query: &ast::Query) -> Result<(), AnalyzeError> {
        let body = &query.body;
        if let ast::SetExpr::Select(select) = body &&
        let Some(selection) = select.selection.as_ref() {
            self.analyze_expr(selection)?;
        }

        Ok(())
    }

    /**
     * Some examples of queries:
     * select * from OpenRank where repoName = 'rust-lang/rust'; Valid
     * select * from OpenRank, OpenRank where repoName = 'rust-lang/rust'; Invalid, Only one unnamed table is allowed
     * select * from OpenRank o1, OpenRank o2 where o1.repoName = 'rust-lang/rust' and o2.repoName = 'rust-lang/rust'; Invalid
     **/

    pub fn analyze_statements(
        mut self,
        statements: &[ast::Statement],
    ) -> Result<AnalysisOutput, AnalyzeError> {
        for statement in statements {
            if let ast::Statement::Query(query) = statement {
                tracing::debug!("Analyze query: {:?}", query);

                self.collect_tables_and_alias(&query.body)?;
                self.analyze_query(query)?;
            }
        }

        tracing::debug!("Aliases: {:?}", self.aliases);
        tracing::debug!("Tables: {:?}", self.entries);

        if self.entries.is_empty() {
            return Err(AnalyzeError::BadQuery);
        }

        let output = AnalysisOutput::build_from_table_entries(self.entries);

        Ok(output)
    }
}
