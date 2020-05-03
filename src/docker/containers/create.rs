use crate::{http_client::HttpClient, Result};
use serde::{Deserialize, Serialize};

/// A request to create a new docker container
#[derive(Debug)]
pub struct Create<'a> {
    http_client: &'a HttpClient,
    query: Query,
    body: Body,
}

impl<'a> Create<'a> {
    pub(crate) fn new(http_client: &'a HttpClient) -> Self {
        let query = Query {};
        let body = Body {};
        Self {
            http_client,
            query,
            body,
        }
    }

    /// Consume the request builder and return a [`Response`]
    pub async fn send(self) -> Result<Response> {
        self.http_client
            .post("containers/create")
            .query(self.query)
            .json_body(self.body)
            .into_json()
            .await
    }
}

#[derive(Debug, Serialize)]
struct Query {}

#[derive(Debug, Serialize)]
struct Body {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    id: String,
    warnings: Vec<String>,
}