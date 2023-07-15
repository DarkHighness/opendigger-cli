use crate::command::Commands;
use crate::networkgraph::analyze;
use crate::sql::Storage;
use crate::ui::{TableUI, UIMode};
use anyhow::Context;
use gluesql::core::ast;
use gluesql::prelude::{Payload, PayloadVariable};
use once_cell::sync::Lazy;
use regex::Regex;

pub use util::{execute_sql_queries, execute_sql_query};
pub use value::{translate_from_gluesql_value, Value};

mod util;
mod value;

#[derive(Debug)]
pub struct Engine {}

#[derive(Debug, thiserror::Error)]
pub enum EngineExecutionError {
    #[error(transparent)]
    Api(#[from] crate::api::ApiError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Storage(#[from] crate::sql::StorageError),
    #[error(transparent)]
    Engine(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    UI(#[from] crate::ui::UIError),
    #[error(transparent)]
    Report(#[from] crate::report::ReportError),
}

impl Engine {
    fn new() -> Self {
        Engine {}
    }

    async fn download_data(
        &self,
        name: &str,
        metric: crate::api::Metric,
        output_file: Option<String>,
    ) -> Result<(), EngineExecutionError> {
        let output_file = output_file
            .unwrap_or_else(|| format!("repo-{}-{}.json", name.replace('/', "-"), metric.as_ref()));

        let repo_data = crate::api::get().bytes(name, metric).await?;

        tokio::fs::write(output_file, repo_data).await?;

        Ok(())
    }

    async fn execute_sql_query(
        &self,
        statements: Vec<ast::Statement>,
        strategy: Box<dyn crate::sql::StorageStrategy>,
        output_file: Option<String>,
        ui_mode: UIMode,
    ) -> Result<(), EngineExecutionError> {
        let storage = Storage::build_from_strategy(strategy).await?;

        tracing::debug!("Storage: {:?}", storage);

        let mut engine = gluesql::prelude::Glue::new(storage);

        for statement in statements {
            let payload = engine
                .execute_stmt(&statement)
                .map_err(|e| EngineExecutionError::Engine(Box::new(e)))?;

            tracing::debug!("Payload: {:?}", payload);

            match payload {
                Payload::Select { labels, rows } => {
                    let rows = rows
                        .into_iter()
                        .map(|row| {
                            row.into_iter()
                                .map(|value| {
                                    translate_from_gluesql_value(value)
                                        .context("Failed to translate GlueSQL value to Value")
                                        .unwrap()
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();

                    if let Some(output_file) = output_file.as_ref() {
                        let rows = rows
                            .iter()
                            .map(|row| {
                                row.iter()
                                    .map(|value| value.to_string())
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>();
                        let output = crate::ui::render_csv(&labels, &rows);

                        tokio::fs::write(output_file, output).await?;

                        tracing::info!("Output written to {}", output_file);
                    }

                    let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    let title = format!(" Query Result: {} ", time);

                    TableUI::new(ui_mode, title, labels, rows)?.render()?;
                }
                Payload::ShowColumns(columns) => {
                    let rows = columns
                        .into_iter()
                        .map(|(name, typ)| vec![name.into(), typ.to_string().into()])
                        .collect::<Vec<_>>();

                    TableUI::new(
                        ui_mode,
                        "Columns".to_string(),
                        vec!["Column".to_string(), "Type".to_string()],
                        rows,
                    )?
                    .render()?;
                }
                Payload::ShowVariable(payload) => {
                    if let PayloadVariable::Tables(tables) = payload {
                        let rows = tables
                            .into_iter()
                            .map(|table| vec![table.into()])
                            .collect::<Vec<_>>();

                        TableUI::new(ui_mode, "Tables".to_string(), vec!["name".into()], rows)?
                            .render()?;
                    } else {
                        tracing::error!("Invalid payload: {:?}", payload);
                    }
                }
                _ => {
                    tracing::error!("Invalid payload: {:?}", payload);
                }
            }
        }

        Ok(())
    }

    async fn execute_cypher_query(
        &self,
        statements: String,
        output_file: Option<String>,
        ui_mode: UIMode,
    ) -> Result<(), EngineExecutionError> {
        let match_query1 = Regex::new(r#"MATCH \(n:Node \{value: '(.+?)'\}\)-\[]-\(neighbor\).*where n\.owner='(.+?)' and n\.metric='(.+?)'"#).unwrap();
        let match_query2 =
            Regex::new(r"RETURN (\w+),*\s*(\w+)='(\w+)' and (\w+)='(\w+)'.*LIMIT 1").unwrap();
        let match_query3 = Regex::new(
            r"MATCH \(n:Node \{value: '(?P<a>[^']+)'\}\) WHERE n.owner = '(?P<b>[^']+)'\s+AND n.metric = '(?P<c>[^']+)'\s+RETURN n"
        )
        .unwrap();
      
        if let Some(captures) = match_query1.captures(statements.as_str()) {
            if let Some(node) = captures.get(1) {
                if let Some(owner) = captures.get(2) {
                    if let Some(metric) = captures.get(3) {
                        let data =
                            analyze::get_neighbors(metric.as_str(), owner.as_str(), node.as_str())
                                .await;
                        match data {
                            Ok(data) => {
                                let varible = data
                                    .into_iter()
                                    .map(|data| vec![data.into()])
                                    .collect::<Vec<_>>();

                                TableUI::new(
                                    ui_mode,
                                    "Neighbors".to_string(),
                                    vec!["name".into()],
                                    varible,
                                )?
                                .render()?;
                            }
                            Err(e) => println!("error: {:?}", e),
                        }
                    }
                }
            }
        } else if let Some(captures) = match_query2.captures(statements.as_str()) {
            let node = captures.get(1).map_or("", |m| m.as_str());
            let owner = captures.get(3).map_or("", |m| m.as_str());
            let metric = captures.get(5).map_or("", |m| m.as_str());
            let data = analyze::get_max_neighbor(metric, owner, node).await;
            match data {
                Ok(data) => {
                    let varible = vec![vec![data.to_string().into()]];

                    TableUI::new(
                        ui_mode,
                        "Max Neighbor".to_string(),
                        vec!["name".into()],
                        varible,
                    )?
                    .render()?;
                }
                Err(e) => println!("error: {:?}", e),
            }
        } else if let Some(captures) = match_query3.captures(statements.as_str()) {
            let node = captures.get(1).map_or("", |m| m.as_str());
            let owner = captures.get(2).map_or("", |m| m.as_str());
            let metric = captures.get(3).map_or("", |m| m.as_str());
            let data = analyze::get_node_value(metric, owner, node).await;
            match data {
                Ok(data) => {
                    let varible = vec![vec![data.to_string().into()]];

                    TableUI::new(
                        ui_mode,
                        "Node Value".to_string(),
                        vec!["name".into()],
                        varible,
                    )?
                    .render()?;
                }
                Err(e) => println!("error: {:?}", e),
            }
        } else {
            println!("Please enter supported Cypher query statement");
        }

        Ok(())
    }

    pub async fn execute_command(&self, command: Commands) -> Result<(), EngineExecutionError> {
        tracing::debug!("Executing command: {:?}", command);

        match command {
            Commands::Download(command) => {
                self.download_data(&command.name, command.metric, command.output_file)
                    .await?;
            }
            Commands::SqlQuery(command) => {
                self.execute_sql_query(
                    command.statements,
                    command.strategy,
                    command.output_file,
                    command.ui_mode,
                )
                .await?;
            }
            Commands::CypherQuery(command) => {
                self.execute_cypher_query(command.statements, command.output_file, command.ui_mode)
                    .await?;
            }
            Commands::Report(command) => {
                let owner = command.owner;
                if owner.contains('/') {
                    let mut report = crate::report::RepoOverview::new(owner);
                    report.generate_report().await?;

                    //crate::ui::RepoOverviewUI::new(report).run()?;
                }
            }
        }

        Ok(())
    }
}

pub static ENGINE: Lazy<Engine> = Lazy::new(Engine::new);
