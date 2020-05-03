//! Endpoints and objects for juggling Docker containers

use crate::http_client::HttpClient;
use std::sync::Arc;

mod create;
pub use create::Create;

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
    pub fn create(&self) -> Create {
        Create::new(&self.http_client)
    }
}
