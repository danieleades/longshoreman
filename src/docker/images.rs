use crate::http_client::HttpClient;
use futures_util::io::AsyncRead;
use std::sync::Arc;

mod load;
use load::Load;

pub struct Images {
    http_client: Arc<HttpClient>,
}

impl Images {
    pub(crate) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    pub fn load<'a>(&'a self, tar_archive: impl AsyncRead + 'a) -> Load<'a> {
        Load::new(&self.http_client, tar_archive)
    }
}
