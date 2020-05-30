//! Endpoints and objects for juggling Docker volumes

use crate::http_client::HttpClient;
use std::sync::Arc;

mod create;
pub use create::Create;

mod start;
pub use start::Start;

/// A client to the 'exec' subset of Docker API endpoints
#[derive(Debug)]
pub struct Exec {
    http_client: Arc<HttpClient>,
}

impl Exec {
    pub(crate) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    /// Create a new Exec instance
    pub fn create<'a>(&'a self, id: &'a str, command: Vec<&'a str>) -> Create<'a> {
        Create::new(&self.http_client, id, command)
    }

    /// Start an existing Exec instance
    pub fn start<'a>(&'a self, id: &'a str) -> Start<'a> {
        Start::new(&self.http_client, id)
    }
}
