use rand::{Rng, seq::SliceRandom};
use crate::broker::Broker;
use crate::simulation::Stock;

#[derive(Debug, Clone)]
pub enum Strategy {
    Aggressive,
    RiskAverse,
    Random,
}

#[derive(Debug, Clone)]
pub enum Action {
    Buy,
    Sell,
    Hold,
}

impl Strategy {
    /// Decide the next action for a broker based on their strategy.
    pub fn decide_action(&self, broker: &Broker, stocks: &[Stock]) -> Action {
        match self {
            Strategy::Aggressive => Self::decide_aggressive(broker, stocks),
            Strategy::RiskAverse => Self::decide_risk_averse(broker, stocks),
            Strategy::Random => Self::decide_random(),
        }
    }

    /// Decide action for the aggressive strategy.
    fn decide_aggressive(broker: &Broker, _stocks: &[Stock]) -> Action {
        if broker.cash > 0.0 {
            Action::Buy
        } else {
            Action::Hold
        }
    }

    /// Decide action for the risk-averse strategy.
    fn decide_risk_averse(broker: &Broker, _stocks: &[Stock]) -> Action {
        if broker.holdings.iter().any(|(_, &qty)| qty > 0) {
            Action::Sell
        } else if broker.cash > 0.0 {
            Action::Buy
        } else {
            Action::Hold
        }
    }

    /// Decide action for the random strategy.
    fn decide_random() -> Action {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            0 => Action::Buy,
            1 => Action::Sell,
            _ => Action::Hold,
        }
    }

    /// Execute the trading strategy for a broker.
    pub fn execute(&self, broker: &mut Broker, stocks: &mut [Stock]) {
        match self {
            Strategy::Aggressive => self.execute_aggressive(broker, stocks),
            Strategy::RiskAverse => self.execute_risk_averse(broker, stocks),
            Strategy::Random => self.execute_random(broker, stocks),
        }
    }

    /// Aggressive Strategy: Prioritize buying high-value stocks with available cash.
    fn execute_aggressive(&self, broker: &mut Broker, stocks: &mut [Stock]) {
        let mut rng = rand::thread_rng();

        // Sort stocks by price (descending) for aggressive buying.
        stocks.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

        for stock in stocks.iter_mut() {
            let quantity = rng.gen_range(1..=5); // Randomize quantity to buy (1-5).
            if broker.buy(stock, quantity).is_ok() {
                log::info!(
                    "Broker {} (Aggressive): Bought {} shares of {}.",
                    broker.id, quantity, stock.id
                );
                break;
            }
        }
    }

    /// Risk-Averse Strategy: Sell high-value stocks or buy small quantities of low-value stocks.
    fn execute_risk_averse(&self, broker: &mut Broker, stocks: &mut [Stock]) {
        let mut rng = rand::thread_rng();

        // Sort stocks by price (ascending) for cautious buying.
        stocks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        for stock in stocks.iter_mut() {
            // Prioritize selling high-value stocks.
            if broker.sell(stock, rng.gen_range(1..=3)).is_ok() {
                log::info!(
                    "Broker {} (Risk-Averse): Sold shares of {}.",
                    broker.id, stock.id
                );
                return;
            }

            // Buy small quantities if cash is available.
            let quantity = rng.gen_range(1..=2);
            if broker.buy(stock, quantity).is_ok() {
                log::info!(
                    "Broker {} (Risk-Averse): Bought {} shares of {}.",
                    broker.id, quantity, stock.id
                );
                return;
            }
        }
    }

    /// Random Strategy: Perform random buy or sell actions.
    fn execute_random(&self, broker: &mut Broker, stocks: &mut [Stock]) {
        let mut rng = rand::thread_rng();

        // Randomly pick a stock.
        if let Some(stock) = stocks.choose_mut(&mut rng) {
            if rng.gen_bool(0.5) {
                // Attempt to buy randomly.
                let quantity = rng.gen_range(1..=3);
                if broker.buy(stock, quantity).is_ok() {
                    log::info!(
                        "Broker {} (Random): Bought {} shares of {}.",
                        broker.id, quantity, stock.id
                    );
                }
            } else {
                // Attempt to sell randomly.
                let quantity = rng.gen_range(1..=3);
                if broker.sell(stock, quantity).is_ok() {
                    log::info!(
                        "Broker {} (Random): Sold {} shares of {}.",
                        broker.id, quantity, stock.id
                    );
                }
            }
        }
    }
}
