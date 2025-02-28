use ntex::rt;
use ntex::http::{Client, StatusCode};
use ntex::http::client::{Connector, ClientResponse};

use nanocl_utils::io_error::FromIo;
use nanocl_utils::http_error::HttpError;
use nanocl_utils::http_client_error::HttpClientError;

/// Controller client
pub struct CtrlClient {
  /// Name of the controller eg: (ProxyRule)
  pub(crate) name: String,
  /// HTTP client
  pub(crate) client: Client,
  /// Base url
  pub(crate) base_url: String,
}

impl CtrlClient {
  /// Create a new controller client
  pub(crate) fn new(name: &str, url: &str) -> Self {
    let (client, url) = match url {
      url if url.starts_with("unix://") => {
        let url = url.to_string();
        let client = Client::build()
          .connector(
            Connector::default()
              .connector(ntex::service::fn_service(move |_| {
                let path = url.trim_start_matches("unix://").to_string();
                async move { Ok(rt::unix_connect(path).await?) }
              }))
              .timeout(ntex::time::Millis::from_secs(20))
              .finish(),
          )
          .timeout(ntex::time::Millis::from_secs(20))
          .finish();

        (client, "http://localhost")
      }
      url if url.starts_with("http://") || url.starts_with("https://") => {
        let client = Client::build().finish();
        (client, url)
      }
      _ => panic!("Invalid url: {}", url),
    };

    Self {
      client,
      name: name.to_string(),
      base_url: url.to_string(),
    }
  }

  /// Format url with base url
  fn format_url(&self, path: &str) -> String {
    format!("{}{}", self.base_url, path)
  }

  /// Check if the response is an API error
  async fn is_api_error(
    &self,
    res: &mut ClientResponse,
    status: &StatusCode,
  ) -> Result<(), HttpClientError> {
    if status.is_server_error() || status.is_client_error() {
      let body = res
        .json::<serde_json::Value>()
        .await
        .map_err(|err| err.map_err_context(|| self.name.to_string()))?;
      let msg = body["msg"].as_str().ok_or(HttpError {
        status: *status,
        msg: String::default(),
      })?;
      return Err(HttpClientError::HttpError(HttpError {
        status: *status,
        msg: format!("{}: {msg}", self.name),
      }));
    }
    Ok(())
  }

  /// Parse http response to json
  async fn res_json<T>(
    &self,
    res: &mut ClientResponse,
  ) -> Result<T, HttpClientError>
  where
    T: serde::de::DeserializeOwned,
  {
    let body = res
      .json::<T>()
      .await
      .map_err(|err| err.map_err_context(|| self.name.to_string()))?;
    Ok(body)
  }

  /// Call apply rule method on controller
  pub(crate) async fn apply_rule(
    &self,
    version: &str,
    name: &str,
    data: &serde_json::Value,
  ) -> Result<serde_json::Value, HttpClientError> {
    let mut res = self
      .client
      .put(self.format_url(&format!("/{version}/rules/{name}")))
      .send_json(data)
      .await
      .map_err(|err| err.map_err_context(|| self.name.to_string()))?;
    let status = res.status();
    self.is_api_error(&mut res, &status).await?;

    self.res_json(&mut res).await
  }

  /// Call delete rule method on controller
  pub(crate) async fn delete_rule(
    &self,
    version: &str,
    name: &str,
  ) -> Result<(), HttpClientError> {
    let mut res = self
      .client
      .delete(self.format_url(&format!("/{version}/rules/{name}")))
      .send()
      .await
      .map_err(|err| err.map_err_context(|| self.name.to_string()))?;
    let status = res.status();
    self.is_api_error(&mut res, &status).await?;

    Ok(())
  }
}
