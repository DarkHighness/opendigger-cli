use crate::api::{ApiError, Metric};
use std::sync::Arc;

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
        let url = format!("{}/{}/{}.json", self.base_url, name, r#type.as_ref());
        let response = self.client.get(&url).send().await?;
        let status = response.status();

        let data = match status {
            _ if status.is_success() => response.bytes().await.map_err(ApiError::ReqwestError)?,
            http::status::StatusCode::NOT_FOUND => return Err(ApiError::DataNotFound(url)),
            _ => return Err(ApiError::BadReqwestResponse(status)),
        };

        Ok(data)
    }

    async fn get<T>(&self, name: &str, r#type: Metric) -> Result<Arc<T>, ApiError>
    where
        for<'a> T: serde::Deserialize<'a> + Send + Sync + Clone + 'static,
    {
        let url = format!("{}/{}/{}.json", self.base_url, name, r#type.as_ref());
        if let Some(data) = self
            .cache
            .get(url.as_str())
            .and_then(|data| data.downcast::<T>().ok())
        {
            return Ok(data);
        }

        let response = self.client.get(&url).send().await?;
        let status = response.status();

        let data: T = match status {
            _ if status.is_success() => response.json::<T>().await?,
            http::status::StatusCode::NOT_FOUND => return Err(ApiError::DataNotFound(url.into())),
            _ => return Err(ApiError::BadReqwestResponse(status)),
        };
        let data = Arc::new(data);

        self.cache.insert(url.into(), data.clone());

        Ok(data)
    }
}
