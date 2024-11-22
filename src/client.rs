use service::CalculatorClient;
use std::{io, net::SocketAddr, time::Duration};
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::time::sleep;



#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let server_addr: SocketAddr = "3.225.60.216".parse()?;

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client = CalculatorClient::new(client::Config::default(), transport.await?).spawn();

    loop {
        println!("Enter operation (add, subtract, multiply, divide) or 'exit' to quit:");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation)?;
        let operation = operation.trim();

        if operation == "exit" {
            break;
        }

        println!("Enter first operand:");
        let mut operand1 = String::new();
        io::stdin().read_line(&mut operand1)?;
        let operand1: i32 = operand1.trim().parse()?;

        println!("Enter second operand:");
        let mut operand2 = String::new();
        io::stdin().read_line(&mut operand2)?;
        let operand2: i32 = operand2.trim().parse()?;

        let result = match operation {
            "add" => client.add(context::current(), operand1, operand2).await,
            "subtract" => client.sub(context::current(), operand1, operand2).await,
            "multiply" => client.mult(context::current(), operand1, operand2).await,
            "divide" => client.div(context::current(), operand1, operand2).await.map(|res| res.unwrap_or_else(|e| {
                println!("Error: {}", e);
                0
            })),
            _ => {
                println!("Invalid operation");
                continue;
            }
        };

        match result {
            Ok(res) => println!("Result: {}", res),
            Err(e) => println!("Error: {:?}", e),
        }

        sleep(Duration::from_micros(1)).await;
    }

    Ok(())
}