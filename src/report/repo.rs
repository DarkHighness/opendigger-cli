use std::collections::BTreeMap;

use crate::api::RepositoryMetric;

use super::ReportError;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NetworkGraph {
    nodes: Vec<(String, f64)>,
    edges: Vec<(String, String, f64)>,
}

pub struct RepoOverview {
    pub owner: String,
    pub star_trend: Option<Vec<(String, i64)>>,
    pub repo_network: Option<NetworkGraph>,
}

impl RepoOverview {
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            star_trend: None,
            repo_network: None,
        }
    }

    pub async fn generate_report(&mut self) -> Result<(), ReportError> {
        let api = crate::api::get();
        let star_trend = api
            .get::<BTreeMap<String, i64>>(&self.owner, RepositoryMetric::Stars.into())
            .await?
            .into_iter()
            .fold(Vec::new(), |state, element| {
                let mut state = state;
                if let Some((_, last_value)) = state.last() {
                    let (date, value) = element;
                    state.push((date, value + last_value));
                } else {
                    state.push(element);
                }
                state
            });

        self.star_trend = Some(star_trend);

        let _repo_network = api
            .get::<NetworkGraph>(&self.owner, RepositoryMetric::RepoNetwork.into())
            .await?;

        Ok(())
    }
}
