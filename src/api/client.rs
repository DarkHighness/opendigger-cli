use crate::api::{
    model::{Message, MessageRequestPayload, MessageResponsePayload, MessageRole},
    ApiError, Metric,
};
use std::sync::Arc;

pub static ONEAPI_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
pub static ONEAPI_PROMPT: &str = "I want you to act as a professional SQL engineer. The database contains tables named 'Activity', 'Attention', 'Openrank', 'Stars', 'TechnicalFork','Participants','NewContributors','InactiveContributors','BusFactor','Issues','IssuesNew','IssuesClosed','IssueComments','CodeChangeLinesAdd','CodeChangeLinesRemove','CodeChangeLinesSum','ChangeRequestsOpen','ChangeRequestsAccepted' and 'ChangeRequestsReviews'. These tables have three columns named 'name', 'month' and 'value'. 'ActiveDatesAndTimes' table has columns 'name', 'date', 'hour' and 'value'. 'NewContributorsDetail' and 'BusFactorDetail' tables have four columns named 'name', 'month', 'user' and 'value'. 'IssueResponseTime', 'IssueResolutionDuration', 'ChangeRequestResolutionDuration', 'ChangeRequestAge', 'IssueAge' and 'ChangeRequestResponseTime' tables have eight columns named 'name', 'month', 'avg', 'Q0', 'Q1', 'Q2', 'Q3' and 'Q4'. 'DeveloperNetwork' and 'RepoNetwork' tables have six columns named 'owner', 'from', 'from_weight', 'to', 'to_weight' and 'weight'. Your  Task is to generate the SQL Query I ask and reply the sql statement with only select statement in a single code block, and nothing else. Do not write explanations.";

pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
    cache: quick_cache::sync::Cache<Box<str>, Arc<dyn std::any::Any + Send + Sync>>,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Result<Self, ApiError> {
        let client = reqwest::ClientBuilder::new()
            .gzip(true)
            .deflate(true)
            .connect_timeout(std::time::Duration::from_secs(3))
            .redirect(reqwest::redirect::Policy::default())
            .https_only(true)
            .use_rustls_tls()
            .build()?;

        let cache = quick_cache::sync::Cache::new(32);

        Ok(Self {
            base_url: base_url.to_string(),
            client,
            cache,
        })
    }

    pub async fn bytes(&self, name: &str, r#type: Metric) -> Result<bytes::Bytes, ApiError> {
        tracing::debug!("Fetching raw data from {}:{}", name, r#type.as_ref());

        let url = format!("{}/{}/{}.json", self.base_url, name, r#type.as_ref());
        let response = self.client.get(&url).send().await?;
        let status = response.status();

        let data = match status {
            _ if status.is_success() => response.bytes().await.map_err(ApiError::RequestError)?,
            http::status::StatusCode::NOT_FOUND => return Err(ApiError::DataNotFound(url)),
            _ => return Err(ApiError::BadRequest(status)),
        };

        tracing::debug!("Fetched raw data from {}:{}", name, r#type.as_ref());

        Ok(data)
    }

    pub async fn get<T>(&self, owner: &str, r#type: Metric) -> Result<T, ApiError>
    where
        for<'a> T: serde::Deserialize<'a> + Send + Sync + Clone + 'static,
    {
        tracing::debug!("Fetching data from {}:{}", owner, r#type.as_ref());

        let url = format!("{}/{}/{}.json", self.base_url, owner, r#type.as_ref());
        if let Some(data) = self
            .cache
            .get(url.as_str())
            .and_then(|data| data.downcast::<T>().ok())
        {
            tracing::debug!("Fetched data from cache {}:{}", owner, r#type.as_ref());

            return Ok(data.as_ref().clone());
        }

        let response = self.client.get(&url).send().await?;
        let status = response.status();

        let data: T = match status {
            _ if status.is_success() => response.json::<T>().await?,
            http::status::StatusCode::NOT_FOUND => return Err(ApiError::DataNotFound(url)),
            _ => return Err(ApiError::BadRequest(status)),
        };
        let data = Arc::new(data);

        self.cache.insert(url.into(), data.clone());

        tracing::debug!("Fetched data from {}:{}", owner, r#type.as_ref());

        Ok(data.as_ref().clone())
    }

    pub async fn chatgpt(&self, query: &str) -> Result<String, ApiError> {
        tracing::debug!("Fetching chatgpt response for query: {}", query);

        let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| ApiError::ApiKeyNotSet)?;

        let payload = MessageRequestPayload {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                Message {
                    role: MessageRole::System,
                    content: ONEAPI_PROMPT.to_string()
                },
                Message {
                    role: MessageRole::User,
                    content: format!("My first query is: {}", query)
                }
            ],
        };

        let body: MessageResponsePayload = self
            .client
            .post(ONEAPI_ENDPOINT)
            .bearer_auth(&api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        tracing::debug!("ChatGPT response: {:?}", body);

        let content = body.choices.into_iter().next().map(|content| {
            let content = content.message.content;

            if content.contains("```") {
                content
                    .trim()
                    .lines()
                    .skip_while(|line| *line != "```")
                    .skip(1)
                    .take_while(|line| *line != "```")
                    .map(|line| line.trim())
                    .collect::<Vec<&str>>()
                    .join("\n")
            } else {
                content.trim().to_string()
            }
        });

        if let Some(content) = content {
            tracing::debug!(
                "Fetched chatgpt response for query: {}, sql: {}",
                query,
                content
            );

            Ok(content)
        } else {
            Err(ApiError::UnexpectedResponse)
        }
    }
}
