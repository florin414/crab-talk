pub mod user_service;
pub mod telemetry;

use tonic::transport::Server;

use crab_talk_proto::user::user_service_server::UserServiceServer;
use crate::telemetry::{init_tracing, trace_fn_adapter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing()?;

    let addr = "[::1]:50051".parse()?;
    let service = user_service::CrabTalkUserService::default();

    println!("GRPC server listening on {}", addr);

    Server::builder()
        .trace_fn(|req| trace_fn_adapter(req))  // attach interceptor
        .add_service(UserServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
