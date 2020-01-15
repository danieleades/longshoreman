//! Endpoints and objects for juggling Docker images

use crate::http_client::HttpClient;
use futures_util::io::AsyncRead;
use std::sync::Arc;

mod load;
use load::Load;

/// A client to the 'images' subset of Docker API endpoints
#[derive(Debug)]
pub struct Images {
    http_client: Arc<HttpClient>,
}

impl Images {
    pub(crate) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    /// Load an image or a set of images from a tar archive.
    ///
    /// The archive may be compressed.
    ///
    /// # Example
    /// ```no_run
    /// use longshoreman::Docker;
    /// use async_std::File;
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// let images = Docker::new().images();
    ///
    /// let archive = File::open("path/to/archive.tar.gz").await?;
    ///
    /// let mut images_stream = images_client.load(archive).with_progress();
    ///
    /// while let Some(response) = images_stream.next().await {
    ///     println!("{:?}", response?)
    /// }
    pub fn load<'a>(&'a self, tar_archive: impl AsyncRead + 'a) -> Load<'a> {
        Load::new(&self.http_client, tar_archive)
    }
}
