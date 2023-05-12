use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use gluesql::prelude::{Key, Row, Value};

use crate::api::Metric;

#[derive(Debug, thiserror::Error)]
pub enum DataFetchError {
    #[error(transparent)]
    ApiError(#[from] crate::api::ApiError),
    #[error("Unsupported table type: {0}")]
    InvalidRepositoryMetric(TableType),
    #[error("Unsupported table type: {0}")]
    InvalidUserMetric(TableType),
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
    #[strum(serialize = "Openrank")]
    OpenRank,
    Activity,
    Attention,
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

impl TableOwner {
    pub fn new(owner: &str) -> Option<TableOwner> {
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
        match self {
            TableType::OpenRank => true,
            TableType::Activity => true,
            TableType::Attention => true,
        }
    }

    pub fn support_user_table(&self) -> bool {
        match self {
            TableType::OpenRank => true,
            TableType::Activity => true,
            TableType::Attention => false,
        }
    }

    pub fn as_repo_metric(&self) -> Option<Metric> {
        match self {
            TableType::OpenRank => Some(Metric::Repo(crate::api::RepositoryMetric::OpenRank)),
            TableType::Activity => Some(Metric::Repo(crate::api::RepositoryMetric::Activity)),
            TableType::Attention => Some(Metric::Repo(crate::api::RepositoryMetric::Attention)),
        }
    }

    pub fn as_user_metric(&self) -> Option<Metric> {
        match self {
            TableType::OpenRank => Some(Metric::User(crate::api::UserMetric::OpenRank)),
            TableType::Activity => Some(Metric::User(crate::api::UserMetric::Activity)),
            TableType::Attention => None,
        }
    }
}

impl TableEntry {
    pub fn new(r#type: &str, owner: &str) -> Option<TableEntry> {
        let table_type = TableType::from_str(r#type).ok()?;
        let table_owner = TableOwner::new(owner)?;

        let table_metric = if table_owner.is_repository() && table_type.support_repository_table() {
            table_type.as_repo_metric().unwrap()
        } else if table_owner.is_user() && table_type.support_user_table() {
            table_type.as_user_metric().unwrap()
        } else {
            return None;
        };

        Some(TableEntry {
            r#type: table_type,
            owner: table_owner,
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

    pub async fn fetch_data(&self) -> Result<Vec<(Key, Row)>, DataFetchError> {
        match self.r#type {
            TableType::OpenRank | TableType::Activity | TableType::Attention => {
                common_fetch_data(&self.owner, &self.metric).await
            }
        }
    }
}

impl Display for TableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.r#type.as_ref(), self.owner)
    }
}

async fn common_fetch_data(
    owner: &TableOwner,
    metric: &Metric,
) -> Result<Vec<(Key, Row)>, DataFetchError> {
    let api = crate::api::get();
    let data = api
        .get::<BTreeMap<String, f64>>(owner.to_string().as_str(), metric.clone())
        .await?;

    let items = data
        .iter()
        .map(|(time, value)| {
            let row = Row(vec![
                Value::Str(owner.to_string()),
                Value::Str(time.clone()),
                Value::F64(*value),
            ]);

            let key = Key::Str(time.clone());

            (key, row)
        })
        .collect::<Vec<_>>();

    Ok(items)
}
