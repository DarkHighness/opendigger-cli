use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, strum::AsRefStr)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize, strum::AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum UserMetricTypes {
    #[serde(rename = "openrank")]
    #[strum(serialize = "openrank")]
    OpenRank,
    Activity,
    DeveloperNetwork,
    RepoNetwork,
}
