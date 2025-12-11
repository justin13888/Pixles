use std::sync::Arc;

use axum::Router;
use config::MetadataServerConfig;
use sea_orm::DatabaseConnection;
use tonic::service::Routes;
use tonic::{Request, Response, Status};

use eyre::Result;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tracing::{debug, info};

mod config;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default, Debug)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[tracing::instrument]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        debug!("sending response: {:?}", reply);

        Ok(Response::new(reply))
    }
}

// TODO: flesh this out vv
pub async fn get_router<C: Into<MetadataServerConfig>>(
    conn: Arc<DatabaseConnection>,
    config: C,
) -> Result<Router> {
    let greeter = MyGreeter::default();

    Ok(Routes::new(GreeterServer::new(greeter)).into_axum_router())
}
