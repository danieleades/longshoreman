use crate::{http_client::HttpClient, volumes::Volume, Result};
use serde::Serialize;
use std::collections::HashMap;

/// A request to create a new docker container
///
/// # Example
///
/// ```no_run
/// use longshoreman::{Docker, Result};
///
/// #[tokio::test]
/// async fn test() -> Result<()> {
///     let volume = Docker::new()
///         .volumes()
///         .create()
///         .send()
///         .await?;
///
///     println!("{:#?}", volume);
///
///     Ok(())
/// }
/// ```
///
/// # Example
///
/// ```no_run
/// use longshoreman::{Docker, Result};
///
/// #[tokio::test]
/// async fn test() -> Result<()> {
///
///     let volume = Docker::new()
///         .volumes()
///         .create()
///         .name("my-volume")
///         .driver("local")
///         .label("key", "value")
///         .send()
///         .await?;
///
///     println!("{:#?}", volume);
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Create<'a> {
    http_client: &'a HttpClient,
    body: Body<'a>,
}

impl<'a> Create<'a> {
    pub(crate) fn new(http_client: &'a HttpClient) -> Self {
        let body = Body::default();
        Self { http_client, body }
    }

    /// Set the name of the volume
    ///
    /// If not specified, a name will be generated. Allowed name must match
    /// `/?[a-zA-Z0-9][a-zA-Z0-9_.-]+`
    #[must_use]
    pub fn name(mut self, name: &'a str) -> Self {
        self.body.name = Some(name);
        self
    }

    /// Set the driver for the volume
    ///
    /// The default is "local".
    #[must_use]
    pub fn driver(mut self, driver: &'a str) -> Self {
        self.body.driver = Some(driver);
        self
    }

    /// Driver-specific key-value pairs
    #[must_use]
    pub fn driver_opt(mut self, key: &'a str, value: &'a str) -> Self {
        self.body.driver_opts.insert(key, value);
        self
    }

    /// User-defined metadata
    #[must_use]
    pub fn label(mut self, key: &'a str, value: &'a str) -> Self {
        self.body.labels.insert(key, value);
        self
    }

    /// Consume the request builder and return a [`Volume`]
    pub async fn send(self) -> Result<Volume> {
        self.http_client
            .post("/volumes/create")
            .json_body(self.body)
            .into_json()
            .await
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Body<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver: Option<&'a str>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    driver_opts: HashMap<&'a str, &'a str>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    labels: HashMap<&'a str, &'a str>,
}
