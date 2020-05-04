use crate::{http_client::HttpClient, Result};
use futures_util::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use tokio::stream::Stream;

/// A request to pull an image
///
/// # Examples
///
/// ## Simple
///
/// ```no_run
/// use longshoreman::{Docker, Result};
/// use tokio::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let images = Docker::new().images();
///
///     // Pull an image and wait until the operation is complete
///     images.pull("ubuntu").tag("latest").send().await
/// }
/// ```
///
/// ## Further Options
///
/// ```no_run
/// use longshoreman::{Docker, Result};
/// use tokio::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let images = Docker::new().images();
///
///     // Pull an image and stream the progress
///     let mut stream = Box::pin(
///         images.pull("ubuntu").tag("latest").stream()
///     );
///     while let Some(status_message) = stream.next().await {
///         println!("{:#?}", status_message?);
///     }
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Pull<'a> {
    http_client: &'a HttpClient,
    query: Query<'a>,
}

impl<'a> Pull<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, name: &'a str) -> Self {
        let query = Query::new(name);
        Self { http_client, query }
    }

    /// Choose the tag of the image to pull. If unset, *all* tags will be
    /// pulled.
    #[must_use]
    pub fn tag(mut self, tag: &'a str) -> Self {
        self.query.tag = Some(tag);
        self
    }

    /// Consume the request and return a stream. The stream returns a sequence
    /// of progress messages.
    pub fn stream(self) -> impl Stream<Item = Result<StatusMessage>> + 'a {
        self.http_client
            .post("/images/create")
            .query(self.query)
            .into_stream_json()
    }

    /// Consume the request and return a future that resolves when the image
    /// pull is complete
    pub async fn send(self) -> Result<()> {
        let stream = self.stream();

        let result: Result<Vec<StatusMessage>> = stream.try_collect().await;

        result.map(|_| ())
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct Query<'a> {
    from_image: &'a str,
    tag: Option<&'a str>,
}

impl<'a> Query<'a> {
    fn new(name: &'a str) -> Self {
        let tag = None;
        Self {
            from_image: name,
            tag,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StatusMessage {
    status: String,
    #[serde(flatten)]
    progress: Option<Progress>,
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Progress {
    progress_detail: ProgressDetail,
    progress: String,
}

#[derive(Debug, Deserialize)]
struct ProgressDetail {
    current: u32,
    total: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod status_message {
        use super::*;
        mod deserialise {
            use super::*;
            #[test]
            fn pulling_from_repo() {
                let _: StatusMessage = serde_json::from_str(
                    r#"
                    {
                        "status": "Pulling from library/ubuntu",
                        "id": "10.04"
                    }
                    "#,
                )
                .unwrap();
            }

            #[test]
            fn warning() {
                let _: StatusMessage = serde_json::from_str(
                    r#"
                    {
                        "status": "Image docker.io/library/ubuntu:10.04 uses outdated schema1 manifest format. Please upgrade to a schema2 image for better future compatibility. More information at https://docs.docker.com/registry/spec/deprecated-schema-v1/",
                        "id": "10.04"
                    }
                    "#
                ).unwrap();
            }

            #[test]
            fn pulling_fs_layer() {
                let _: StatusMessage = serde_json::from_str(
                    r#"
                    {
                        "status": "Pulling fs layer",
                        "progressDetail": {},
                        "id": "a3ed95caeb02"
                    }
                    "#,
                )
                .unwrap();
            }

            #[test]
            fn dowloading() {
                let _: StatusMessage = serde_json::from_str(
                    r#"
                    {
                        "status": "Downloading",
                        "progressDetail": {
                            "current": 32,
                            "total": 32
                        },
                        "progress": "[==================================================>]      32B/32B",
                        "id": "a3ed95caeb02"
                    }
                    "#
                ).unwrap();
            }

            #[test]
            fn extracting() {
                let _: StatusMessage = serde_json::from_str(
                    r#"
                    {
                        "status": "Extracting",
                        "progressDetail": {
                            "current": 32,
                            "total": 32
                        },
                        "progress": "[==================================================>]      32B/32B",
                        "id": "a3ed95caeb02"
                    }
                    "#
                ).unwrap();
            }

            #[test]
            fn pulling_complete() {
                let _: StatusMessage = serde_json::from_str(
                    r#"
                    {
                        "status": "Pulling complete",
                        "progressDetail": {},
                        "id": "a3ed95caeb02"
                    }
                    "#,
                )
                .unwrap();
            }
        }
    }
}
