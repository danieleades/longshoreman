use crate::{http_client::HttpClient, Result};
use serde::{Deserialize, Serialize};

/// A request to create a new docker container
#[derive(Debug)]
pub struct Create<'a> {
    http_client: &'a HttpClient,
    query: Query<'a>,
    body: Body<'a>,
}

impl<'a> Create<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, image: &'a str) -> Self {
        let query = Query::default();
        let body = Body::new(image);
        Self {
            http_client,
            query,
            body,
        }
    }

    /// Set the name of the container
    ///
    /// Allowed name must match `/?[a-zA-Z0-9][a-zA-Z0-9_.-]+`
    pub fn name(mut self, name: &'a str) -> Self {
        self.query.name = Some(name);
        self
    }

    /// Consume the request builder and return a [`Response`]
    pub async fn send(self) -> Result<Response> {
        self.http_client
            .post("/containers/create")
            .query(self.query)
            .json_body(self.body)
            .into_json()
            .await
    }
}

#[derive(Debug, Default, Serialize)]
struct Query<'a> {
    name: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Body<'a> {
    image: &'a str,
}

impl<'a> Body<'a> {
    fn new(image: &'a str) -> Self {
        Self { image }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    id: String,
    warnings: Vec<String>,
}

impl Response {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn warnings(&self) -> &Vec<String> {
        &self.warnings
    }
}

#[cfg(test)]
mod tests {
    use super::Response;

    #[test]
    fn deserialize_response() {
        let response_string = r#"
        {
            "Id": "e90e34656806",
            "Warnings": []
          }
        "#;

        let _: Response = serde_json::from_str(response_string).unwrap();
    }
}
