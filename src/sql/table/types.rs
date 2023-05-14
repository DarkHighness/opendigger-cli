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
        // All table types support repository table
        true
    }

    pub fn support_user_table(&self) -> bool {
        matches!(
            self,
            TableType::OpenRank
                | TableType::Activity
                | TableType::DeveloperNetwork
                | TableType::RepoNetwork
        )
    }

    pub fn as_repo_metric(&self) -> Option<Metric> {
        let metric = match self {
            TableType::OpenRank => crate::api::RepositoryMetric::OpenRank.into(),
            TableType::Activity => crate::api::RepositoryMetric::Activity.into(),
            TableType::Attention => crate::api::RepositoryMetric::Attention.into(),
            TableType::ActiveDatesAndTimes => {
                crate::api::RepositoryMetric::ActiveDatesAndTimes.into()
            }
            TableType::Stars => crate::api::RepositoryMetric::Stars.into(),
            TableType::TechnicalFork => crate::api::RepositoryMetric::TechnicalFork.into(),
            TableType::Participants => crate::api::RepositoryMetric::Participants.into(),
            TableType::NewContributors => crate::api::RepositoryMetric::NewContributors.into(),
            TableType::NewContributorsDetail => {
                crate::api::RepositoryMetric::NewContributorsDetail.into()
            }
            TableType::InactiveContributors => {
                crate::api::RepositoryMetric::InactiveContributors.into()
            }
            TableType::BusFactor => crate::api::RepositoryMetric::BusFactor.into(),
            TableType::BusFactorDetail => crate::api::RepositoryMetric::BusFactorDetail.into(),
            TableType::Issues => Metric::Custom,
            TableType::IssuesNew => crate::api::RepositoryMetric::IssuesNew.into(),
            TableType::IssuesClosed => crate::api::RepositoryMetric::IssuesClosed.into(),
            TableType::IssueComments => crate::api::RepositoryMetric::IssueComments.into(),
            TableType::IssueResponseTime => crate::api::RepositoryMetric::IssueResponseTime.into(),
            TableType::IssueResolutionDuration => {
                crate::api::RepositoryMetric::IssueResolutionDuration.into()
            }
            TableType::IssueAge => crate::api::RepositoryMetric::IssueAge.into(),
            TableType::CodeChangeLines => Metric::Custom,
            TableType::CodeChangeLinesAdd => {
                crate::api::RepositoryMetric::CodeChangeLinesAdd.into()
            }
            TableType::CodeChangeLinesRemove => {
                crate::api::RepositoryMetric::CodeChangeLinesRemove.into()
            }
            TableType::CodeChangeLinesSum => {
                crate::api::RepositoryMetric::CodeChangeLinesSum.into()
            }
            TableType::ChangeRequests => Metric::Custom,
            TableType::ChangeRequestsOpen => crate::api::RepositoryMetric::ChangeRequests.into(),
            TableType::ChangeRequestsAccepted => {
                crate::api::RepositoryMetric::ChangeRequestsAccepted.into()
            }
            TableType::ChangeRequestsReviews => {
                crate::api::RepositoryMetric::ChangeRequestsReviews.into()
            }
            TableType::ChangeRequestResponseTime => {
                crate::api::RepositoryMetric::ChangeRequestResponseTime.into()
            }
            TableType::ChangeRequestResolutionDuration => {
                crate::api::RepositoryMetric::ChangeRequestResolutionDuration.into()
            }
            TableType::ChangeRequestAge => crate::api::RepositoryMetric::ChangeRequestAge.into(),
            TableType::RepoNetwork => crate::api::RepositoryMetric::RepoNetwork.into(),
            TableType::DeveloperNetwork => crate::api::RepositoryMetric::DeveloperNetwork.into(),
        };

        Some(metric)
    }

    pub fn as_user_metric(&self) -> Option<Metric> {
        let metric = match self {
            TableType::OpenRank => crate::api::UserMetric::OpenRank.into(),
            TableType::Activity => crate::api::UserMetric::Activity.into(),
            TableType::DeveloperNetwork => crate::api::UserMetric::DeveloperNetwork.into(),
            TableType::RepoNetwork => crate::api::UserMetric::RepoNetwork.into(),
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

    pub async fn fetch_data(&self) -> Result<Vec<(Key, Row)>, DataError> {
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
            TableType::Issues => issue::fetch_combined_data(&self.owner).await,
            TableType::IssueResponseTime
            | TableType::IssueResolutionDuration
            | TableType::IssueAge => issue::fetch_detail_data(&self.owner, &self.metric).await,
            TableType::CodeChangeLines => code_change_lines::fetch_combined_data(&self.owner).await,
            TableType::ChangeRequests => change_request::fetch_combined_data(&self.owner).await,
            TableType::ChangeRequestResponseTime
            | TableType::ChangeRequestResolutionDuration
            | TableType::ChangeRequestAge => {
                change_request::fetch_detail_data(&self.owner, &self.metric).await
            }
            TableType::RepoNetwork | TableType::DeveloperNetwork => {
                network::fetch_network_data(&self.owner, &self.metric).await
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
) -> Result<Vec<(Key, Row)>, DataError> {
    let api = crate::api::get();
    let data = api
        .get::<BTreeMap<String, f64>>(owner.to_string().as_str(), *metric)
        .await?;

    let items = data
        .into_iter()
        .map(|(month, value)| {
            let row = Row(vec![
                Value::Str(owner.to_string()),
                Value::Str(month),
                Value::F64(value),
            ]);

            let key = Key::Str(owner.to_string());

            (key, row)
        })
        .collect::<Vec<_>>();

    Ok(items)
}
