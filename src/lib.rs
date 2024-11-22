#[tarpc::service]
pub trait Calculator {
    async fn add(a: i32, b: i32) -> i32;
    async fn sub(a: i32, b: i32) -> i32;
    async fn mult(a: i32, b: i32) -> i32;
    async fn div(a: i32, b: i32) -> Result<i32, String>;
}
