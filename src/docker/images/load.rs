use crate::{http_client::HttpClient, Result};
use futures_util::{
    future::TryFutureExt,
    io::{AsyncRead, AsyncReadExt},
    stream::{Stream, TryStreamExt},
};
use serde::Deserialize;
use std::pin::Pin;

pub struct Load<'a> {
    http_client: &'a HttpClient,
    tar_archive: Pin<Box<dyn AsyncRead + 'a>>,
}

impl<'a> Load<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, tar_archive: impl AsyncRead + 'a) -> Self {
        let tar_archive = Box::pin(tar_archive);

        Self {
            http_client,
            tar_archive,
        }
    }

    async fn read_archive(&mut self) -> Result<Vec<u8>> {
        let mut bytes = Vec::default();
        self.tar_archive.read_to_end(&mut bytes).await?;
        Ok(bytes)
    }

    /// Return a representation of the raw stream returned from the docker API.
    ///
    /// this can be used for returning progress updates on the import process.
    pub fn with_progress(mut self) -> impl Stream<Item = Result<Status>> + 'a {
        async move {
            let bytes = self.read_archive().await?;

            Ok(self
                .http_client
                .post("/images/load")
                .tar_body(bytes)
                .query(&[("quiet", false)])
                .into_stream_json())
        }
        .try_flatten_stream()
    }

    /// Return a stream of tuples of imported images (`([image], [tag])`)
    pub fn send(self) -> impl Stream<Item = Result<(String, String)>> + 'a {
        self.with_progress().try_filter_map(|status| async move {
            let ret = if let Status::Complete(complete) = status {
                let (image, tag) = complete.image();
                Some((image.to_string(), tag.to_string()))
            } else {
                None
            };
            Ok(ret)
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Status {
    Progress(Progress),
    Complete(Complete),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub id: String,
    pub progress_detail: ProgressDetail,
    #[serde(rename = "progress")]
    pub progress_string: String,
}

#[derive(Debug, Deserialize)]
pub struct ProgressDetail {
    pub current: i64,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct Complete {
    #[serde(rename = "stream")]
    status: String,
}

impl Complete {
    pub fn image(&self) -> (&str, &str) {
        let mut components = self.status.split(':').skip(1);

        let image = components.next().unwrap().trim();
        let tag = components.next().unwrap().trim();

        (image, tag)
    }
}

#[cfg(test)]
mod tests {

    use super::{Complete, Progress, Status};

    #[test]
    fn progress_deserialisation() {
        let string = r#"{"status":"Loading layer","progressDetail":{"current":32768,"total":1292800},"progress":"[=                                                 ] 32.77 kB/1.293 MB","id":"8ac8bfaff55a"}"#;

        let _: Progress = serde_json::from_str(string).unwrap();
    }

    #[test]
    fn complete_deserialisation() {
        let string = r#"{"stream":"Loaded image: busybox:latest\n"}"#;

        let _: Complete = serde_json::from_str(string).unwrap();
    }

    #[test]
    fn status_deserialisation() {
        let string = r#"{"status":"Loading layer","progressDetail":{"current":32768,"total":1292800},"progress":"[=                                                 ] 32.77 kB/1.293 MB","id":"8ac8bfaff55a"}"#;

        let _: Status = serde_json::from_str(string).unwrap();

        let string = r#"{"stream":"Loaded image: busybox:latest\n"}"#;

        let _: Status = serde_json::from_str(string).unwrap();
    }

    #[test]
    fn complete_image() {
        let string = r#"{"stream":"Loaded image: busybox:latest\n"}"#;

        let complete: Complete = serde_json::from_str(string).unwrap();

        assert_eq!(complete.image(), ("busybox", "latest"))
    }
}
