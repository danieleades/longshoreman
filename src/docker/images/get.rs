use crate::{http_client::HttpClient, Result};
use futures_util::stream::{Stream, TryStreamExt};
use hyper::body::Bytes;
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
    pub fn send(self) -> Result<impl Stream<Item = Result<Bytes>> + 'a> {
        let endpoint = "/images/get";
        let query = Query::new(self.images);
        let byte_stream = self
            .http_client
            .get(endpoint)
            .query_string(query.to_query())
            .into_stream();

        Ok(byte_stream)
    }

    /// Convenience method to write the image directly into an `impl AsyncWrite`
    pub async fn write(self, mut writer: impl AsyncWrite + 'a + Unpin) -> Result<()> {
        let stream = self.send()?;
        let mut reader: Box<dyn AsyncRead + Unpin + 'a> = Box::new(
            Box::pin(stream.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
                .into_async_read()
                .compat(),
        );
        io::copy(&mut reader, &mut writer).await?;
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
    fn to_query(&self) -> String {
        let mut output = vec![];
        for i in self.images_to_export {
            let encoded = serde_urlencoded::to_string(InternalQuery::new(i)).unwrap();
            output.push(encoded);
        }
        output.join("&")
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
