use crate::stock::Stock;
use std::collections::HashMap;

pub fn print_stock_list(stocks: &HashMap<String, Stock>) {
    println!("{:<10} {:>12} {:>12}", "Stock ID", "Price ($)", "Available");
    println!("{}", "-".repeat(36));
    
    let mut stock_list: Vec<&Stock> = stocks.values().collect();
    stock_list.sort_by(|a, b| a.id.cmp(&b.id));
    
    for stock in stock_list {
        println!(
            "{:<10} {:>12.2} {:>12}",
            stock.id,
            stock.price,
            stock.available_quantity
        );
    }
    println!("{}", "-".repeat(36));
}