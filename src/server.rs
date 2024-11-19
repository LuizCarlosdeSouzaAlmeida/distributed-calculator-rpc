// src/server.rs
use tarpc::server::{self, Channel};
use tokio::net::TcpListener;
use tokio_serde::formats::Bincode;

mod service;
use service::{HelloServer, Hello};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("localhost:10000").await?;
    println!("Listening on {}", listener.local_addr()?);

    let server = HelloServer;

    server::incoming(listener, None)
        .filter_map(|r| async move { r.ok() })
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = server.clone();
            channel.execute(server.serve())
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}