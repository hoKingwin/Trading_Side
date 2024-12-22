use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Consumer,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::stock::Stock;

pub async fn connect_to_rabbitmq() -> Result<Channel, lapin::Error> {
    let addr = "amqp://guest:guest@localhost:5672";
    let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    Ok(channel)
}

pub async fn send_stock_updates(
    stocks: &Arc<Mutex<HashMap<String, Stock>>>,
    channel: &Channel,
) -> Result<(), lapin::Error> {
    let stocks_guard = stocks.lock().await;
    
    // Convert HashMap to Vec for consistent format
    let stock_list: Vec<Stock> = stocks_guard.values().cloned().collect();
    let payload = serde_json::to_vec(&stock_list).unwrap();
    
    channel
        .basic_publish(
            "",
            "stock_updates",
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await?;
    
    Ok(())
}

pub async fn consume_messages(channel: &Channel, queue_name: &str) -> Consumer {
    channel
        .basic_consume(
            queue_name,
            "stock_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to create consumer")
}