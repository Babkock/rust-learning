#![deny(warnings)]

use futures_util::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

/// Service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as `curl localhost:3000/echo -XPOST -d 'hello world'`"
        ))),

        // echo the body back to the client
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        // convert to uppercase before sending back to client using a stream
        (&Method::POST, "/echo/uppercase") => {
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
        }

        // reverse the entire body
        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
        }

        // return the 404 not found for other routes
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

