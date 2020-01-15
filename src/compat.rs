use pin_project::pin_project;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

#[pin_project]
pub struct Compat<S> {
    #[pin]
    reader_writer: S,
}

impl<S> Compat<S> {
    pub fn new(reader_writer: S) -> Self {
        Self { reader_writer }
    }
}

impl<S> futures_util::io::AsyncRead for Compat<S>
where
    S: tokio::io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.project().reader_writer.poll_read(cx, buf)
    }
}

impl<S> futures_util::io::AsyncWrite for Compat<S>
where
    S: tokio::io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.project().reader_writer.poll_write(cx, buf)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().reader_writer.poll_flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().reader_writer.poll_shutdown(cx)
    }
}

impl<S> tokio::io::AsyncRead for Compat<S>
where
    S: futures_util::io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.project().reader_writer.poll_read(cx, buf)
    }
}

impl<S> tokio::io::AsyncWrite for Compat<S>
where
    S: futures_util::io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.project().reader_writer.poll_write(cx, buf)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().reader_writer.poll_flush(cx)
    }
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().reader_writer.poll_close(cx)
    }
}
