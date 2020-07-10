use super::{BodyType, HttpClient};
use crate::{Error, Result};
use futures_util::{
    future::TryFutureExt,
    stream::{Stream, StreamExt, TryStreamExt},
};
use hyper::{
    body::{to_bytes, Bytes},
    Body, Method, Request, StatusCode,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// A builder API for constructing and sending an HTTP request to the Docker
/// host.
///
/// ## Builder Methods
/// This object has a number of 'builder' methods for building the request.
/// These are modelled after `Reqwest`'s API.
///
/// ## Finalisers
/// A number of 'finaliser' methods are provided for turning the received
/// responses into different response types, depending on needs of the endpoint.
/// These include unary responses, streams, and decoding into JSON objects.
///
/// Implementations of specific endpoints may make use of these finalisers, and
/// need not reimplement this logic.
pub(crate) struct RequestBuilder<'a> {
    http_client: &'a HttpClient,
    uri_base: String,
    query: Option<String>,
    body: Option<BodyType>,
    builder: http::request::Builder,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(http_client: &'a HttpClient, endpoint: impl AsRef<str>) -> Self {
        let uri_base = http_client.uri(endpoint.as_ref());
        let query = None;
        let body = None;
        let mut builder = hyper::Request::builder();
        builder = builder.method(Method::GET);

        debug_assert!(builder.method_ref().is_some());

        Self {
            http_client,
            uri_base,
            query,
            body,
            builder,
        }
    }

    // Builder methods

    /// Set the HTTP method
    pub fn method(mut self, method: Method) -> Self {
        self.builder = self.builder.method(method);
        self
    }

    /// Add a custom header to the request
    /*     pub fn header(mut self, key: impl IntoHeaderName, value: &str) -> Self {
        let value = http::header::HeaderValue::from_str(value).unwrap();
        self.builder.headers_mut().unwrap().append(key, value);
        self
    } */

    /// Add a form-encoded query to the request.
    ///
    /// Any type that implements [`serde::Deserialize`] can be used
    pub fn query<T: Serialize>(mut self, query: T) -> Self {
        let query_string = serde_urlencoded::ser::to_string(query).unwrap();

        self.query = Some(query_string);
        self
    }

    /// Add a form-encoded query to the request.
    ///
    /// The provided string will be appended to the request, delimited by '?'.
    /// For convenience, you probably want to use the provided [`query`] method.
    pub fn query_string(mut self, query_string: String) -> Self {
        self.query = Some(query_string);
        self
    }

    /// Add a JSON body to the request.
    ///
    /// Any type that implements [`serde::Deserialize`] can be used. This method
    /// will automatically set the content type header
    pub fn json_body<T: Serialize>(mut self, body: T) -> Self {
        let data = serde_json::to_vec(&body).unwrap();
        self.body = Some(BodyType::json(data));
        self
    }

    /// Add a tar archive as the body of the request
    ///
    /// This method will automatically set the content type header
    pub fn tar_body(mut self, data: Vec<u8>) -> Self {
        self.body = Some(BodyType::tar(data));
        self
    }

    // Finalisers

    /// Build the request
    fn into_request(self) -> Result<(Request<Body>, &'a HttpClient)> {
        let uri = if let Some(query_string) = &self.query {
            format!("{}?{}", self.uri_base, query_string)
        } else {
            self.uri_base
        };

        let mut builder = self.builder;
        builder = builder.uri(uri);

        let request = match self.body {
            Some(body_type) => {
                let mime = http::HeaderValue::try_from(body_type.mime())?;
                let data = body_type.into_data();
                let body = Body::from(data);

                builder.headers_mut().unwrap().append("content-type", mime);
                builder.body(body)?
            }
            None => builder.body(Body::empty())?,
        };

        Ok((request, self.http_client))
    }

    /// Send the request and return a [`hyper::Result`]
    pub async fn into_response(self) -> Result<hyper::Response<Body>> {
        let (request, client) = self.into_request()?;
        Ok(client.send_request(request).await?)
    }

    /// Send the request and return the body of the response.
    ///
    /// This method will check the status code of the response convert it into
    /// an error, as required.
    async fn into_body(self) -> Result<Body> {
        let response = self.into_response().await?;
        let status = response.status();

        match status {
            // Success case: pass on the response
            StatusCode::OK
            | StatusCode::CREATED
            | StatusCode::SWITCHING_PROTOCOLS
            | StatusCode::NO_CONTENT => Ok(response.into_body()),
            // Error case: parse the text
            _ => {
                let bytes = to_bytes(response.into_body()).await?;

                Err(Error::Fault {
                    code: status,
                    message: get_error_message(&bytes).unwrap_or_else(|_| {
                        status
                            .canonical_reason()
                            .unwrap_or_else(|| "unknown error code")
                            .to_owned()
                    }),
                })
            }
        }
    }

    /// Upgrade the HTTP connection into a duplex stream
    /*     pub async fn upgrade(mut self) -> Result<impl AsyncRead + AsyncWrite> {
        self = self.header(hyper::header::CONNECTION, "Upgrade");
        self = self.header(hyper::header::UPGRADE, "tcp");

        let hyper_response = self.into_response().await?;

        match hyper_response.status() {
            StatusCode::SWITCHING_PROTOCOLS => Ok(hyper_response.into_body().on_upgrade().await?),
            _ => Err(Error::ConnectionNotUpgraded),
        }
    } */

    /// Send the request and concatenate the response into a Bytes object
    async fn into_bytes(self) -> Result<Bytes> {
        let body = self.into_body().await?;
        Ok(to_bytes(body).await?)
    }

    /*     /// Send the request, and concatenate the response into a string
    pub async fn into_string(self) -> Result<String> {
        let bytes = self.into_bytes().await?;
        Ok(String::from_utf8(bytes.to_vec())?)
    } */

    /// Send the request and deserialize the JSON body response into an object
    pub async fn into_json<T>(self) -> Result<T>
    where
        for<'de> T: Deserialize<'de>,
    {
        let bytes = self.into_bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Send the request, and return the response body as a stream of bytes
    pub fn into_stream(self) -> impl Stream<Item = Result<Bytes>> + 'a {
        async fn unfold(mut body: Body) -> Option<(Result<Bytes>, Body)> {
            let bytes_result = body.next().await?.map_err(Error::from);

            Some((bytes_result, body))
        }

        async move {
            let body = self.into_body().await?;
            Ok(futures_util::stream::unfold(body, unfold))
        }
        .try_flatten_stream()
    }

    /// Send the request, and deserialize the returned stream of JSON into a
    /// stream of objects.
    pub fn into_stream_json<T>(self) -> impl Stream<Item = Result<T>> + 'a
    where
        for<'de> T: Deserialize<'de>,
    {
        let byte_stream = self.into_stream();

        byte_stream.and_then(|bytes| async move { Ok(serde_json::from_slice(&bytes)?) })
    }

    /*     /// Send the request, and deserialize the returned stream using the given
    /// codec
    pub fn decode<T, C, I, E>(self, codec: C) -> impl Stream<Item = Result<T>> + 'a
    where
        for<'de> T: Deserialize<'de>,
        C: Decoder<Item = I, Error = E> + 'a,
        I: AsRef<[u8]> + 'a,
        Error: From<E>,
        E: From<std::io::Error> + 'a,
    {
        let stream = self
            .into_stream()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e));

        let reader = stream_reader(stream);
        FramedRead::new(reader, codec)
            .map_err(Error::from)
            .and_then(|bytes| async move { Ok(serde_json::from_slice(bytes.as_ref())?) })
    } */
}

fn get_error_message(bytes: impl AsRef<[u8]>) -> Result<String> {
    #[derive(Serialize, Deserialize)]
    struct ErrorResponse {
        message: String,
    }

    let error_response: ErrorResponse = serde_json::from_slice(bytes.as_ref())?;
    Ok(error_response.message)
}
