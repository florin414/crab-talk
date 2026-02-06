use crab_talk::user::{
    user_service_client::UserServiceClient,
    UserRequest,
    UserResponse,
};
pub struct WrappedUserResponse(pub UserResponse);
impl std::fmt::Debug for WrappedUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WrappedUserResponse")
            .field("user_id", &self.0.user_id)
            .field("name", &self.0.name)
            .field("email", &self.0.email)
            .finish()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        UserServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(UserRequest { user_id: 42 });

    let response = client.get_user(request).await?;

    let wrapped_response = WrappedUserResponse(response.into_inner());

    println!("User = {:?}", wrapped_response);

    Ok(())
}