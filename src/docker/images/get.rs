use crate::{http_client::HttpClient, Result};
use futures_util::stream::TryStreamExt;
use serde::Serialize;
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio_util::compat::FuturesAsyncReadCompatExt;
/// TODO docs
#[derive(Debug)]
pub struct Get<'a> {
    http_client: &'a HttpClient,
    images: &'a [&'a str],
}

impl<'a> Get<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, images: &'a [&'a str]) -> Self {
        Self {
            http_client,
            images,
        }
    }

    /// Consume the request and get the image
    pub fn send(self) -> Result<Box<dyn AsyncRead + 'a + Unpin>> {
        let endpoint = "/images/get";
        let byte_stream = Box::pin(
            self.http_client
                .get(endpoint)
                .query_string(Query::new(self.images).to_query()?)
                .into_stream()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
        )
        .into_async_read()
        .compat();

        Ok(Box::new(byte_stream))
    }

    /// TODO docs
    pub async fn to_writer(self, mut writer: impl AsyncWrite + 'a + Unpin) -> Result<()> {
        let mut stream = self.send()?;
        io::copy(&mut stream, &mut writer).await?;
        Ok(())
    }
}

#[derive(Debug)]
struct Query<'a> {
    images_to_export: &'a [&'a str],
}
impl<'a> Query<'a> {
    fn new(images_to_export: &'a [&'a str]) -> Self {
        assert!(!images_to_export.is_empty());
        Self { images_to_export }
    }
    fn to_query(&self) -> Result<String> {
        let mut output = vec![];
        for i in self.images_to_export {
            let encoded = serde_urlencoded::to_string(InternalQuery::new(i))?;
            output.push(encoded);
        }
        Ok(output.join("&"))
    }
}

#[derive(Serialize)]
struct InternalQuery<'a> {
    names: &'a str,
}
impl<'a> InternalQuery<'a> {
    fn new(names: &'a str) -> Self {
        Self { names }
    }
}
