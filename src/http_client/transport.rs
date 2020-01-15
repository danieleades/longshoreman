mod tcp;
pub use tcp::Tcp;

#[cfg(target_os = "linux")]
mod uds;
#[cfg(target_os = "linux")]
pub use uds::Uds;

pub trait Transport {
    fn uri(&self, endpoint: &str) -> String;

    fn send_request(&self, req: hyper::Request<hyper::Body>) -> hyper::client::ResponseFuture;
}
