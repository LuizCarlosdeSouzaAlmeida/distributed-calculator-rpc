use futures::{future, prelude::*};
use service::Calculator;
use std::
    net::{IpAddr, Ipv4Addr}
;
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};

#[derive(Clone)]
struct CalculatorServer();

impl Calculator for CalculatorServer {
    async fn add(self, _: context::Context, a: i32, b: i32) -> i32 {
        let result = a + b;
        println!("add: {} + {} = {}", a, b, result);
        result
    }

    async fn sub(self, _: context::Context, a: i32, b: i32) -> i32 {
        let result = a - b;
        println!("sub: {} - {} = {}", a, b, result);
        result
    }

    async fn mult(self, _: context::Context, a: i32, b: i32) -> i32 {
        let result = a * b;
        println!("mult: {} * {} = {}", a, b, result);
        result
    }

    async fn div(self, _: context::Context, a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Division by zero".into())
        } else {
            let result = a / b;
            println!("div: {} / {} = {}", a, b, result);
            Ok(result)
        }
    }
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = 15000;
    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), port);

    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = CalculatorServer();
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
