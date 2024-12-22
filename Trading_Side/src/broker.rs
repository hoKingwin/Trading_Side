use std::collections::HashMap;
use std::fmt;
use crate::trading_strategy::Strategy;
use crate::simulation::Stock;

pub struct Broker {
    pub id: u32,
    pub cash: f64,
    pub strategy: Strategy,
    pub holdings: HashMap<String, usize>,
}

impl Broker {
    pub fn new(id: u32, initial_cash: f64, strategy: Strategy) -> Self {
        Broker {
            id,
            cash: initial_cash,
            strategy,
            holdings: HashMap::new(),
        }
    }

    pub fn buy(&mut self, stock: &Stock, quantity: usize) -> Result<(), &'static str> {
        let cost = stock.price * quantity as f64;
        if self.cash >= cost && stock.available_quantity >= quantity {
            self.cash -= cost;
            *self.holdings.entry(stock.id.clone()).or_insert(0) += quantity;
            Ok(())
        } else {
            Err("Insufficient funds or stock quantity")
        }
    }

    pub fn sell(&mut self, stock: &Stock, quantity: usize) -> Result<(), &'static str> {
        if let Some(&current_quantity) = self.holdings.get(&stock.id) {
            if current_quantity >= quantity {
                let revenue = stock.price * quantity as f64;
                self.cash += revenue;
                
                let new_quantity = current_quantity - quantity;
                if new_quantity == 0 {
                    self.holdings.remove(&stock.id);
                } else {
                    self.holdings.insert(stock.id.clone(), new_quantity);
                }
                Ok(())
            } else {
                Err("Insufficient stock quantity in holdings")
            }
        } else {
            Err("Stock not found in holdings")
        }
    }

    pub fn get_total_value(&self, stocks: &[Stock]) -> f64 {
        let holdings_value: f64 = stocks.iter()
            .filter_map(|stock| {
                self.holdings.get(&stock.id)
                    .map(|&quantity| stock.price * quantity as f64)
            })
            .sum();
        self.cash + holdings_value
    }

    pub fn get_holdings(&self) -> &HashMap<String, usize> {
        &self.holdings
    }

    pub fn get_cash(&self) -> f64 {
        self.cash
    }
}

impl fmt::Display for Broker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Broker {}: Cash: ${:.2}, Holdings: {:?}", 
            self.id, self.cash, self.holdings)
    }
}

impl fmt::Debug for Broker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Broker {{ id: {}, cash: ${:.2}, strategy: {:?}, holdings: {:?} }}", 
            self.id, self.cash, self.strategy, self.holdings)
    }
}