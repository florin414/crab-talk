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
        let user_id = request.into_inner().user_id;

        let reply = UserResponse {
            user_id,
            name: "user1".into(),
            email: "user1@exemple.com".into(),
        };

        Ok(Response::new(reply))
    }
}

