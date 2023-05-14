use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use gluesql::prelude::{Key, Row, Value};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::api::Metric;

use super::{
    active_dates_and_times, bus_factor, change_request, code_change_lines, issue, network,
    new_contributors,
};

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
    ActiveDatesAndTimes,
    Stars,
    TechnicalFork,
    Participants,
    NewContributors,
    NewContributorsDetail,
    InactiveContributors,
    BusFactor,
    BusFactorDetail,
    Issues,
    IssuesNew,
    IssuesClosed,
    IssueComments,
    IssueResponseTime,
    IssueResolutionDuration,
    IssueAge,
    CodeChangeLines,
    CodeChangeLinesAdd,
    CodeChangeLinesRemove,
    CodeChangeLinesSum,
    ChangeRequests,
    ChangeRequestsOpen,
    ChangeRequestsAccepted,
    ChangeRequestsReviews,
    ChangeRequestResponseTime,
    ChangeRequestResolutionDuration,
    ChangeRequestAge,
    RepoNetwork,
    DeveloperNetwork,
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
        match self {
            TableType::OpenRank
            | TableType::Activity
            | TableType::Attention
            | TableType::ActiveDatesAndTimes
            | TableType::Stars
            | TableType::TechnicalFork
            | TableType::Participants
            | TableType::NewContributors
            | TableType::NewContributorsDetail
            | TableType::InactiveContributors
            | TableType::BusFactor
            | TableType::BusFactorDetail
            | TableType::Issues
            | TableType::IssuesNew
            | TableType::IssuesClosed
            | TableType::IssueComments
            | TableType::IssueResponseTime
            | TableType::IssueResolutionDuration
            | TableType::IssueAge
            | TableType::CodeChangeLines
            | TableType::CodeChangeLinesAdd
            | TableType::CodeChangeLinesRemove
            | TableType::CodeChangeLinesSum
            | TableType::ChangeRequests
            | TableType::ChangeRequestsOpen
            | TableType::ChangeRequestsAccepted
            | TableType::ChangeRequestsReviews
            | TableType::ChangeRequestResponseTime
            | TableType::ChangeRequestResolutionDuration
            | TableType::ChangeRequestAge
            | TableType::RepoNetwork
            | TableType::DeveloperNetwork => true,
            _ => false,
        }
    }

    pub fn support_user_table(&self) -> bool {
        match self {
            TableType::OpenRank | TableType::Activity => true,
            _ => false,
        }
    }

    pub fn as_repo_metric(&self) -> Option<Metric> {
        let metric = match self {
            TableType::OpenRank => Metric::Repo(crate::api::RepositoryMetric::OpenRank),
            TableType::Activity => Metric::Repo(crate::api::RepositoryMetric::Activity),
            TableType::Attention => Metric::Repo(crate::api::RepositoryMetric::Attention),
            TableType::ActiveDatesAndTimes => {
                Metric::Repo(crate::api::RepositoryMetric::ActiveDatesAndTimes)
            }
            TableType::Stars => Metric::Repo(crate::api::RepositoryMetric::Stars),
            TableType::TechnicalFork => Metric::Repo(crate::api::RepositoryMetric::TechnicalFork),
            TableType::Participants => Metric::Repo(crate::api::RepositoryMetric::Participants),
            TableType::NewContributors => {
                Metric::Repo(crate::api::RepositoryMetric::NewContributors)
            }
            TableType::NewContributorsDetail => {
                Metric::Repo(crate::api::RepositoryMetric::NewContributorsDetail)
            }
            TableType::InactiveContributors => {
                Metric::Repo(crate::api::RepositoryMetric::InactiveContributors)
            }
            TableType::BusFactor => Metric::Repo(crate::api::RepositoryMetric::BusFactor),
            TableType::BusFactorDetail => {
                Metric::Repo(crate::api::RepositoryMetric::BusFactorDetail)
            }
            TableType::Issues => Metric::Custom,
            TableType::IssuesNew => Metric::Repo(crate::api::RepositoryMetric::IssuesNew),
            TableType::IssuesClosed => Metric::Repo(crate::api::RepositoryMetric::IssuesClosed),
            TableType::IssueComments => Metric::Repo(crate::api::RepositoryMetric::IssueComments),
            TableType::IssueResponseTime => {
                Metric::Repo(crate::api::RepositoryMetric::IssueResponseTime)
            }
            TableType::IssueResolutionDuration => {
                Metric::Repo(crate::api::RepositoryMetric::IssueResolutionDuration)
            }
            TableType::IssueAge => Metric::Repo(crate::api::RepositoryMetric::IssueAge),
            TableType::CodeChangeLines => Metric::Custom,
            TableType::CodeChangeLinesAdd => {
                Metric::Repo(crate::api::RepositoryMetric::CodeChangeLinesAdd)
            }
            TableType::CodeChangeLinesRemove => {
                Metric::Repo(crate::api::RepositoryMetric::CodeChangeLinesRemove)
            }
            TableType::CodeChangeLinesSum => {
                Metric::Repo(crate::api::RepositoryMetric::CodeChangeLinesSum)
            }
            TableType::ChangeRequests => Metric::Custom,
            TableType::ChangeRequestsOpen => {
                Metric::Repo(crate::api::RepositoryMetric::ChangeRequests)
            }
            TableType::ChangeRequestsAccepted => {
                Metric::Repo(crate::api::RepositoryMetric::ChangeRequestsAccepted)
            }
            TableType::ChangeRequestsReviews => {
                Metric::Repo(crate::api::RepositoryMetric::ChangeRequestsReviews)
            }
            TableType::ChangeRequestResponseTime => {
                Metric::Repo(crate::api::RepositoryMetric::ChangeRequestResponseTime)
            }
            TableType::ChangeRequestResolutionDuration => {
                Metric::Repo(crate::api::RepositoryMetric::ChangeRequestResolutionDuration)
            }
            TableType::ChangeRequestAge => {
                Metric::Repo(crate::api::RepositoryMetric::ChangeRequestAge)
            }
            TableType::RepoNetwork => Metric::Repo(crate::api::RepositoryMetric::RepoNetwork),
            TableType::DeveloperNetwork => {
                Metric::Repo(crate::api::RepositoryMetric::DeveloperNetwork)
            }
            _ => return None,
        };

        Some(metric)
    }

    pub fn as_user_metric(&self) -> Option<Metric> {
        let metric = match self {
            TableType::OpenRank => Metric::User(crate::api::UserMetric::OpenRank),
            TableType::Activity => Metric::User(crate::api::UserMetric::Activity),
            TableType::DeveloperNetwork => Metric::User(crate::api::UserMetric::DeveloperNetwork),
            TableType::RepoNetwork => Metric::User(crate::api::UserMetric::RepoNetwork),
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

    pub async fn fetch_data(&self) -> Result<Vec<(Key, Row)>, DataFetchError> {
        match self.r#type {
            TableType::OpenRank
            | TableType::Activity
            | TableType::Attention
            | TableType::Stars
            | TableType::TechnicalFork
            | TableType::Participants
            | TableType::NewContributors
            | TableType::InactiveContributors
            | TableType::BusFactor
            | TableType::IssuesNew
            | TableType::IssuesClosed
            | TableType::IssueComments
            | TableType::CodeChangeLinesAdd
            | TableType::CodeChangeLinesRemove
            | TableType::CodeChangeLinesSum
            | TableType::ChangeRequestsOpen
            | TableType::ChangeRequestsAccepted
            | TableType::ChangeRequestsReviews => {
                common_fetch_data(&self.owner, &self.metric).await
            }
            TableType::ActiveDatesAndTimes => {
                active_dates_and_times::fetch_data(&self.owner, &self.metric).await
            }
            TableType::NewContributorsDetail => {
                new_contributors::fetch_detail_data(&self.owner, &self.metric).await
            }
            TableType::BusFactorDetail => {
                bus_factor::fetch_detail_data(&self.owner, &self.metric).await
            }
            TableType::Issues => issue::fetch_data(&self.owner).await,
            TableType::IssueResponseTime
            | TableType::IssueResolutionDuration
            | TableType::IssueAge => issue::fetch_detail_data(&self.owner, &self.metric).await,
            TableType::CodeChangeLines => code_change_lines::fetch_data(&self.owner).await,
            TableType::ChangeRequests => change_request::fetch_data(&self.owner).await,
            TableType::ChangeRequestResponseTime
            | TableType::ChangeRequestResolutionDuration
            | TableType::ChangeRequestAge => {
                change_request::fetch_detail_data(&self.owner, &self.metric).await
            }
            TableType::RepoNetwork | TableType::DeveloperNetwork => {
                network::fetch_data(&self.owner, &self.metric).await
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
        .map(|(month, value)| {
            let row = Row(vec![
                Value::Str(owner.to_string()),
                Value::Str(month.clone()),
                Value::F64(value.clone()),
            ]);

            let key = Key::Str(month.clone());

            (key, row)
        })
        .collect::<Vec<_>>();

    Ok(items)
}
