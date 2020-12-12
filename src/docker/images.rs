//! Endpoints and objects for juggling Docker images

use crate::http_client::HttpClient;
use std::sync::Arc;
use tokio::io::AsyncRead;

mod build;
pub use build::Build;

mod load;
pub use load::Load;

mod pull;
pub use pull::Pull;

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
    /// use tokio::fs::File;
    /// # use futures_util::stream::StreamExt;
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// let images = Docker::new().images();
    ///
    /// let archive = File::open("path/to/archive.tar.gz").await?;
    ///
    /// let mut response_stream = images.load(archive).with_progress();
    ///
    /// while let Some(response) = response_stream.next().await {
    ///     println!("{:?}", response?)
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn load<'a>(&'a self, tar_archive: impl AsyncRead + 'a) -> Load<'a> {
        Load::new(&self.http_client, tar_archive)
    }

    /// Pull an image
    #[must_use]
    pub fn pull<'a>(&'a self, name: &'a str) -> Pull<'a> {
        Pull::new(&self.http_client, name)
    }

     /// Pull an image
     #[must_use]
     pub fn build<'a>(&'a self, tar_archive: impl AsyncRead + 'a) -> Build<'a> {
         Build::new(&self.http_client, tar_archive)
     }
}
