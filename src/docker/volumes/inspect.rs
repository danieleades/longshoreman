use crate::{http_client::HttpClient, volumes::Volume, Result};

/// A request to remove an existing docker container
///
/// # Example
///
/// ```no_run
/// use longshoreman::{Docker, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let id = "VOLUME";
///
///     let response = Docker::new()
///         .volumes()
///         .inspect(id)
///         .await?;
///
///     println!("{:#?}", response);
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Inspect<'a> {
    http_client: &'a HttpClient,
    name: &'a str,
}

impl<'a> Inspect<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, name: &'a str) -> Self {
        Self { http_client, name }
    }

    /// Consume the request and return details about the container
    pub async fn send(self) -> Result<Volume> {
        let endpoint = format!("/volumes/{}", self.name);
        self.http_client.get(endpoint).into_json().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialise_response() {
        let _: Volume = serde_json::from_str(
            r#"{
                "Name": "tardis",
                "Driver": "custom",
                "Mountpoint": "/var/lib/docker/volumes/tardis",
                "Status": {
                    "hello": "world"
                },
                "Labels": {
                    "com.example.some-label": "some-value",
                    "com.example.some-other-label": "some-other-value"
                },
                "Scope": "local",
                "CreatedAt": "2016-06-07T20:31:11.853781916Z"
            }"#,
        )
        .unwrap();
    }
}
