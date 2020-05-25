use crate::{http_client::HttpClient, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

/// A request to remove an existing docker container
///
/// # Example
///
/// ```no_run
/// use longshoreman::{Docker, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let id = "VOLUME";
///
///     let response = Docker::new()
///         .volumes()
///         .inspect(id)
///         .await?;
///
///     println!("{:#?}", response);
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Inspect<'a> {
    http_client: &'a HttpClient,
    name: &'a str,
}

impl<'a> Inspect<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, name: &'a str) -> Self {
        Self { http_client, name }
    }

    /// Consume the request and return details about the container
    pub async fn send(self) -> Result<Response> {
        let endpoint = format!("/volumes/{}", self.name);
        self.http_client.get(endpoint).into_json().await
    }
}

/// A struct representation the information returned by a 'container inspect'
/// command
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    /// The name of the Docker volume
    pub name: String,

    /// The volume driver
    pub driver: String,

    /// The location on the host filesystem where the volume is mounted
    pub mountpoint: PathBuf,

    /// TODO
    pub status: Status,

    /// docker labels on this volume
    pub labels: HashMap<String, String>,

    /// The scope of the volume
    pub scope: Scope,

    /// The datetime that the container was created
    pub created_at: DateTime<Utc>,
}

/// The state of a docker container
#[derive(Debug, Deserialize)]
pub struct Status {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Local,
    Global,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialise_response() {
        let _: Response = serde_json::from_str(
            r#"{
                "Name": "tardis",
                "Driver": "custom",
                "Mountpoint": "/var/lib/docker/volumes/tardis",
                "Status": {
                    "hello": "world"
                },
                "Labels": {
                    "com.example.some-label": "some-value",
                    "com.example.some-other-label": "some-other-value"
                },
                "Scope": "local",
                "CreatedAt": "2016-06-07T20:31:11.853781916Z"
            }"#,
        )
        .unwrap();
    }
}
