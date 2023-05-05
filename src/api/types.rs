use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::AsRefStr, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Target {
    Repo,
    User,
}

#[derive(Debug, Copy, Clone)]
pub enum Metric {
    Repo(RepoMetric),
    User(UserMetric),
}

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    strum::AsRefStr,
    strum::IntoStaticStr,
    strum::EnumString,
    strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RepoMetric {
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
    ChangeRequestAccepted,
    ChangeRequestReviews,
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
        }
    }
}

impl RepoMetric {
    pub fn available_types() -> Vec<&'static str> {
        RepoMetric::iter().map(|t| t.into()).collect::<Vec<_>>()
    }
}

impl UserMetric {
    pub fn available_types() -> Vec<&'static str> {
        UserMetric::iter().map(|t| t.into()).collect::<Vec<_>>()
    }
}
