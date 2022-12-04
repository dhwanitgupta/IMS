mod operations;
mod state;

use aws_smithy_http_server::{AddExtensionLayer, Router};
use clap::Parser;

use liwimean_ims_core_server_sdk::operation_registry::OperationRegistryBuilder;

use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::operations::get_aggregated_transaction_analysis_by_filters::get_aggregated_transaction_analysis_by_filters;
use crate::operations::list_transactions::do_list_transactions;
use crate::operations::start_session::do_start_session;
use crate::state::State;
use dashmap::DashMap;
use env_logger::Env;
use std::process;
use tower::layer::Layer;
use tower::make::Shared;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,
    #[clap(short, long, default_value = "12446")]
    port: u16,
    #[clap(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let env = Env::default().filter_or("MY_LOG_LEVEL", args.log_level);

    env_logger::init_from_env(env);

    let app: Router = OperationRegistryBuilder::default()
        .list_transactions(do_list_transactions)
        .start_session(do_start_session)
        .get_aggregated_transaction_analysis_by_filters(
            get_aggregated_transaction_analysis_by_filters,
        )
        .build()
        .expect("Unable to build operation registry")
        .into();

    let shared_state = Arc::new(State {
        resources_map: DashMap::new(),
    });

    let _state_clone = Arc::clone(&shared_state);

    let app = app.layer(ServiceBuilder::new().layer(AddExtensionLayer::new(shared_state)));

    let core_endpoint = format!("{}:{}", args.address, args.port);

    let bind: SocketAddr = core_endpoint
        .parse()
        .expect("unable to parse the server bind address and port");

    let cors = CorsLayer::permissive().layer(app);

    let server = hyper::Server::bind(&bind).serve(Shared::new(cors));

    info!("Server running on {} (PID {})", bind, process::id());

    if let Err(err) = server.await {
        error!("Server error: {}", err)
    };
}
