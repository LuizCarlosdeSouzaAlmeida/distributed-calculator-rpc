// src/client.rs
use tarpc::context;
use tokio::net::TcpStream;
use tokio_serde::formats::Bincode;
use tokio_util::codec::LengthDelimitedCodec;

mod service;
use service::HelloClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("localhost:10000").await?;
    let transport = tarpc::serde_transport::new(
        tokio_util::codec::Framed::new(stream, LengthDelimitedCodec::new()),
        Bincode::default(),
    );

    let client = HelloClient::new(tarpc::client::Config::default(), transport).spawn()?;

    let response = client.hello(context::current(), "World".to_string()).await?;
    println!("Response: {}", response);

    Ok(())
}