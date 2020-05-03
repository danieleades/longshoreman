use crate::http_client::HttpClient;
use hyper::Uri;
use std::{path::PathBuf, sync::Arc};

pub mod images;

use images::Images;

pub mod containers;

/// A Docker client.
///
/// The [`Docker`] client provides top-level API endpoints, and is used to
/// instantiate the other clients.
///
/// # Example
/// ```
/// use longshoreman::Docker;
///
/// let docker_client = Docker::new();
/// ```
#[derive(Debug)]
pub struct Docker {
    http_client: Arc<HttpClient>,
}

impl Default for Docker {
    fn default() -> Self {
        Self::new()
    }
}

impl Docker {
    /// constructs a new Docker instance for a docker host listening at a url
    /// specified by an env var `DOCKER_HOST`, falling back to
    /// `unix:///var/run/docker.sock`
    #[must_use]
    pub fn new() -> Docker {
        match std::env::var("DOCKER_HOST").ok() {
            Some(host) => {
                let host = host.parse().expect("invalid url");
                Self::host(&host)
            }
            #[cfg(target_os = "linux")]
            None => Self::unix(PathBuf::from("/var/run/docker.sock")),
            #[cfg(not(target_os = "linux"))]
            None => panic!("Unix socket support is disabled"),
        }
    }

    /// Creates a new docker instance for a docker host
    /// listening on a given Unix socket.
    #[cfg(target_os = "linux")]
    pub fn unix(socket_path: impl Into<PathBuf>) -> Docker {
        let http_client = Arc::new(HttpClient::unix(socket_path));
        Self { http_client }
    }

    /// constructs a new Docker instance for docker host listening at the given
    /// host url
    #[allow(clippy::single_match_else)]
    pub fn host(host: &Uri) -> Docker {
        match host.scheme_str() {
            #[cfg(target_os = "linux")]
            Some("unix") => Self::unix(host.path().to_owned()),

            #[cfg(not(target_os = "linux"))]
            Some("unix") => panic!("Unix socket support is disabled"),

            #[cfg(feature = "tls")]
            _ => {
                let tcp_host_str = format!(
                    "{}://{}:{}",
                    host.scheme_str().unwrap(),
                    host.host().unwrap().to_owned(),
                    host.port_u16().unwrap_or(80)
                );
                Self::tls(tcp_host_str)
            }

            #[cfg(not(feature = "tls"))]
            _ => {
                let tcp_host_str = format!(
                    "{}://{}:{}",
                    host.scheme_str().unwrap(),
                    host.host().unwrap().to_owned(),
                    host.port_u16().unwrap_or(80)
                );
                Self::tcp(tcp_host_str)
            }
        }
    }

    #[cfg(not(feature = "tls"))]
    fn tcp(host: String) -> Docker {
        let http_client = Arc::new(HttpClient::tcp(host));
        Self { http_client }
    }

    #[cfg(feature = "tls")]
    fn tls(host: String) -> Docker {
        let http_client = Arc::new(HttpClient::tls(host));
        Self { http_client }
    }

    /// Return an [`Images`] client.
    ///
    /// See the [`Images`] client docs for more details
    ///
    /// # Example
    /// ```
    /// use longshoreman::Docker;
    ///
    /// let images = Docker::new().images();
    /// ```
    #[must_use]
    pub fn images(&self) -> Images {
        Images::new(Arc::clone(&self.http_client))
    }
}
