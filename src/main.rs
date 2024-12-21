mod simulation;
mod trading_strategy;
mod broker;
mod messaging;
mod utils;

use simulation::{run_trading_side, Stock};
use broker::Broker;
use trading_strategy::Strategy;
use messaging::{connect_to_rabbitmq, receive_stock_updates};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::info;
use env_logger::Env;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging first
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("[Trading_Side] Starting...");

    // Connect to RabbitMQ
    let channel = connect_to_rabbitmq().await?;
    info!("[Trading_Side] Connected to RabbitMQ");

    // Initialize shared state
    let stocks = Arc::new(Mutex::new(Vec::<Stock>::new()));
    let brokers = Arc::new(Mutex::new(vec![
        Broker::new(1, 10_000.0, Strategy::RiskAverse),
        Broker::new(2, 20_000.0, Strategy::Aggressive),
        Broker::new(3, 15_000.0, Strategy::Random),
    ]));

    // Start stock updates consumer first
    let stocks_clone = Arc::clone(&stocks);
    let channel_clone = channel.clone();
    tokio::spawn(async move {
        receive_stock_updates(&channel_clone, stocks_clone).await;
    });

    // Small delay to ensure consumer is ready
    sleep(Duration::from_millis(100)).await;
    info!("[Trading_Side] System Initialized");

    // Start main trading simulation
    run_trading_side(
        Arc::clone(&brokers),
        Arc::clone(&stocks),
        channel,
    ).await;

    Ok(())
}