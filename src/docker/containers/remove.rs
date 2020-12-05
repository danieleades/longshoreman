use crate::{http_client::HttpClient, Result};
use serde::Serialize;

/// A request to remove an existing docker container
///
/// # Example
///
/// ```no_run
/// use longshoreman::{Docker, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let id = "CONTAINER_ID";
///
///     Docker::new()
///         .containers()
///         .remove(id)
///         .force(true)
///         .send()
///         .await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Remove<'a> {
    http_client: &'a HttpClient,
    container: &'a str,
    query: Query,
}

impl<'a> Remove<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, container: &'a str) -> Self {
        let query = Query::default();
        Self {
            http_client,
            container,
            query,
        }
    }

    /// Whether to also remove volumes associated with the container.
    ///
    /// default is 'false'.
    #[must_use]
    pub fn remove_volumes(mut self, remove_volumes: bool) -> Self {
        self.query.v = remove_volumes;
        self
    }

    /// Whether to force removal of a container, even if it is still running.
    ///
    /// default is 'false'.
    #[must_use]
    pub fn force(mut self, force: bool) -> Self {
        self.query.force = force;
        self
    }

    /// Consume the request builder and send the request to the Docker host
    pub async fn send(self) -> Result<()> {
        let endpoint = format!("/containers/{}", self.container);
        self.http_client
            .delete(&endpoint)
            .query(self.query)
            .into_response()
            .await?;

        Ok(())
    }
}

#[derive(Debug, Default, Serialize)]
struct Query {
    v: bool,
    force: bool,
    link: bool,
}
