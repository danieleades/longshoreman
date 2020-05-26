use crate::{http_client::HttpClient, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A request to create a new 'exec' instance
#[derive(Debug)]
pub struct Create<'a> {
    http_client: &'a HttpClient,
    id: &'a str,
    body: Body<'a>,
}

impl<'a> Create<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, id: &'a str, command: Vec<&'a str>) -> Self {
        let body = Body::new(command);
        Self {
            http_client,
            id,
            body,
        }
    }

    /// Attach stdout to the running command
    ///
    /// Default is true.
    #[must_use]
    pub fn attach_stdout(mut self, attach: bool) -> Self {
        self.body.attach_stdout = attach;
        self
    }

    /// Attach stderr to the running command
    ///
    /// Default is false.
    #[must_use]
    pub fn attach_stderr(mut self, attach: bool) -> Self {
        self.body.attach_stderr = attach;
        self
    }

    /// Attach stdin to the running command
    ///
    /// Default is false.
    #[must_use]
    pub fn attach_stdin(mut self, attach: bool) -> Self {
        self.body.attach_stdin = attach;
        self
    }

    /// Run the command with sudo privileges.
    ///
    /// Default is false.
    #[must_use]
    pub fn privileged(mut self, privileged: bool) -> Self {
        self.body.privileged = privileged;
        self
    }

    /// The directory in which to run the command.
    ///
    /// If unset, this command will be run in the working directory set by the
    /// image
    #[must_use]
    pub fn working_dir(mut self, path: &'a Path) -> Self {
        self.body.working_dir = Some(path);
        self
    }

    /// Consume the request builder and return the 'exec' instance id
    pub async fn send(self) -> Result<String> {
        let endpoint = format!("/containers/{}/exec", self.id);
        let response: Response = self
            .http_client
            .post(endpoint)
            .json_body(self.body)
            .into_json()
            .await?;

        Ok(response.id)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Body<'a> {
    attach_stdin: bool,
    attach_stdout: bool,
    attach_stderr: bool,
    cmd: Vec<&'a str>,
    privileged: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<&'a Path>,
}

impl<'a> Body<'a> {
    fn new(cmd: Vec<&'a str>) -> Self {
        Self {
            attach_stdin: false,
            attach_stdout: true,
            attach_stderr: false,
            cmd,
            privileged: false,
            working_dir: None,
        }
    }
}

#[derive(Deserialize)]
struct Response {
    id: String,
}
