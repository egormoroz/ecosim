use std::iter::repeat;

use rand::prelude::*;

use crate::{
    good::*,
    market::MarketNetwork,
    population::Population,
    misc::*,
    bns::*,
};

pub const FACTORY_COST: i32 = 5000;

#[derive(Debug, Clone, Copy)]
pub struct Factory {
    pub good: Good,
    pub money: i32,
    pub inventory: i32,
    pub days_closed: i32,
}

#[derive(Default)]
pub struct IndustryBuilder {
    initial_data: [(usize, Option<Factory>); NUM_OF_GOODS],
}

impl IndustryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with(mut self, num_factories: usize, 
                initial_state: Factory) -> Self 
    {
        self.initial_data[initial_state.good as usize] = 
            (num_factories, Some(initial_state));
        self
    }

    pub fn build(self) -> Industry {
        let total = self.initial_data.iter().map(|x| x.0).sum();
        let mut factories = Vec::with_capacity(total);

        for (n, f) in self.initial_data {
            if let Some(f) = f {
                debug_assert!(n != 0);
                factories.extend(repeat(f).take(n));
            }
        }

        Industry { factories }
    }
}

pub struct Industry {
    factories: Vec<Factory>,
}

impl Industry {
    pub fn shuffle(&mut self) {
        self.factories.shuffle(&mut thread_rng());
    }

    pub fn liquidate(&mut self, idx: usize) -> Inheritance {
        Inheritance(self.factories.swap_remove(idx).money)
    }

    pub fn build(&mut self, parent_idx: usize) {
        let good = {
            let parent = &mut self.factories[parent_idx];
            debug_assert!(parent.money >= FACTORY_COST);

            parent.money -= FACTORY_COST;
            parent.good
        };

        self.factories.push(Factory {
            good,
            money: FACTORY_COST,
            inventory: 0,
            days_closed: 0,
        });
    }

    pub fn buy(&mut self, target_amount: i32, good: Good,
               market_price: i32) -> Purchase 
    {
        let offset = thread_rng()
            .gen_range(0..self.factories.len());

        let mut bought_amount = 0;
        for idx in (offset..self.factories.len())
            .chain(0..offset)
        {
            let mut factory = &mut self.factories[idx];
            if factory.good != good { continue; }

            let amount = factory.inventory.min(
                target_amount - bought_amount);

            factory.money += amount * market_price;
            factory.inventory -= amount;

            bought_amount += amount;
            if bought_amount == target_amount {
                break;
            }
        }


        Purchase { 
            amount: bought_amount,
            cost: bought_amount * market_price
        }
    }

    pub fn tick(&mut self, population: &mut Population,
                market: &mut MarketNetwork) 
    {
        self.factories.shuffle(&mut thread_rng());
        for factory in self.factories.iter_mut() {
            factory.tick(population, market);
        }

        let mut idx = 0;
        let mut n = self.factories.len();

        while idx < n {
            let Factory { days_closed, money, .. } = self.factories[idx];

            if days_closed >= 5 {
                let inheritance = self.liquidate(idx);
                population.distribute_inheritance(inheritance);

                n -= 1;
                continue;
            }

            if money >= FACTORY_COST * 2 {
                self.build(idx);
            }

            idx += 1;
        }
    }
}

impl Factory {
    pub fn new(good: Good, money: i32) -> Self {
        Self {
            good, money,
            inventory: 0,
            days_closed: 0,
        }
    }

    pub fn tick(&mut self, population: &mut Population, 
                market: &mut MarketNetwork) 
    {
        let budget_labour = self.money * 80 / 100;
        let target_amount = budget_labour / market.price(Good::Labour);

        const MIN_LABOUR: i32 = 1;

        let Purchase { amount: labour, cost } = buy_labour(
            population, market, target_amount);
        self.money -= cost;

        if labour < MIN_LABOUR { 
            self.days_closed += 1;
            return;
        }

        self.days_closed = 0;

        match self.good {
            Good::Food => {
                self.inventory = self.inventory * 90 / 100;

                let production = 2.0 * f32::ln_1p(labour as f32);
                self.inventory += production as i32;
            },
            Good::Clothes => {
                self.inventory = self.inventory * 98 / 100;

                let production = 0.7 * f32::ln_1p(labour as f32);
                self.inventory += production as i32;
            },
            Good::Labour => panic!("factories cannot produce labour"),
        };

        publish_for_sale(market, self.good, self.inventory);
    }
}

