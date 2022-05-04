use std::cmp::Ordering;

use crate::good::*;

#[derive(Default)]
pub struct MarketNetwork {
    markets: [Market; NUM_OF_GOODS],
}

impl MarketNetwork {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn open(&mut self) {
        for market in self.markets.iter_mut() {
            market.open();
        }
    }

    pub fn close(&mut self) {
        for market in self.markets.iter_mut() {
            market.close();
        }
    }

    pub fn price(&self, good: Good) -> i32 {
        self.markets[good as usize].price
    }

    pub fn inc_demand(&mut self, good: Good, n: i32) {
        self.markets[good as usize].demand += n;
    }

    pub fn inc_supply(&mut self, good: Good, n: i32) {
        self.markets[good as usize].supply += n;
    }
}

#[derive(Default, Clone, Copy)]
pub struct Market {
    pub price: i32,
    pub price_f32: f32,
    pub price_velocity: f32,
    pub demand: i32,
    pub supply: i32,
}

impl Market {
    pub fn open(&mut self) {
        self.price = self.price.max(1);
        self.price_f32 = self.price_f32.max(1.0);

        self.demand = 0;
        self.supply = 0;
    }

    pub fn close(&mut self) {
        match self.demand.cmp(&self.supply) {
            Ordering::Greater => {
                match self.price_velocity > 0.0 {
                    true => self.price_velocity *= 1.5,
                    false => self.price_velocity *= -0.5,
                };
                self.price_velocity = self.price_velocity.max(0.01);
            },
            Ordering::Less => {
                match self.price_velocity < 0.0 {
                    true => self.price_velocity *= 1.5,
                    false => self.price_velocity *= -0.5,
                };
                self.price_velocity = self.price_velocity.min(-0.01);
            },
            Ordering::Equal => self.price_velocity = 0.0,
        };

        self.price_f32 += self.price_velocity;
        self.price = self.price_f32.round() as i32;
    }
}

