use std::fs;

use clap::{Parser, Subcommand};
use middleware::AuthMiddlewareLayer;
use proxy::{PrivateEthNamespaceServer, PrivateProxy};
use serde::Deserialize;
use whitelist::ContractWhitelist;
use zksync_web3_decl::jsonrpsee::server::ServerBuilder;
use zksync_web3_decl::{jsonrpsee::RpcModule, namespaces::EthNamespaceServer};
mod proxy;
mod whitelist;
use crate::proxy::Proxy;

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

#[derive(Debug, Deserialize, Clone)]
pub struct WhitelistEntry {
    address: String,
    fully_whitelisted: bool,
    methods: Option<Methods>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Methods {
    unrestricted: Option<Vec<String>>,
    requires_authorization: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct Config {
    // If enabled, allows anyone to create new contracts.
    allow_contract_creation: bool,

    // List of contracts that can be called with 'call'.
    whitelist: Vec<WhitelistEntry>,
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
        whitelist: ContractWhitelist::init(config.whitelist.clone()),
    };

    let private_proxy = PrivateProxy {
        sequencer_url: opt.sequencer_url,
        whitelist: ContractWhitelist::init(config.whitelist),
        credentials: Default::default(),
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
