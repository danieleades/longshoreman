//! Endpoints and objects for juggling Docker containers

use crate::http_client::HttpClient;
use std::sync::Arc;

mod create;
pub use create::Create;

mod remove;
pub use remove::Remove;

/// A client to the 'containers' subset of Docker API endpoints
#[derive(Debug)]
pub struct Containers {
    http_client: Arc<HttpClient>,
}

impl Containers {
    pub(crate) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    /// Create a new Docker container
    #[must_use]
    pub fn create<'a>(&'a self, image: &'a str) -> Create<'a> {
        Create::new(&self.http_client, image)
    }

    /// Remove an existing Docker container
    ///
    /// The 'container' parameter may be a name or an id.
    pub fn remove<'a>(&'a self, container: &'a str) -> Remove<'a> {
        Remove::new(&self.http_client, container)
    }
}
