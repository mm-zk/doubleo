use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use clap::{Parser, Subcommand};
use futures::{channel::oneshot, future, FutureExt};
use jsonrpc_core::{MetaIoHandler, Metadata};
use serde::{Deserialize, Serialize};

mod eth;
use crate::eth::EthNamespaceT;

mod proxy;
use crate::proxy::Proxy;

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

#[derive(Clone, Debug, Default)]
pub struct Meta();
impl Metadata for Meta {}

#[allow(clippy::too_many_arguments)]
async fn build_json_http(addr: SocketAddr, proxy: Proxy) -> tokio::task::JoinHandle<()> {
    let (sender, recv) = oneshot::channel::<()>();

    let io_handler = {
        let mut io: MetaIoHandler<Meta> = MetaIoHandler::default();
        io.extend_with(EthNamespaceT::to_delegate(proxy.clone()));
        io
    };

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(1)
            .build()
            .unwrap();

        let server = jsonrpc_http_server::ServerBuilder::new(io_handler)
            .threads(1)
            .event_loop_executor(runtime.handle().clone())
            .start_http(&addr)
            .unwrap();
        server.wait();
        let _ = sender.send(());
    });

    tokio::spawn(recv.map(drop))
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let opt = Cli::parse();
    let config = parse_config(&opt.config_file_path).unwrap();

    println!("config: {:?}", config);
    tracing_subscriber::fmt::init();

    let proxy = Proxy {
        sequencer_url: opt.sequencer_url,
    };

    let threads = build_json_http(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), opt.port),
        proxy,
    )
    .await;

    tracing::info!("========================================");
    tracing::info!("  Node is ready at 127.0.0.1:{}", opt.port);
    tracing::info!("========================================");

    future::select_all(vec![threads]).await.0.unwrap();
    Ok(())
}
