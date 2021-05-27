use calc::calc::{
    Binary,
    Calculator,
    CalculatorClient,
    Nary, // Calc, CalcServer
};
use frodobuf::actor::prelude::*;
use http_server::http_server::{HttpRequest, HttpResponse, HttpServer, HttpServerServer};
use serde_json::json;
use system::system::{Actor, ActorServer, HealthCheckRequest, HealthCheckResponse};

const CALCULATOR_ID: &str = "wasmcloud:calculator";

#[derive(Default, Debug, FrodobufActor)]
#[services(Actor, HttpServer)]
struct CalcActor {}

#[async_trait]
impl HttpServer for CalcActor {
    async fn handle_request(
        &self,
        ctx: &context::Context<'_>,
        value: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        // upon receiving http request, send
        let client = CalculatorClient::new(
            client::ClientConfig::target(CALCULATOR_ID),
            WasmHost::default(),
        );

        // upon receiving http request, parse url and send parameters
        let path = value.path.strip_prefix('/').unwrap_or(&value.path);
        let paths = path.split('/').collect::<Vec<&str>>();
        let (op, terms) = paths.split_at(1);
        let result = match op[0] {
            "add" => client.add(ctx, &to_nary(floats(terms, false)?)).await?,
            "sub" => client.sub(ctx, &to_binary(floats(terms, false)?)).await?,
            "mul" => client.mul(ctx, &to_nary(floats(terms, false)?)).await?,
            "div" => client.div(ctx, &to_binary(floats(terms, false)?)).await?,
            "pow" => client.pow(ctx, &to_binary(floats(terms, false)?)).await?,
            _ => {
                return Ok(HttpResponse {
                    status_code: 400,
                    status: "OK".to_string(),
                    header: Default::default(),
                    body: serde_json::to_vec(&json!({ "error": "invalid request" }))
                        .map_err(|e| RpcError::Ser(e.to_string()))?,
                });
            }
        };

        Ok(HttpResponse {
            status_code: 200,
            status: "OK".to_string(),
            header: Default::default(),
            body: serde_json::to_vec(&json!({ "result": result }))
                .map_err(|e| RpcError::Ser(e.to_string()))?,
        })
    }
}

type ParseResults = Vec<Result<f64, core::num::ParseFloatError>>;

fn floats(paths: &[&str], is_binary: bool) -> Result<Vec<f64>, RpcError> {
    let (good, errs): (ParseResults, ParseResults) = paths
        .iter()
        .map(|p| p.parse::<f64>())
        .partition(|r| r.is_ok());
    if !errs.is_empty() {
        let errs = errs
            .into_iter()
            .map(|e| e.as_ref().err().as_ref().unwrap().to_string())
            .collect::<Vec<String>>()
            .join(",");

        return Err(RpcError::InvalidParameter(format!(
            "Illegal parameter: some values were not float: {}",
            errs
        )));
    }
    if is_binary && good.len() > 2 {
        return Err(RpcError::InvalidParameter(
            "Only two values may allowed for tihs operation".to_string(),
        ));
    }
    if good.len() < 2 {
        return Err(RpcError::InvalidParameter(
            "At least two parameters are required".to_string(),
        ));
    }
    Ok(good.into_iter().map(|v| v.ok().unwrap()).collect())
}

fn to_binary(v: Vec<f64>) -> Binary {
    // already validated to have exactly 2 members
    assert_eq!(v.len(), 2usize);
    Binary {
        left: *v.get(0).unwrap(),
        right: *v.get(1).unwrap(),
    }
}

fn to_nary(v: Vec<f64>) -> Nary {
    Nary { terms: v }
}

/// Implementation of Actor server methods
#[async_trait]
impl Actor for CalcActor {
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
