use crate::broker::Broker;
use crate::trading_strategy::Action;
use crate::messaging::send_broker_action;
use crate::utils::print_stock_list;
use lapin::Channel;
use chrono::{NaiveTime, Timelike};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error};
use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub id: String,
    pub price: f64,
    pub available_quantity: usize,
}

pub async fn run_trading_side(
    brokers: Arc<Mutex<Vec<Broker>>>,
    stocks: Arc<Mutex<Vec<Stock>>>,
    channel: Channel,
) {
    let mut current_time = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
    
    info!("[System] Market Open at {}", current_time.format("%I:%M %p"));
    log_broker_accounts("Broker Accounts Before Market Open", &brokers).await;

    // Wait for initial stock data
    loop {
        let stocks_guard = stocks.lock().await;
        if !stocks_guard.is_empty() {
            info!("=== Initial Stock Prices ===");
            print_stock_list(&stocks_guard);
            break;
        }
        drop(stocks_guard);
        info!("Waiting for stock data...");
        sleep(Duration::from_secs(1)).await;
    }

    // Main trading loop
    while current_time.hour() < 16 {
        info!("\n=== Trading Round: {} ===", current_time.format("%I:%M %p"));
        
        // 1. Show updated broker accounts
        log_broker_accounts("Current Broker Accounts", &brokers).await;
        
        // 2. Show updated stock prices
        {
            let stocks_guard = stocks.lock().await;
            info!("=== Updated Stock Prices ===");
            print_stock_list(&stocks_guard);
        }
        
        // 3. Process broker actions
        info!("=== Broker Actions ===");
        perform_broker_actions(&stocks, &brokers, current_time, &channel).await;
        
        info!("----------------------------------------");
        
        sleep(Duration::from_secs(10)).await;
        current_time = current_time
            .overflowing_add_signed(chrono::Duration::minutes(30))
            .0;
    }

    info!("[System] Market Close at {}", current_time.format("%I:%M %p"));
}

async fn perform_broker_actions(
    stocks: &Arc<Mutex<Vec<Stock>>>,
    brokers: &Arc<Mutex<Vec<Broker>>>,
    _current_time: NaiveTime,  // Added underscore to unused variable
    channel: &Channel,
) {
    let mut brokers_locked = brokers.lock().await;
    let stocks_locked = stocks.lock().await;

    if stocks_locked.is_empty() {
        info!("No stocks available for trading");
        return;
    }

    let stocks_clone = stocks_locked.clone();
    let stocks_len = stocks_clone.len();

    for broker in brokers_locked.iter_mut() {
        sleep(Duration::from_millis(500)).await;
        let mut rng = thread_rng();
        
        let action = broker.strategy.decide_action(broker, &stocks_clone);
        match action {
            Action::Buy => {
                if stocks_len > 0 {
                    if let Some(stock) = stocks_clone.get(rng.gen_range(0..stocks_len)) {
                        let quantity = rng.gen_range(1..=5);
                        if broker.buy(stock, quantity).is_ok() {
                            if let Err(e) = send_broker_action(channel, broker.id, "Buy", &stock.id, quantity).await {
                                error!("Failed to send buy action: {:?}", e);
                            }
                            log_broker_action(broker.id, "Buy", &stock.id, quantity);
                        }
                    }
                }
            }
            Action::Sell => {
                if !broker.holdings.is_empty() && stocks_len > 0 {
                    if let Some(stock) = stocks_clone.get(rng.gen_range(0..stocks_len)) {
                        let quantity = rng.gen_range(1..=5);
                        if broker.sell(stock, quantity).is_ok() {
                            if let Err(e) = send_broker_action(channel, broker.id, "Sell", &stock.id, quantity).await {
                                error!("Failed to send sell action: {:?}", e);
                            }
                            log_broker_action(broker.id, "Sell", &stock.id, quantity);
                        }
                    }
                }
            }
            Action::Hold => {
                log_broker_action(broker.id, "Hold", "", 0);
            }
        }
    }
}

async fn log_broker_accounts(message: &str, brokers: &Arc<Mutex<Vec<Broker>>>) {
    let brokers_guard = brokers.lock().await;
    info!("   === {} ===", message);
    for broker in brokers_guard.iter() {
        info!("        {}", broker);
    }
    info!("");
}

fn log_broker_action(broker_id: u32, action: &str, stock_id: &str, quantity: usize) {
    match action {
        "Hold" => info!("Broker {} is holding position", broker_id),
        _ => info!(
            "Broker {} {}s {} units of stock {}",
            broker_id, action, quantity, stock_id
        ),
    }
}