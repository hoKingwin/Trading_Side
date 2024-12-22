mod stock;
mod brokers;
mod utils;
mod messaging;

use messaging::{connect_to_rabbitmq, send_stock_updates, consume_messages};
use stock::{initialize_stocks, apply_price_fluctuations, Stock};
use brokers::{process_broker_activities, BrokerActivity};
use utils::print_stock_list;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use chrono::{NaiveTime, Timelike};
use log::{info, error};
use futures_util::StreamExt;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting Stock Subsystem...");

    // RabbitMQ connection
    let channel = connect_to_rabbitmq()
        .await
        .expect("Failed to connect to RabbitMQ");

    // Initialize stocks
    let stocks = Arc::new(Mutex::new(initialize_stocks()));
    
    // Print initial stock list
    {
        let stocks_guard = stocks.lock().await;
        print_stock_list(&stocks_guard);
    }

    // Market timing (9:00 AM start)
    let mut current_time = NaiveTime::from_hms_opt(9, 0, 0).expect("Invalid time");

    // Spawn broker activities processor
    let stocks_clone = Arc::clone(&stocks);
    let result = channel.queue_declare("dummy_queue", Default::default(), Default::default()).await;

    match result {
        Ok(_) => {
            // If the operation succeeds, clone the channel and print a success message.
            println!("Connection is open and channel is created.");
        }
        Err(err) => {
            // If the operation fails (e.g., channel is closed), return the error.
            eprintln!("Failed to create channel: {:?}", err);
        }
    }
    let channel_clone = channel.clone();
    
    let _broker_handle = tokio::spawn(async move {
        process_broker_activities_from_queue(&channel_clone, &stocks_clone).await;
    });

    // Main market simulation loop
    loop {
        info!("Market time: {}", current_time.format("%I:%M %p"));

        // Update stocks and prices
        {
            let mut stocks_guard = stocks.lock().await;
            apply_price_fluctuations(&mut stocks_guard);
            print_stock_list(&stocks_guard);
        }

        // Send updates to Trading Side
        if let Err(e) = send_stock_updates(&stocks, &channel).await {
            error!("Failed to send stock updates: {:?}", e);
        } else {
            info!("Stock updates sent successfully");
        }

        // Wait for next cycle
        sleep(Duration::from_secs(10)).await;

        // Advance market time
        current_time = current_time
            .overflowing_add_signed(chrono::Duration::minutes(30))
            .0;

        // Check market close (4:00 PM)
        if current_time.hour() >= 16 {
            info!("Market closed at 4:00 PM");
            break;
        }
    }

    info!("Stock Subsystem stopped");
    Ok(())
}

async fn process_broker_activities_from_queue(
    channel: &lapin::Channel,
    stocks: &Arc<Mutex<HashMap<String, Stock>>>,
) {
    let mut consumer = consume_messages(channel, "broker_activities").await;

    while let Some(delivery) = consumer.next().await {
        if let Ok(message) = delivery {
            match serde_json::from_slice::<BrokerActivity>(&message.data) {
                Ok(broker_activity) => {
                    let activity_clone = broker_activity.clone();
                    
                    {
                        let mut stocks_guard = stocks.lock().await;
                        process_broker_activities(vec![broker_activity], &mut stocks_guard);
                    }

                    if let Err(e) = message.ack(lapin::options::BasicAckOptions::default()).await {
                        error!("Failed to acknowledge message: {:?}", e);
                    }

                    info!("Processed broker activity: {:?}", activity_clone);
                }
                Err(e) => error!("Failed to parse broker activity: {:?}", e),
            }
        }
    }
}