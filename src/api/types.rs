use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::AsRefStr, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Type {
    Repository,
    User,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Metric {
    Repo(RepositoryMetric),
    User(UserMetric),
    Custom,
}

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    strum::AsRefStr,
    strum::IntoStaticStr,
    strum::EnumString,
    strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RepositoryMetric {
    #[serde(rename = "openrank")]
    #[strum(serialize = "openrank")]
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
    IssuesNew,
    IssuesClosed,
    IssueComments,
    IssueResponseTime,
    IssueResolutionDuration,
    IssueAge,
    CodeChangeLinesAdd,
    CodeChangeLinesRemove,
    CodeChangeLinesSum,
    ChangeRequests,
    ChangeRequestsAccepted,
    ChangeRequestsReviews,
    ChangeRequestResponseTime,
    ChangeRequestResolutionDuration,
    ChangeRequestAge,
    DeveloperNetwork,
    RepoNetwork,
}

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    strum::AsRefStr,
    strum::IntoStaticStr,
    strum::EnumString,
    strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum UserMetric {
    #[serde(rename = "openrank")]
    #[strum(serialize = "openrank")]
    OpenRank,
    Activity,
    DeveloperNetwork,
    RepoNetwork,
}

impl AsRef<str> for Metric {
    fn as_ref(&self) -> &str {
        match self {
            Metric::Repo(metric) => metric.as_ref(),
            Metric::User(metric) => metric.as_ref(),
            Metric::Custom => "custom",
        }
    }
}

impl From<RepositoryMetric> for Metric {
    fn from(metric: RepositoryMetric) -> Self {
        Metric::Repo(metric)
    }
}

impl From<UserMetric> for Metric {
    fn from(metric: UserMetric) -> Self {
        Metric::User(metric)
    }
}

impl Metric {
    pub fn is_repo_metric(&self) -> bool {
        matches!(self, Metric::Repo(_))
    }

    pub fn is_user_metric(&self) -> bool {
        matches!(self, Metric::User(_))
    }
}

impl RepositoryMetric {
    pub fn available_types() -> Vec<&'static str> {
        RepositoryMetric::iter()
            .map(|t| t.into())
            .collect::<Vec<_>>()
    }
}


impl UserMetric {
    pub fn available_types() -> Vec<&'static str> {
        UserMetric::iter().map(|t| t.into()).collect::<Vec<_>>()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
