use lapin::{
    options::*, 
    types::FieldTable, 
    BasicProperties, 
    Channel, 
    Connection,
    ConnectionProperties,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::simulation::Stock;
use log::{info, error};
use futures_util::StreamExt;

const STOCK_UPDATES_QUEUE: &str = "stock_updates";
const BROKER_ACTIVITIES_QUEUE: &str = "broker_activities";

pub async fn connect_to_rabbitmq() -> Result<Channel, lapin::Error> {
    let addr = "amqp://guest:guest@localhost:5672";
    let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    // Declare queues
    channel
        .queue_declare(
            STOCK_UPDATES_QUEUE,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_declare(
            BROKER_ACTIVITIES_QUEUE,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(channel)
}

pub async fn send_broker_action(
    channel: &Channel,
    broker_id: u32,
    action: &str,
    stock_id: &str,
    quantity: usize,
) -> Result<(), lapin::Error> {
    let payload = serde_json::json!({
        "broker_id": broker_id,
        "action": action,
        "stock_id": stock_id,
        "quantity": quantity
    });

    channel
        .basic_publish(
            "",
            BROKER_ACTIVITIES_QUEUE,
            BasicPublishOptions::default(),
            &serde_json::to_vec(&payload).unwrap(),
            BasicProperties::default(),
        )
        .await?;

    Ok(())
}

pub async fn receive_stock_updates(channel: &Channel, stocks: Arc<Mutex<Vec<Stock>>>) {
    let mut consumer = match channel
        .basic_consume(
            STOCK_UPDATES_QUEUE,
            "trading_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
    {
        Ok(consumer) => consumer,
        Err(err) => {
            error!("Failed to create consumer: {:?}", err);
            return;
        }
    };

    info!("Waiting for stock updates...");
    
    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            if let Ok(updates) = serde_json::from_slice::<Vec<Stock>>(&delivery.data) {
                let mut stocks_guard = stocks.lock().await;
                *stocks_guard = updates;
                info!("Received stock updates");
            }
            
            if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
                error!("Failed to acknowledge message: {:?}", err);
            }
        }
    }
}