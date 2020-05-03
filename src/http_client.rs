//! Transports for communicating with the docker daemon

use hyper::Method;
use std::path::PathBuf;

mod transport;
use transport::Transport;

mod request;
use request::RequestBuilder;

#[derive(Debug)]
pub(crate) enum HttpClient {
    Tcp(transport::Tcp),

    #[cfg(target_os = "linux")]
    Uds(transport::Uds),
}

impl HttpClient {
    pub fn tcp(host: impl Into<String>) -> Self {
        let transport = transport::Tcp::new(host);
        Self::Tcp(transport)
    }

    #[cfg(target_os = "linux")]
    pub fn unix(path: impl Into<PathBuf>) -> Self {
        let transport = transport::Uds::new(path);
        Self::Uds(transport)
    }

    fn request(&self, endpoint: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder::new(self, endpoint)
    }

    /*     pub fn get(&self, endpoint: impl AsRef<str>) -> RequestBuilder {
        self.request(endpoint).method(Method::GET)
    } */

    pub fn post(&self, endpoint: impl AsRef<str>) -> RequestBuilder {
        self.request(endpoint).method(Method::POST)
    }

    /*     pub fn put(&self, endpoint: impl AsRef<str>) -> RequestBuilder {
        self.request(endpoint).method(Method::PUT)
    } */

    pub fn delete<'a>(&'a self, endpoint: &'a str) -> RequestBuilder<'a> {
        self.request(endpoint).method(Method::DELETE)
    }

    fn transport(&self) -> &dyn Transport {
        match self {
            Self::Tcp(transport) => transport,

            #[cfg(target_os = "linux")]
            Self::Uds(transport) => transport,
        }
    }

    fn uri(&self, endpoint: impl AsRef<str>) -> String {
        self.transport().uri(endpoint.as_ref())
    }

    fn send_request(&self, req: hyper::Request<hyper::Body>) -> hyper::client::ResponseFuture {
        self.transport().send_request(req)
    }
}

pub enum BodyType {
    Json(Vec<u8>),
    Tar(Vec<u8>),
}

impl BodyType {
    fn json(data: Vec<u8>) -> Self {
        Self::Json(data)
    }

    fn tar(data: Vec<u8>) -> Self {
        Self::Tar(data)
    }

    fn mime(&self) -> String {
        match self {
            Self::Json(_) => mime::APPLICATION_JSON.to_string(),
            Self::Tar(_) => "application/x-tar".to_string(),
        }
    }

    fn into_data(self) -> Vec<u8> {
        match self {
            Self::Json(data) | Self::Tar(data) => data,
        }
    }
}
