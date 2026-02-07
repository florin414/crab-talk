use tonic::{Request, Response, Status};

use crab_talk_proto::user::{
    user_service_server::UserService,
    UserRequest,
    UserResponse,
};

#[derive(Default)]
pub struct CrabTalkUserService;

#[tonic::async_trait]
impl UserService for CrabTalkUserService {
    async fn get_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let span = tracing::info_span!("grpc_request", grpc.method = "GetUser", user_id = %request.get_ref().user_id);
        let _enter = span.enter();

        tracing::info!("Handling GetUser request");

        let user_id = request.into_inner().user_id;

        let response = UserResponse {
            user_id,
            name: "user1".into(),
            email: "user1@exemple.com".into(),
        };

        Ok(Response::new(response))
    }
}

