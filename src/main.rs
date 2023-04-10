use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use tokio_tungstenite::WebSocketStream;
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let addr = ([127, 0, 0, 1], 8080).into();

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Server started on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    if let Some(ws_req) =
        tokio_tungstenite::tungstenite::handshake::server::Request::from_request(request)
    {
        let response = ws_req.into_response();
        let ws_stream = WebSocketStream::from_raw_socket(
            ws_req.into_stream(),
            tokio_tungstenite::tungstenite::protocol::Role::Server,
            None,
        )
        .await;
        // Handle WebSocket stream here.
        Ok(response)
    } else {
        let response = Response::builder()
            .status(400)
            .body(Body::from("Invalid request"))
            .unwrap();
        Ok(response)
    }
}
