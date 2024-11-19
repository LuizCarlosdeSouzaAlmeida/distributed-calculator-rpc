// src/service.rs
use tarpc::context;

#[derive(Clone)]
pub struct HelloServer;

#[tarpc::service]
pub trait Hello {
    async fn hello(name: String) -> String;
}

#[tarpc::server]
impl Hello for HelloServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        format!("Hello, {}!", name)
    }
}