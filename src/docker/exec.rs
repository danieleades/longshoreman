//! Endpoints and objects for juggling Docker volumes

use crate::http_client::HttpClient;
use std::sync::Arc;

mod create;
pub use create::Create;

/// A client to the 'images' subset of Docker API endpoints
#[derive(Debug)]
pub struct Exec {
    http_client: Arc<HttpClient>,
}

impl Exec {
    pub(crate) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    /// Create a new Docker volume
    pub fn create<'a>(&'a self, id: &'a str, command: Vec<&'a str>) -> Create<'a> {
        Create::new(&self.http_client, id, command)
    }
}
