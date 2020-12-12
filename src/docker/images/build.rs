use crate::{http_client::HttpClient, Result};
use futures_util::{
    future::TryFutureExt,
    stream::{Stream},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncReadExt};
/// A request to build an image from a dockerfile/folder
#[allow(missing_debug_implementations)]
pub struct Build<'a> {
    http_client: &'a HttpClient,
    query: Query<'a>,
    tar_archive: Pin<Box<dyn AsyncRead + 'a>>,
    labels: Map<String, Value>,
    buildargs: Map<String, Value>,
}

impl<'a> Build<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, tar_archive: impl AsyncRead + 'a) -> Self {
        let tar_archive = Box::pin(tar_archive);
        let query = Query::default();
        let labels = Map::new();
        let buildargs = Map::new();
        Self {
            http_client,
            tar_archive,
            query,
            labels,
            buildargs,
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
        Box::pin(
            async move {
                let bytes = self.read_archive().await?;
                println!("test this shit");
                let serialized_buildargs=serde_json::to_string(&self.buildargs).unwrap_or_default();
                print!("{:?}",serialized_buildargs);
                self.query.buildargs=Some(serialized_buildargs);
              
                let serialized_labels=serde_json::to_string(&self.labels).unwrap_or_default();
                print!("{:?}",serialized_labels);
                self.query.labels=Some(serialized_labels);
                Ok(self
                    .http_client
                    .post("/build")
                    .tar_body(bytes)
                    .query(self.query)
                    .into_stream_json())
            }
            .try_flatten_stream()
        )
    }
    /// Choose the tag to set for the build image.
    #[must_use]
    pub fn tag(mut self, tag: &'a str) -> Self {
        self.query.t = Some(tag);
        self
    }
    /// Specify Dockerfile to use from context if not default.
    #[must_use]
    pub fn dockerfile(mut self, dockerfile: &'a str) -> Self {
        self.query.dockerfile = Some(dockerfile);
        self
    }
    /// Specify if cache should be used.
    #[must_use]
    pub fn nocache(mut self, nocache: bool) -> Self {
        self.query.nocache = Some(nocache);
        self
    }
    /// Specify network to be attached while building.
    #[must_use]
    pub fn net(mut self, network: &'a str) -> Self {
        self.query.networkmode = Some(network);
        self
    }
    /// Specify target to run.
    #[must_use]
    pub fn target(mut self, target: &'a str) -> Self {
        self.query.target = Some(target);
        self
    }

    /// labelmap
    #[must_use]
    pub fn label(mut self, key: &'a str, value: &'a str) -> Self {
        self.labels.insert(key.into(), Value::String(value.into()));
        self
    }
    /// buildargs
    #[must_use]
    pub fn buildarg(mut self, key: &'a str, value: &'a str) -> Self {
        self.buildargs
            .insert(key.into(), Value::String(value.into()));
        self
    }
    /// buildargs
    #[must_use]
    pub fn remote(mut self, remote: &'a str) -> Self {
        self.query.remote=Some(remote);   
        self
    }
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Status {
    Progress(Progress),
    Other(Other),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Other {
    pub aux: Aux
}
#[derive(Debug, Deserialize)]
pub struct Aux {
    #[serde(rename = "ID")]
    id: String
}

#[derive(Debug, Deserialize)]
pub struct Progress {
    #[serde(rename = "stream")]
    status: String,
}

#[derive(Debug, Default, Serialize)]
struct Query<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    t: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extrahosts: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nocache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cachefrom: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forcerm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buildargs: Option<String>, //Serialized map
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<String>, //Serialized map
    #[serde(skip_serializing_if = "Option::is_none")]
    networkmode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outputs: Option<&'a str>,
}
#[cfg(test)]
mod tests {

    use super::{Other, Progress};


    #[test]
    fn complete_deserialisation() {
        let string = r#"{"stream":"Loaded image: busybox:latest\n"}"#;

        let _: Progress = serde_json::from_str(string).unwrap();
    }

    #[test]
    fn aux_deserialisation() {
        let string = r#"{"aux":{"ID":"sha256:393b8b26c9c7367490906dcf79234323f6b3c36477f6aa4b83660593b066a0fd"}}"#;
        let _: Other = serde_json::from_str(string).unwrap();
    }
}
