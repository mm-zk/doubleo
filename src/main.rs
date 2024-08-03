use std::{
    borrow::Cow,
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use clap::{Parser, Subcommand};
use futures::{channel::oneshot, future, FutureExt};
use middleware::AuthMiddlewareLayer;
use proxy::{PrivateEthNamespaceServer, PrivateProxy};
use serde::{Deserialize, Serialize};
use serde_json::{json, value::RawValue, Value};
use tower::Service;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use zksync_web3_decl::jsonrpsee::{
    core::async_trait,
    server::{ServerBuilder, TowerServiceBuilder},
};
use zksync_web3_decl::{jsonrpsee::RpcModule, namespaces::EthNamespaceServer};

mod proxy;
use crate::proxy::Proxy;
use base64::decode;
use futures::future::BoxFuture;
use hyper::{Body, Request, Response};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::Layer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use zksync_web3_decl::jsonrpsee::http_client::types::Request as JsonRpcRequest;
//use zksync_web3_decl::jsonrpsee::http_server::{
//    HttpServerBuilder, HttpServerMiddleware, Request as JsonRpcRequest, Response as JsonRpcResponse,
//};
use zksync_web3_decl::jsonrpsee::proc_macros::rpc;
use zksync_web3_decl::jsonrpsee::types::Params;

mod middleware;
#[derive(Debug, Parser)]
#[command(author = "Matter Labs", version, about = "Proxy", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(long, default_value = "8015")]
    /// Port to listen on - default: 8011
    port: u16,

    #[arg(long)]
    sequencer_url: String,

    #[arg(long, default_value = "config.yaml")]
    config_file_path: String,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(name = "run")]
    Run,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    // If enabled, allows anyone to create new contracts.
    allow_contract_creation: bool,

    // List of contracts that can be called with 'call'.
    contract_call_whitelist: Vec<String>,
}

fn parse_config(path: &str) -> eyre::Result<Config> {
    let file_content = fs::read_to_string(path).expect("Unable to read file");

    // Parse the YAML string into the Config struct
    let config: Config = serde_yaml::from_str(&file_content)?;

    return Ok(config);
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let opt = Cli::parse();
    let config = parse_config(&opt.config_file_path).unwrap();

    println!("config: {:?}", config);
    tracing_subscriber::fmt::init();

    let proxy = Proxy {
        sequencer_url: opt.sequencer_url.clone(),
    };

    let private_proxy = PrivateProxy {
        sequencer_url: opt.sequencer_url,
    };

    let mut rpc = RpcModule::new(());
    rpc.merge(proxy.into_rpc()).unwrap();
    rpc.merge(private_proxy.into_rpc()).unwrap();

    let http_middleware = tower::ServiceBuilder::new().layer(AuthMiddlewareLayer {});

    // Create the server with custom middleware
    let builder = ServerBuilder::default();
    let server = builder
        .set_http_middleware(http_middleware)
        .http_only()
        .build(format!("127.0.0.1:{:?}", opt.port))
        .await
        .unwrap();

    let handle = server.start(rpc);

    tracing::info!("========================================");
    tracing::info!("  Node is ready at 127.0.0.1:{}", opt.port);
    tracing::info!("========================================");

    // Wait for the server to finish
    handle.stopped().await;

    Ok(())
}
