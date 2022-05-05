use rand::prelude::*;
use std::fmt;

use crate::{
    good::Good, 
    misc::*, 
    industry::Industry,
    bns::*, 
    market::MarketNetwork,
};

#[derive(Debug, Clone, Copy)]
pub struct Pop {
    pub money: i32,
    pub labour: i32,
    pub health: i32,
}

pub struct Population {
    pops: Vec<Pop>,
}

impl Population {
    pub fn new(n: usize, pop: Pop) -> Self {
        Self { pops: vec![pop; n] }
    }

    pub fn kill(&mut self, idx: usize) -> Inheritance {
        Inheritance(self.pops.swap_remove(idx).money)
    }

    pub fn reproduce(&mut self, idx: usize) {
        let birth_money = self.pops[idx].money;

        self.pops[idx].money -= birth_money;
        self.pops[idx].health /= 2;

        self.pops.push(Pop { 
            money: birth_money, 
            health: 3,
            labour: 0, 
        });
    }

    pub fn buy_labour(&mut self, target_amount: i32,
                      market_price: i32) -> Purchase
    {
        let offset = thread_rng()
            .gen_range(0..self.pops.len());

        let mut bought_amount = 0;
        for idx in (offset..self.pops.len())
            .chain(0..offset)
        {
            let mut pop = &mut self.pops[idx];

            let amount = pop.labour.min(
                target_amount - bought_amount);

            pop.money += amount * market_price;
            pop.labour -= amount;

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

    pub fn tick(&mut self, industry: &mut Industry,
                market: &mut MarketNetwork) 
    {
        self.pops.shuffle(&mut thread_rng());
        for pop in self.pops.iter_mut() {
            pop.tick(industry, market);
        }

        let mut idx = 0;
        let mut n = self.pops.len();

        while idx < n {
            let health = self.pops[idx].health;

            if health <= 0 {
                let inheritance = self.kill(idx);
                self.distribute_inheritance(inheritance);

                n -= 1;
                continue;
            }

            if health >= 10 {
                self.reproduce(idx);
            }

            idx += 1;
        }
    }

    pub fn distribute_inheritance(&mut self, inheritance: Inheritance) {
        let heir = self.pops.choose_mut(&mut thread_rng());
        if let Some(heir) = heir {
            heir.money += inheritance.0;
        }
    }

    pub fn count(&self) -> usize {
        self.pops.len()
    }

    pub fn money_supply(&self) -> i32 {
        self.pops.iter()
            .map(|p| p.money)
            .sum()
    }
}

impl Pop {
    pub fn tick(&mut self, industry: &mut Industry,
                market: &mut MarketNetwork) 
    {
        let budget_food = self.money * 40 / 100;
        let budget_clothes = self.money * 20 / 100;

        let food_to_buy = budget_food / market.price(Good::Food);
        let clothes_to_buy = budget_clothes / market.price(Good::Clothes);

        const LABOUR_OUTPUT: i32 = 10;
        const MIN_FOOD: i32 = 2;
        const MIN_CLOTHES: i32 = 1;

        let food_purchase = buy_commodities(
            industry, market, Good::Food, food_to_buy);
        let clothes_purchase = buy_commodities(
            industry, market, Good::Clothes, clothes_to_buy);

        self.money -= food_purchase.cost + clothes_purchase.cost;

        self.labour = LABOUR_OUTPUT;
        publish_for_sale(market, Good::Labour, self.labour);

        if food_purchase.amount >= MIN_FOOD 
            && clothes_purchase.amount >= MIN_CLOTHES 
        {
            self.health += 1;
        } else {
            self.health -= 1;
        }
    }
}

impl fmt::Display for Population {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} people", self.pops.len())?;
        writeln!(f, "{: <10} | {: <10 } | {: <10}",
                 "Money", "Health", "Labour")?;

        for pop in self.pops.iter() {
            writeln!(f, "{: <10} | {: <10 } | {: <10}", 
                     pop.money, pop.health, pop.labour)?;
        }

        Ok(())
    }
}

