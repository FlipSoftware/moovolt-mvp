use std::io;

use axum::{extract::ws::{Message, WebSocket, WebSocketUpgrade},
        response::IntoResponse, routing::get, Router};

use axum::extract::Path;

use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::info;

mod rpc;
use crate::rpc::messages::OcppMessageType;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();

    let router = Router::new()
        .route("/ws/:station_id", get(ws_connect))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    const ADDR: &str = "0.0.0.0:3000";
    let listener = TcpListener::bind(ADDR).await?;
    info!("[ SERVER ] Running on address: {ADDR}");
    axum::serve(listener, router).await?;

    Ok(())
}

fn station_valid_check(station: i32) -> io::Result<()> {
    match station {
        1 => io::Result::Ok(()),
        _ => io::Result::Err(io::Error::from(io::ErrorKind::InvalidInput)),
    }
}

async fn ws_connect(
    Path(station): Path<i32>,
    ws: WebSocketUpgrade
) -> impl IntoResponse {
    tracing::info!("Incoming connection from station {}", station);

    match station_valid_check(station) {
        Ok(_) => {
            ws.on_upgrade(handle_socket)
        }
        Err(_) => {
            ws.on_upgrade(handle_error)
        }
    }
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    tracing::info!("client sent str: {:?}", t);
                    let msg = r#"[2,"19223201","BootNotification",{"reason":"PowerUp","chargingStation":{"model":"SingleSocketCharger", "vendorName":"VendorX"}}]"#.to_string();
                    let ocpp_message_type = serde_json::from_str::<OcppMessageType>(&msg).unwrap();
                    tracing::info!("client sent str: {:?}", ocpp_message_type);
                }
                Message::Binary(_) => {
                    tracing::info!("client sent binary data");
                }
                Message::Ping(_) => {
                    tracing::info!("socket ping");
                }
                Message::Pong(_) => {
                    tracing::info!("socket pong");
                }
                Message::Close(_) => {
                    tracing::info!("client disconnected");
                    return;
                }
            }
        } else {
            tracing::info!("client disconnected");
            return;
        }
    }

    loop {
        if socket
            .send(Message::Text(String::from("Testing string message")))
            .await
            .is_err()
        {
            tracing::info!("Client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

async fn handle_error(mut socket: WebSocket) {

    if socket
        .send(Message::Text(String::from("Not a valid station")))
        .await
        .is_err()
    {
        tracing::info!("Client disconnected");
        return;
    }
    tracing::info!("Closing socket due to invalid station");
    let _ = socket.close().await;
}
