//! Endpoints and objects for juggling Docker images

use crate::{http_client::HttpClient, Result};
use std::sync::Arc;

pub mod inspect;
#[doc(inline)]
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

    pub async fn inspect<'a>(&'a self, name: &'a str) -> Result<inspect::Response> {
        Inspect::new(&self.http_client, name).send().await
    }
}
