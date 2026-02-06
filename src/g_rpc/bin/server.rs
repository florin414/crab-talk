use tonic::{transport::Server};
use crab_talk::user::user_service_server::{UserServiceServer};
use crab_talk::g_rpc::user_service::CrabTalkUserService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = CrabTalkUserService::default();

    println!("GRPC server listening on {}", addr);

    Server::builder().add_service(UserServiceServer::new(service)).serve(addr).await?;

    Ok(())
}