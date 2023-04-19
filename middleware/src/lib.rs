use bindings::{
    downstream,
    inbound_http::{InboundHttp, Request, Response},
};
use flate2::{write::GzEncoder, Compression};
use std::io::Write;

struct Component;

impl InboundHttp for Component {
    fn handle_request(req: Request) -> Response {
        // Send the request to the downstream service
        let mut response = downstream::handle_request(&downstream::Request {
            headers: req.headers,
            params: req.params,
            method: downstream::Method::Get,
            uri: req.uri,
            body: req.body,
        });

        // If the response is already encoded, leave it alone
        let is_encoded = response
            .headers
            .as_ref()
            .map(|headers| headers
                .iter()
                .any(|(k, _)| k == "content-encoding"))
                .unwrap_or(false);

        if is_encoded {
            return Response {
                status: response.status,
                body: response.body,
                headers: response.headers,
            };
        }

        match response.body {
            Some(body) => {
                // Set the `content-encoding` header to `gzip`
                response
                    .headers
                    .get_or_insert_with(|| Default::default())
                    .push(("content-encoding".to_string(), "gzip".to_string()));

                // Compress the response body
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                match encoder.write_all(&body) {
                    Err(error) => {
                        eprintln!("compression error: {error}");
                        Response {
                            status: 500,
                            body: None,
                            headers: None,
                        }
                    }
                    Ok(()) => {
                        match encoder.finish() {
                            Err(error) => {
                                eprintln!("compression error: {error}");
                                Response {
                                    status: 500,
                                    body: None,
                                    headers: None,
                                }
                            }
                            Ok(body) => {
                                Response {
                                    status: response.status,
                                    body: Some(body),
                                    headers: response.headers,
                                }   
                            }
                        }
                    }
                }
            },
            None => {
                Response {
                    status: response.status,
                    body: None,
                    headers: response.headers,
                }
            },
        }
    }
}

bindings::export!(Component);