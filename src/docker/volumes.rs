//! Endpoints and objects for juggling Docker volumes

use crate::{http_client::HttpClient, Result};
use std::sync::Arc;

mod create;
pub use create::Create;

mod types;
pub use types::Volume;

mod inspect;
pub use inspect::Inspect;

/// A client to the 'images' subset of Docker API endpoints
#[derive(Debug)]
pub struct Volumes {
    http_client: Arc<HttpClient>,
}

impl Volumes {
    pub(crate) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    /// Create a new Docker volume
    pub fn create(&self) -> Create {
        Create::new(&self.http_client)
    }

    /// Inspect a Docker volume
    pub async fn inspect<'a>(&'a self, name: &'a str) -> Result<Volume> {
        Inspect::new(&self.http_client, name).send().await
    }
}
