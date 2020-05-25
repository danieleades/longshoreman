//! Endpoints and objects for juggling Docker volumes

use crate::{http_client::HttpClient, Result};
use std::sync::Arc;

mod inspect;
pub use inspect::{Inspect, Response as InspectResponse};

/// A client to the 'images' subset of Docker API endpoints
#[derive(Debug)]
pub struct Volumes {
    http_client: Arc<HttpClient>,
}

impl Volumes {
    pub(crate) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    /// Inspect a Docker volume
    pub async fn inspect<'a>(&'a self, name: &'a str) -> Result<InspectResponse> {
        Inspect::new(&self.http_client, name).send().await
    }
}
