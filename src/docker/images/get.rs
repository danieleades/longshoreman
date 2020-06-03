use crate::{http_client::HttpClient, Result};
use futures_util::stream::TryStreamExt;
use serde::Serialize;
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio_util::compat::FuturesAsyncReadCompatExt;
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
#[derive(Debug)]
pub struct Get<'a> {
    http_client: &'a HttpClient,
    images: &'a Vec<&'a str>,
}

impl<'a> Get<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, images: &'a Vec<&'a str>) -> Self {
        Self {
            http_client,
            images,
        }
    }

    /// Consume the request and get the image
    pub fn send(self) -> Box<dyn AsyncRead + 'a + Unpin> {
        let endpoint = "/images/get";
        let byte_stream = Box::pin(
            self.http_client
                .get(endpoint)
                .query(Query {
                    // TODO: It's possible to get multiple images by giving
                    // a sequence like
                    // GET /v1.24/images/get?names=myname%2Fmyapp%3Alatest&names=busybox
                    // but: serde-encoded doesn't support serialization of Vec<T>
                    // https://github.com/nox/serde_urlencoded/issues/6
                    // Ignore this feature or implement it ourselves?
                    // FIXME
                    names: self.images[0],
                })
                .into_stream()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
        )
        .into_async_read()
        .compat();

        Box::new(byte_stream)
    }

    /// TODO docs
    pub async fn to_writer(self, mut writer: impl AsyncWrite + 'a + Unpin) -> Result<()> {
        let mut stream = self.send();
        io::copy(&mut stream, &mut writer).await?;
        Ok(())
    }
}

#[derive(Debug, Default, Serialize)]
struct Query<'a> {
    names: &'a str,
}
