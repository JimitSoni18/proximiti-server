use tonic::{transport::Server, Request, Response};

mod auth {
    use tonic::include_proto;

	include_proto!("auth_service");
}

#[derive(Default)]
pub struct AuthService;

#[tokio::main]
async fn main() {
}
