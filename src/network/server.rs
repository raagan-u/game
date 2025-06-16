use axum::{extract::{ws::WebSocket, WebSocketUpgrade}, http::Response};

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub async fn run(&self) {
        let app = axum::Router::new()
            .route("/", axum::routing::get(|| async { "Hello, World!" }));
        
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
    
    async fn handler(&self, ws: WebSocketUpgrade) -> Response<> {
        ws.on_upgrade(self.handle_socket)
    }
    
    async fn handle_socket(&self, mut socket: WebSocket) {
        while let Some(msg) = socket.recv().await {
            let msg = if let Ok(msg) = msg {
                msg
            } else {
                // client disconnected
                return;
            };
    
            if socket.send(msg).await.is_err() {
                // client disconnected
                return;
            }
        }
    }
}
