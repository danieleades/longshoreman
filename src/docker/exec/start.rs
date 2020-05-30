use crate::{http_client::HttpClient, Result};
use serde::Serialize;

/// A request to create a new 'exec' instance
#[derive(Debug)]
pub struct Start<'a> {
    http_client: &'a HttpClient,
    id: &'a str,
    body: Body,
}

impl<'a> Start<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, id: &'a str) -> Self {
        let body = Body::default();
        Self {
            http_client,
            id,
            body,
        }
    }

    /// Consume the request builder and return the 'exec' instance id
    pub async fn send(self) -> Result<()> {
        let endpoint = format!("/exec/{}/start", self.id);
        self.http_client.post(endpoint).into_response().await?;

        Ok(())
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Body {
    detach: bool,
    tty: bool,
}
