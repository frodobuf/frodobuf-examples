use frodobuf::actor::prelude::*;
use http_server::http_server::{HttpRequest, HttpResponse, HttpServer, HttpServerServer};
use ping::ping::{Ping, Ponger, PongerClient};
use serde_json::json;
use wasmcloud_system_interface::system::{
    Actor, ActorServer, HealthCheckRequest, HealthCheckResponse,
};

#[derive(Default, Debug, FrodobufActor)]
#[services(HttpServer, Actor)]
struct PingActor {}

const PONGER_ID: &str = "frodobuf_examples/actor_to_actor";

/// Implementation of HttpServer methods
#[async_trait]
impl HttpServer for PingActor {
    async fn handle_request(
        &self,
        _ctx: &context::Context<'_>,
        _value: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        // upon receiving http request, send ping to Ponger
        let client =
            PongerClient::new(client::ClientConfig::target(PONGER_ID), WasmHost::default());
        // send ping and wait for response
        let res = client
            .ping(
                &context::Context::default(),
                &Ping {
                    value: "Hello Ponger!".to_string(),
                },
            )
            .await?;

        // respond to http caller
        Ok(HttpResponse {
            status_code: 200,
            status: "OK".to_string(),
            header: Default::default(),
            body: serde_json::to_vec(&json!({ "response": res.value }))
                .map_err(|e| RpcError::Ser(e.to_string()))?,
        })
    }
}

/// Implementation of Actor server methods
#[async_trait]
impl Actor for PingActor {
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
