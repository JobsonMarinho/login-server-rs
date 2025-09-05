
use tonic::{Request, Response, Status};
use crate::state::SharedState;
use crate::models::{Character as CharModel, World as WorldModel};

pub mod pb {
    tonic::include_proto!("login");
}
use pb::{login_server::{Login, LoginServer}, LoginRequest, LoginReply, Character, World};

#[derive(Clone)]
pub struct LoginSvc {
    state: SharedState,
}

impl LoginSvc {
    pub fn new(state: SharedState) -> Self { Self { state } }
    pub fn into_server(self) -> LoginServer<LoginSvc> { LoginServer::new(self) }
}

#[tonic::async_trait]
impl Login for LoginSvc {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        let req = request.into_inner();
        let ip = req.ip;
        let account = req.account;
        let password = req.password;

        let (chars, world) = self.state.login(&account, &password).await
            .map_err(|e| Status::unauthenticated(e.to_string()))?;

        let reply = LoginReply {
            ok: true,
            message: format!("ok (ip={})", ip),
            characters: chars.into_iter().map(to_pb_char).collect(),
            world: Some(to_pb_world(world)),
        };
        Ok(Response::new(reply))
    }
}

fn to_pb_char(c: CharModel) -> Character {
    Character { name: c.name, level: c.level, vocation: c.vocation }
}
fn to_pb_world(w: WorldModel) -> World {
    World { name: w.name, ip: w.ip, port: w.port as u32, location: w.location }
}
