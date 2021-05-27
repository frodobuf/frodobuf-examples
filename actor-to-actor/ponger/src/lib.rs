use frodobuf::actor::prelude::*;
use ping::ping::{Ping, Pong, Ponger, PongerServer};
use wasmcloud_system_interface::system::{
    Actor, ActorServer, HealthCheckRequest, HealthCheckResponse,
};

#[derive(Default, Debug, FrodobufActor)]
#[services(Ponger, Actor)]
struct PongActor {}

/// Implementation of Ponger server methods
#[async_trait]
impl Ponger for PongActor {
    async fn ping(
        &self,
        _ctx: &context::Context<'_>,
        _value: &Ping,
    ) -> std::result::Result<Pong, RpcError> {
        Ok(Pong {
            value: "Hello back to ya".to_string(),
        })
    }
}

/// Implementation of Actor server methods
#[async_trait]
impl Actor for PongActor {
    async fn health_request(
        &self,
        _ctx: &context::Context<'_>,
        _value: &HealthCheckRequest,
    ) -> std::result::Result<HealthCheckResponse, RpcError> {
        Ok(HealthCheckResponse {
            healthy: true,
            message: String::from("ok"),
        })
    }
}
