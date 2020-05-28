use crate::{http_client::HttpClient, Result};
use futures_util::stream::StreamExt;
use serde::Serialize;
use std::{fmt, pin::Pin};
use tokio::io::{AsyncWrite, AsyncWriteExt};
/// A request to get a tarball containing an image
///
/// # Example
///
/// ```
/// use longshoreman::{Docker, Result};
/// use tokio::fs::File;
/// use tempfile::NamedTempFile;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let id = "alpine:latest";
///     let tmp = NamedTempFile::new()?;
///     let save_to = File::from_std(tmp.reopen()?);
///
///     Docker::new()
///         .images()
///         .get(id, save_to)
///         .send()
///         .await?;
///
///     println!("Exported image size on disk: {}", tmp.into_file().metadata()?.len());
///
///     Ok(())
/// }
/// ```
pub struct Get<'a> {
    http_client: &'a HttpClient,
    image: &'a str,
    write_to: Pin<Box<dyn AsyncWrite + 'a>>,
}
impl fmt::Debug for Get<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Get")
            .field("http_client", &self.http_client)
            .field("image", &self.image)
            .finish()
    }
}

impl<'a> Get<'a> {
    pub(crate) fn new(
        http_client: &'a HttpClient,
        image: &'a str,
        write_to: impl AsyncWrite + 'a,
    ) -> Self {
        let write_to = Box::pin(write_to);
        Self {
            http_client,
            image,
            write_to,
        }
    }

    /// Consume the request and get the image
    pub async fn send(mut self) -> Result<()> {
        let endpoint = "/images/get";
        let mut request = Box::pin(
            self.http_client
                .get(endpoint)
                .query(Query {
                    // TODO: It's possible to get multiple images by giving
                    // a sequence like
                    // GET /v1.24/images/get?names=myname%2Fmyapp%3Alatest&names=busybox
                    // but: serde-encoded doesn't support serialization of Vec<T>
                    // https://github.com/nox/serde_urlencoded/issues/6
                    // Ignore this feature or implement it ourselves?
                    names: self.image,
                })
                .into_stream(),
        );

        while let Some(bytes) = request.next().await {
            let bytes = bytes?;
            self.write_to.write_all(&bytes).await?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Serialize)]
struct Query<'a> {
    names: &'a str,
}
