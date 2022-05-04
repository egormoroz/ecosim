use crate::{
    population::Population, 
    market::MarketNetwork, 
    industry::Industry,
    good::Good,
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Purchase {
    pub cost: i32,
    pub amount: i32,
}

pub fn buy_labour(
    population: &mut Population, 
    market: &mut MarketNetwork,
    target_amount: i32) -> Purchase
{
    market.inc_demand(Good::Labour, target_amount);
    population.buy_labour(
        target_amount, market.price(Good::Labour))
}

pub fn buy_commodities(
    industry: &mut Industry,
    market: &mut MarketNetwork,
    good: Good,
    target_amount: i32) -> Purchase
{
    debug_assert!(good != Good::Labour);
    market.inc_demand(good, target_amount);
    industry.buy(target_amount, good, market.price(good))
}

pub fn publish_for_sale(
    market: &mut MarketNetwork,
    good: Good,
    amount: i32)
{
    market.inc_supply(good, amount);
}

