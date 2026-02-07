use tonic::transport::Server;

use crab_talk_proto::user::user_service_server::UserServiceServer;

mod user_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = user_service::CrabTalkUserService::default();

    println!("GRPC server listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

