use std::collections::HashMap;
use crate::stock::Stock; // Use the Stock struct from stock.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerActivity {
    pub broker_id: u32,
    pub stock_id: String,
    pub action: String,
    pub quantity: usize,
}

pub fn process_broker_activities(
    broker_activities: Vec<BrokerActivity>,
    stocks: &mut HashMap<String, Stock>,
) {
    for activity in broker_activities {
        if let Some(stock) = stocks.get_mut(&activity.stock_id) {
            match activity.action.as_str() {
                "Buy" => {
                    if stock.available_quantity >= activity.quantity {
                        stock.available_quantity -= activity.quantity;
                        println!(
                            "Broker {} bought {} shares of {}.",
                            activity.broker_id, activity.quantity, activity.stock_id
                        );
                    } else {
                        println!(
                            "Broker {} failed to buy shares of {} (not enough available).",
                            activity.broker_id, activity.stock_id
                        );
                    }
                }
                "Sell" => {
                    stock.available_quantity += activity.quantity;
                    println!(
                        "Broker {} sold {} shares of {}.",
                        activity.broker_id, activity.quantity, activity.stock_id
                    );
                }
                _ => {
                    println!("Invalid action: {}", activity.action);
                }
            }
        } else {
            println!(
                "Broker {} attempted to trade an unknown stock: {}.",
                activity.broker_id, activity.stock_id
            );
        }
    }
}
