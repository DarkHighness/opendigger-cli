use std::collections::BTreeMap;

use crate::api::RepositoryMetric;

use super::ReportError;

pub struct ReporOverview {
    pub owner: String,
    pub star_trend: Option<Vec<(String, i64)>>,
}

impl ReporOverview {
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            star_trend: None,
        }
    }

    pub async fn generate_report(&mut self) -> Result<(), ReportError> {
        let api = crate::api::get();
        let star_trend = api
            .get::<BTreeMap<String, i64>>(&self.owner, RepositoryMetric::Stars.into())
            .await?
            .iter()
            .map(|(date, value)| (date.clone(), *value))
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

        Ok(())
    }
}
