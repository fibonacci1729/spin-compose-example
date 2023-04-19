use bindings::inbound_http::{InboundHttp, Request, Response};
use std::str;

struct Component;

impl InboundHttp for Component {
    fn handle_request(req: Request) -> Response {
        // The content should be plain text
        let Some(content_type) = req
            .headers
            .iter()
            .find(|(k, _)| k == "content-type")
            .map(|(_, v)| v) else {
                return Response {
                    status: 400,
                    headers: None,
                    body: None,
                };
            };

        if content_type != "text/plain" {
            return Response {
                status: 400,
                headers: None,
                body: None,
            };
        }

        match req.body {
            Some(body) => {
                match str::from_utf8(&body) { // We assume the body is UTF-8 encoded
                    Err(error) => {
                        eprintln!("body is not a valid UTF8 string: {error}");
                        Response {
                            status: 400,
                            headers: None,
                            body: None,   
                        }
                    }
                    Ok(text) => {
                        Response {
                            headers: Some(vec![("content-type".to_string(), "text/plain".to_string())]),
                            body: Some(format!("The request body was: {text}").into_bytes()),
                            status: 200,
                        }
                    },
                }
            },
            None => {
                eprintln!("missing request body");
                Response {
                    status: 400,
                    headers: None,
                    body: None,   
                }   
            }
        }
    }
}

bindings::export!(Component);