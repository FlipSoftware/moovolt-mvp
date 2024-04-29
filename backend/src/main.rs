use std::io::Result;

use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    const ADDR: &str = "0.0.0.0:3000";
    let router = Router::new().route("/", get(root));
    let listener = TcpListener::bind(ADDR).await?;

    info!("[ SERVER ] Running on address: {ADDR}");
    axum::serve(listener, router).await?;

    Ok(())
}

async fn root() -> Html<&'static str> {
    Html::from(
        "<nav><h1>Moovolt</h1></nav>
        <main>Content</main>
        ",
    )
}
