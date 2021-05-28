use async_trait::async_trait;
use calc::calc::{Binary, Calculator, CalculatorServer, Nary, CalcResult};
use frodobuf::provider::prelude::*;
use std::sync::{Arc, RwLock};
use wasmcloud_provider_core as provider;
use wasmcloud_system_interface::system::{
    HealthCheckRequest, HealthCheckResponse, Provider, ProviderServer,
};

//const CAPABILITY_ID: &str = "wasmcloud:calculator";

#[derive(Clone, FrodobufProvider)]
#[services(Calculator, Provider)]
struct CalcProvider {
    dispatcher: Arc<RwLock<Option<Box<dyn provider::Dispatcher>>>>,
}

impl Default for CalcProvider {
    fn default() -> CalcProvider {
        CalcProvider {
            dispatcher: Arc::new(RwLock::new(None)),
        }
    }
}

provider::capability_provider!(CalcProvider, CalcProvider::default);

/// Implementation of Calculator server methods
#[async_trait]
impl Calculator for CalcProvider {
    async fn add(
        &self,
        _ctx: &context::Context<'_>,
        value: &Nary,
    ) -> std::result::Result<CalcResult, RpcError> {
        let mut sum: f64 = 0.0;
        for v in value.terms.iter() {
            sum += v;
        }
        Ok(CalcResult{value:sum})
    }

    async fn sub(
        &self,
        _ctx: &context::Context<'_>,
        value: &Binary,
    ) -> std::result::Result<CalcResult, RpcError> {
        Ok(CalcResult{value: (value.left - value.right) })
    }


async fn mul(
        &self,
        _ctx: &context::Context<'_>,
        value: &Nary,
    ) -> std::result::Result<CalcResult, RpcError> {
        let mut product: f64 = 1.0;
        for v in value.terms.iter() {
            product *= v;
        }
        Ok(CalcResult{value:product})
    }

    async fn div(
        &self,
        _ctx: &context::Context<'_>,
        value: &Binary,
    ) -> std::result::Result<CalcResult, RpcError> {
        if value.right == 0.0 {
            Err(RpcError::InvalidParameter("division by zero".to_string()))
        } else {
            Ok(CalcResult{value:value.left / value.right})
        }
    }

    async fn pow(
        &self,
        _ctx: &context::Context<'_>,
        value: &Binary,
    ) -> std::result::Result<CalcResult, RpcError> {
        Ok(CalcResult{value: value.left.powf(value.right)})
    }
}

/// Implementation of Actor server methods
#[async_trait]
impl Provider for CalcProvider {
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

    // Calculator doesn't need these
    // We might implement these if we needed to authorize connections,
    // or keep track of actors for state or callbacks
    async fn bind_actor(
        &self,
        _ctx: &context::Context<'_>,
        _value: &String,
    ) -> std::result::Result<(), RpcError> {
        Ok(())
    }

    async fn remove_actor(
        &self,
        _ctx: &context::Context<'_>,
        _value: &String,
    ) -> std::result::Result<(), RpcError> {
        Ok(())
    }
}

impl CalcProvider {
    /// This function is called to let the capability provider know that it is being removed
    /// from the host runtime. This gives the provider an opportunity to clean up any
    /// resources and stop any running threads.
    /// WARNING: do not do anything in this function that can
    /// cause a panic, including attempting to write to STDOUT while the host process is terminating
    fn stop(&self) {}
}
