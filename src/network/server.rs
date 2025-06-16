use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}},
    response::Response,
    routing::get,
    Router,
};
use std::net::SocketAddr;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub async fn run(&self) {
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/ws", get(Self::ws_handler));

        let addr = SocketAddr::from(([0, 0, 0, 0], 1234));
        println!("Listening on {}", addr);
        axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
    }

    async fn ws_handler(ws: WebSocketUpgrade) -> Response {
        ws.on_upgrade(handle_socket)
    }
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(result) = socket.recv().await {
        match result {
            Ok(msg) => {
                if socket.send(msg).await.is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server() {
        let server = Server::new();
        server.run().await;
    }
}
