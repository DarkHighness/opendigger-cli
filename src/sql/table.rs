use crate::sql::storage::tables::StorageTable;
use crate::sql::storage::StorageBuildError;
use once_cell::sync::Lazy;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum::IntoEnumIterator;

use super::storage;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TableEntry {
    pub r#type: TableTypes,
    pub owner: String,
}

#[derive(Debug, Clone)]
pub struct AggregateTableEntry {
    pub r#type: TableTypes,
    pub owners: Vec<String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TableTypes {
    Repo(RepoTables),
    User(UserTables),
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
pub enum RepoTables {
    #[strum(serialize = "Openrank")]
    OpenRank,
    Activity,
    Attention,
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
pub enum UserTables {}

pub static SUPPORTED_TABLE_NAMES: Lazy<Vec<&'static str>> =
    Lazy::new(TableEntry::supported_table_names);

impl TableEntry {
    pub fn try_build_from_type_and_owner(r#type: &str, owner: &str) -> Option<TableEntry> {
        let is_repo_type = owner.contains('/');
        let table_type = if is_repo_type {
            TableTypes::Repo(RepoTables::from_str(r#type).ok()?)
        } else {
            TableTypes::User(UserTables::from_str(r#type).ok()?)
        };

        Some(TableEntry {
            r#type: table_type,
            owner: owner.to_string(),
        })
    }

    pub fn supported_table_names() -> Vec<&'static str> {
        RepoTables::iter()
            .map(|t| t.into())
            .chain(UserTables::iter().map(|t| t.into()))
            .collect::<Vec<_>>()
    }

    pub fn is_repo_table(&self) -> bool {
        matches!(self.r#type, TableTypes::Repo { .. })
    }

    pub fn is_user_table(&self) -> bool {
        matches!(self.r#type, TableTypes::User { .. })
    }

    pub fn get_owner(&self) -> &str {
        &self.owner
    }
}

impl Display for TableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.r#type {
            TableTypes::Repo(table) => write!(f, "{}.{}", self.owner, table.as_ref()),
            TableTypes::User(table) => write!(f, "{}.{}", self.owner, table.as_ref()),
        }
    }
}

impl TableTypes {
    async fn to_storage_table(
        &self,
        owners: &Vec<String>,
    ) -> Result<Box<dyn StorageTable>, StorageBuildError> {
        let table = match self {
            TableTypes::Repo(table) => match table {
                RepoTables::OpenRank => Box::new(
                    storage::tables::OpenRankTable::build(owners, crate::api::Type::Repository)
                        .await?,
                ),
                RepoTables::Activity => unimplemented!(),
                RepoTables::Attention => unimplemented!(),
            },
            TableTypes::User(_table) => unimplemented!(),
        };

        Ok(table)
    }
}

impl AggregateTableEntry {
    pub async fn to_storage_table(&self) -> Result<Box<dyn StorageTable>, StorageBuildError> {
        self.r#type.to_storage_table(&self.owners).await
    }
}
