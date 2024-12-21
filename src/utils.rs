use crate::simulation::Stock;
use std::collections::HashMap;
use log::info;

pub fn print_stock_list(stocks: &[Stock]) {
    info!("=== Current Market Prices ===");
    println!("{:<10} {:>12} {:>12}", "Stock ID", "Price ($)", "Volume");
    println!("{}", "-".repeat(36));
    
    for stock in stocks {
        println!(
            "{:<10} {:>12.2} {:>12}",
            stock.id,
            stock.price,
            stock.available_quantity
        );
    }
    println!("{}", "-".repeat(36));
}

pub fn print_broker_holdings(broker_id: u32, cash: f64, holdings: &HashMap<String, usize>) {
    info!("=== Broker {} Status ===", broker_id);
    println!("Available Cash: ${:.2}", cash);
    if !holdings.is_empty() {
        println!("\nCurrent Holdings:");
        println!("{:<10} {:>12}", "Stock ID", "Quantity");
        println!("{}", "-".repeat(24));
        for (stock_id, quantity) in holdings {
            println!("{:<10} {:>12}", stock_id, quantity);
        }
    } else {
        println!("No current holdings");
    }
    println!("");
}

pub fn print_market_separator() {
    println!("{}", "-".repeat(50));
    println!("");
}