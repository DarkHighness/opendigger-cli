use std::fmt::{Display, Formatter};
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;
use petgraph::Graph;
use petgraph::Undirected;

use crate::api::Metric;

use super::{
    openrank_network, generic_network, 
    generic_network::NodeData, openrank_network::OpenRankNode
};

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error(transparent)]
    ApiError(#[from] crate::api::ApiError),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TableOwner {
    Repository(String, String),
    User(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TableEntry {
    pub r#type: TableType,
    pub owner: TableOwner,
    pub metric: Metric,
}

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    strum::EnumString,
    strum::EnumIter,
    strum::IntoStaticStr,
    strum::AsRefStr,
)]
pub enum TableType {
    RepoNetwork,
    DeveloperNetwork,
    SpecialCircumstances,
    
}

impl Display for TableOwner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TableOwner::Repository(owner, name) => write!(f, "{}/{}", owner, name),
            TableOwner::User(owner) => write!(f, "{}", owner),
        }
    }
}

impl Display for TableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

pub static YEAR_MONTH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\d{4})-(\d{2})"#).unwrap());
pub static YEAR_MONTH_DAY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(\d{4})-(\d{2})-(\d{2})"#).unwrap());

impl TableOwner {
    pub fn new(owner: &str) -> Option<TableOwner> {
        // Blacklist some invalid owner names
        match (
            YEAR_MONTH_REGEX.is_match(owner),
            YEAR_MONTH_DAY_REGEX.is_match(owner),
        ) {
            (false, false) => {}
            _ => return None,
        }

        let parts = owner.split('/').collect::<Vec<_>>();
      
        match parts.len() {
            1 => Some(TableOwner::User(parts[0].to_string())),
            2 => Some(TableOwner::Repository(
                parts[0].to_string(),
                parts[1].to_string(),
            )),
            _ => None,
        }
        

    }

    pub fn is_repository(&self) -> bool {
        matches!(self, TableOwner::Repository(_, _))
    }

    pub fn is_user(&self) -> bool {
        matches!(self, TableOwner::User(_))
    }
}

impl TryFrom<&str> for TableOwner {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value).ok_or("Invalid table owner")
    }
}

impl TableType {
    pub fn support_repository_table(&self) -> bool {
        // All table types support repository table
        true
    }

    pub fn support_user_table(&self) -> bool {
        matches!(
            self,
            TableType::DeveloperNetwork
                | TableType::RepoNetwork
        )
    }

    pub fn as_repo_metric(&self) -> Option<Metric> {
        let metric = match self {
            TableType::SpecialCircumstances => Metric::SpecialCircumstances,
            TableType::RepoNetwork => crate::api::RepositoryMetric::RepoNetwork.into(),
            TableType::DeveloperNetwork => crate::api::RepositoryMetric::DeveloperNetwork.into(),
        };

        Some(metric)
    }
    pub fn as_user_metric(&self) -> Option<Metric> {
        let metric = match self {
            TableType::RepoNetwork => crate::api::RepositoryMetric::RepoNetwork.into(),
            TableType::DeveloperNetwork => crate::api::RepositoryMetric::DeveloperNetwork.into(),
            _ => return None,
        };

        Some(metric)
    }
}

impl TableEntry {
    pub fn parse(r#type: &str, owner: &str) -> Option<TableEntry> {
        let table_type = TableType::from_str(r#type).ok()?;
        let table_owner = TableOwner::new(owner)?;

        Self::new(table_type, table_owner)
    }

    pub fn new(r#type: TableType, owner: TableOwner) -> Option<TableEntry> {
        let table_metric = if owner.is_repository() && r#type.support_repository_table() {
            r#type.as_repo_metric().unwrap()
        } else if owner.is_user() && r#type.support_user_table() {
            r#type.as_user_metric().unwrap()
        } else {
            tracing::warn!(
                "Invalid table type: {} for owner: {}",
                r#type,
                owner.to_string()
            );

            return None;
        };

        Some(TableEntry {
            r#type,
            owner,
            metric: table_metric,
        })
    }

    pub fn support_repository_table(&self) -> bool {
        self.r#type.support_repository_table()
    }

    pub fn support_user_table(&self) -> bool {
        self.r#type.support_user_table()
    }

    pub fn get_owner(&self) -> &TableOwner {
        &self.owner
    }

    pub fn get_table_type(&self) -> &TableType {
        &self.r#type
    }

    pub fn get_metric_type(&self) -> &Metric {
        &self.metric
    }

    pub async fn fetch_generic_network_data(&self) -> Result<Graph<NodeData, f64, Undirected>, DataError> {
        generic_network::fetch_network_data(&self.owner, &self.metric).await
    }
    pub async fn fetch_openrank_network_data(&self) -> Result<Graph<OpenRankNode, f64, Undirected>, DataError> {
        openrank_network::fetch_network_data(&self.owner, &self.metric).await
    }
}

impl Display for TableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.r#type.as_ref(), self.owner)
    }
}

