use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::AsRefStr, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TargetType {
    Repo,
    User,
}

pub trait MetricType {
    fn available_types() -> Vec<String>;
}

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, strum::AsRefStr, strum::EnumString, strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RepoMetricTypes {
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
    Debug, Copy, Clone, Serialize, Deserialize, strum::AsRefStr, strum::EnumString, strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum UserMetricTypes {
    #[serde(rename = "openrank")]
    #[strum(serialize = "openrank")]
    OpenRank,
    Activity,
    DeveloperNetwork,
    RepoNetwork,
}

impl MetricType for RepoMetricTypes {
    fn available_types() -> Vec<String> {
        RepoMetricTypes::iter()
            .map(|t| t.as_ref().into())
            .collect::<Vec<_>>()
    }
}

impl MetricType for UserMetricTypes {
    fn available_types() -> Vec<String> {
        UserMetricTypes::iter()
            .map(|t| t.as_ref().into())
            .collect::<Vec<_>>()
    }
}
