use std::collections::HashMap;
use serde::{Deserialize, Serialize};
// use lapin::{Channel, BasicProperties, options::BasicPublishOptions};
// use tokio::sync::Mutex;
// use std::sync::Arc;
use rand::Rng;

pub fn apply_price_fluctuations(stocks: &mut HashMap<String, Stock>) {
    let mut rng = rand::thread_rng();
    for stock in stocks.values_mut() {
        // Random price change between -5% and +5%
        let change_percent = rng.gen_range(-5.0..=5.0);
        let price_change = stock.price * (change_percent / 100.0);
        stock.price = (stock.price + price_change).max(1.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub id: String,
    pub price: f64,
    pub available_quantity: usize,
}

pub fn initialize_stocks() -> HashMap<String, Stock> {
    let stock_data = vec![
        ("AAPL", 150.0), ("GOOG", 2800.0), ("AMZN", 3400.0), ("MSFT", 310.0), ("TSLA", 700.0),
        ("META", 320.0), ("NFLX", 600.0), ("NVDA", 800.0), ("ORCL", 85.0), ("CSCO", 54.0),
        ("ADBE", 560.0), ("IBM", 120.0), ("INTC", 29.0), ("AMD", 85.0), ("PYPL", 68.0),
        ("CRM", 210.0), ("UBER", 46.0), ("LYFT", 9.0), ("TWTR", 60.0), ("DIS", 92.0),
        ("SONY", 90.0), ("BABA", 84.0), ("V", 220.0), ("MA", 370.0), ("JPM", 145.0),
        ("BAC", 33.0), ("C", 48.0), ("WFC", 42.0), ("T", 18.0), ("VZ", 34.0),
        ("TMUS", 140.0), ("SBUX", 95.0), ("KO", 59.0), ("PEP", 180.0), ("MCD", 290.0),
        ("NKE", 92.0), ("PG", 155.0), ("XOM", 108.0), ("CVX", 160.0), ("BP", 36.0),
        ("F", 12.0), ("GM", 32.0), ("GE", 100.0), ("BA", 190.0), ("CAT", 260.0),
        ("DE", 380.0), ("TSM", 90.0), ("INTU", 490.0), ("SQ", 55.0), ("SHOP", 50.0),
        ("ZM", 69.0), ("ROKU", 45.0), ("DOCU", 45.0), ("ETSY", 75.0), ("SNOW", 150.0),
    ];

    stock_data
        .into_iter()
        .map(|(id, price)| {
            (
                id.to_string(),
                Stock {
                    id: id.to_string(),
                    price,
                    available_quantity: 100,
                },
            )
        })
        .collect()
}

// pub async fn run_stock_side(stocks: Arc<Mutex<HashMap<String, Stock>>>, channel: Channel) {
//     let mut rng = rand::thread_rng();

//     loop {
//         {
//             let mut stocks_guard = stocks.lock().await;
//             for stock in stocks_guard.values_mut() {
//                 let fluctuation = rng.gen_range(-0.05..0.05); // ±5% fluctuation
//                 stock.price = (stock.price * (1.0 + fluctuation)).max(1.0); // Ensure non-negative price
//             }

//             let updates: Vec<_> = stocks_guard.values().cloned().collect();
//             let payload = serde_json::to_string(&updates).unwrap();

//             channel
//                 .basic_publish(
//                     "",
//                     "stock_updates",
//                     BasicPublishOptions::default(),
//                     payload.as_bytes(),
//                     BasicProperties::default(),
//                 )
//                 .await
//                 .unwrap();

//             println!("[Stock Side] Sent stock updates: {:?}", updates);
//         }
//         tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
//     }
// }
