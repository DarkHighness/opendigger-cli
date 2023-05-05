use sqlparser::ast;
use sqlparser::ast::Visit;
use std::str::FromStr;
use std::{collections::HashMap, ops::ControlFlow};

mod storage;
mod table;

#[derive(Debug)]
struct Analyzer {
    aliases: HashMap<String, String>,
    tables: Vec<(table::Tables, String)>,
}

#[derive(Debug, thiserror::Error)]
enum AnalyzeError {
    #[error("Unexpected body type: {0}")]
    UnexpectedBodyType(String),
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            aliases: Default::default(),
            tables: Default::default(),
        }
    }

    fn lookup_alias(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }

    fn parse_table_and_alias(&mut self, body: &ast::SetExpr) -> Result<(), AnalyzeError> {
        match body {
            ast::SetExpr::Select(body) => body.from.iter().for_each(|table| {
                if let ast::TableFactor::Table {
                    name,
                    alias,
                    args: _args,
                    with_hints: _with_hints,
                } = &table.relation
                {
                    let table_name = name.0.iter().map(|e| e.value.as_str()).fold(
                        String::new(),
                        |mut acc, e| {
                            if !acc.is_empty() {
                                acc.push('.');
                            }
                            acc.push_str(e);
                            acc
                        },
                    );

                    if table::SUPPORTED_TABLE_NAMES.contains(&table_name.as_str()) {
                        if let Some(alias) = alias.as_ref().map(|alias| alias.name.value.clone()) {
                            self.aliases.insert(alias, table_name);
                        }
                    }
                }
            }),
            _ => return Err(AnalyzeError::UnexpectedBodyType(format!("{body}"))),
        };

        Ok(())
    }

    fn parse_tables(&mut self, query: &ast::Query) -> Result<(), AnalyzeError> {
        struct Visitor<'a> {
            analyzer: &'a Analyzer,
            tables: Vec<(table::Tables, String)>,
        }

        impl<'a> ast::Visitor for Visitor<'a> {
            type Break = ();

            fn post_visit_expr(&mut self, expr: &ast::Expr) -> ControlFlow<Self::Break> {
                if let ast::Expr::BinaryOp { left, op, right } = expr {
                    if let ast::BinaryOperator::Eq = op &&
                       let (
                                ast::Expr::CompoundIdentifier(ids),
                                ast::Expr::Value(ast::Value::SingleQuotedString(target)),
                            ) = (left.as_ref(), right.as_ref()) {
                            tracing::debug!("Found table requirement: {:?}:{}", ids, target);

                            let alias = ids.first().unwrap().value.as_str();
                            let condition = ids.last().unwrap().value.as_str();

                            if condition != "name" {
                                return ControlFlow::Continue(());
                            }

                            let table = self.analyzer.lookup_alias(alias);

                            if let Some(table) = table
                            && let Ok(table) = table::Tables::from_str(table.as_str()) {
                                self.tables.push((table, target.to_string()));
                        }
                    }
                }

                ControlFlow::Continue(())
            }
        }

        let mut visitor = Visitor {
            analyzer: self,
            tables: Vec::new(),
        };

        query.visit(&mut visitor);

        self.tables = visitor.tables;

        Ok(())
    }

    pub fn analyze_query(mut self, query: &ast::Query) -> Result<(), AnalyzeError> {
        self.parse_table_and_alias(query.body.as_ref())?;
        self.parse_tables(query)?;

        tracing::debug!("Aliases: {:?}", self.aliases);
        tracing::debug!("Tables: {:?}", self.tables);

        Ok(())
    }
}

pub fn analyse_query(query: &ast::Query) {
    let analyzer = Analyzer::new();
    let _requirement = analyzer.analyze_query(query);
}
