use super::Transport;
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Uds {
    path: PathBuf,
    client: hyper::Client<UnixConnector, hyper::Body>,
}

impl Uds {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let client = hyper::Client::unix();
        Self { path, client }
    }
}

impl Transport for Uds {
    fn uri(&self, endpoint: &str) -> String {
        let uri: hyper::Uri = Uri::new(&self.path, endpoint).into();
        uri.to_string()
    }
    fn send_request(&self, req: hyper::Request<hyper::Body>) -> hyper::client::ResponseFuture {
        self.client.request(req)
    }
}
