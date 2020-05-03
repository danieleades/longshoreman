use crate::{http_client::HttpClient, Result};

/// A request to create a new docker container
#[derive(Debug)]
pub struct Create<'a> {
    http_client: &'a HttpClient,
}

impl<'a> Create<'a> {
    pub(crate) fn new(http_client: &'a HttpClient) -> Self {
        Self { http_client }
    }
}
