use frodobuf::actor::prelude::*;
use http_server::http_server::{HttpRequest, HttpResponse, HttpServer, HttpServerServer};
use serde_json::json;
use system::system::{Actor, ActorServer, HealthCheckRequest, HealthCheckResponse};

#[derive(Default, Debug, FrodobufActor)]
#[services(Actor, HttpServer)]
struct EchoActor {}

/// Implementation of Echoes server methods
#[async_trait]
impl HttpServer for EchoActor {
    async fn handle_request(
        &self,
        _ctx: &context::Context<'_>,
        value: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let body = json!({
            "method": &value.method,
            "path": &value.path,
            "query_string": &value.query_string,
            "body": b"OK".to_vec(),
        });
        Ok(HttpResponse {
            status_code: 200,
            status: "OK".to_string(),
            header: Default::default(),
            body: serde_json::to_vec(&body)
                .map_err(|e| RpcError::ActorHandler(format!("serializing response: {}", e)))?,
        })
    }
}

#[async_trait]
impl Actor for EchoActor {
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
