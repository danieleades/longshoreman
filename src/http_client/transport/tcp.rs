use super::Transport;
use hyper::{client::HttpConnector, Body, Client};

#[derive(Debug)]
pub struct Tcp {
    host: String,
    client: Client<HttpConnector, Body>,
}

impl Tcp {
    pub fn new(host: impl Into<String>) -> Self {
        let client = Client::new();
        let host = host.into();
        Self { host, client }
    }
}

impl Transport for Tcp {
    fn uri(&self, endpoint: &str) -> String {
        format!("{}{}", self.host, endpoint)
    }
    fn send_request(&self, req: hyper::Request<hyper::Body>) -> hyper::client::ResponseFuture {
        self.client.request(req)
    }
}
