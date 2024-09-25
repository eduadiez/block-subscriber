use ansi_term::Colour;
use chrono::Utc;
use clap::Parser;
use ethers::prelude::*;
use ethers::providers::{Middleware, Provider};
use tracing::{event, span, Level};
use tracing_subscriber;

// Default RPC URL for connecting to the Ethereum node
const RPC_URL: &str = "ws://localhost:8546";

// Struct to handle command line arguments using clap
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = RPC_URL)]
    rpc_url: String, // RPC URL argument with a default value
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize the tracing subscriber based on the "trace" feature flag
    #[cfg(not(feature = "trace"))]
    tracing_subscriber::fmt().init();

    #[cfg(feature = "trace")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Create a new span for the main function
    let main_span = span!(Level::INFO, "main");
    let _enter = main_span.enter(); // Entra al span

    event!(Level::INFO, "Connecting ...");

    // Create a provider connected to the specified RPC URL
    let provider = Provider::<Ws>::connect(&args.rpc_url).await?;
    let chain_id = provider.get_chainid().await?;

    event!(
        Level::INFO,
        "Connected to the RPC: {} with chain_id: {}",
        &args.rpc_url,
        chain_id
    );

    // Subscribe to new blocks from the provider
    let mut stream = provider.subscribe_blocks().await?;

    while let Some(block) = stream.next().await {
        let current_timestamp: u64 = Utc::now().timestamp() as u64;
        let block_timestamp = block.timestamp.as_u64();
        let diff: u64 = current_timestamp - block_timestamp;

        let colored_diff = match diff {
            0..=9 => Colour::Green.paint(format!("{}s", diff)),
            10..=19 => Colour::Yellow.paint(format!("{}s", diff)),
            _ => Colour::Red.paint(format!("{}s", diff)),
        };

        #[cfg(feature = "gasPrice")]
        let gas_price = Some(provider.get_gas_price().await?);

        #[cfg(not(feature = "gasPrice"))]
        let gas_price: Option<U256> = None;

        event!(
            Level::INFO,
            "now: {:?} block: {:?}, diff {}, block number: {}{}",
            current_timestamp,
            block_timestamp,
            colored_diff,
            block.number.unwrap(),
            gas_price
                .map(|gp| format!(", gasPrice: {:?}", gp))
                .unwrap_or_default()
        );
    }

    Ok(())
}
