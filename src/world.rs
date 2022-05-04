use crate::{
    industry::*,
    market::MarketNetwork,
    population::{Population, Pop},
    good::Good,
};

pub struct World {
    market: MarketNetwork,

    population: Population,
    industry: Industry,

    day: i32,
}

impl World {
    pub fn new() -> Self {
        let industry = IndustryBuilder::new()
            .with(10, Factory::new(Good::Food, 10_000))
            .with(10, Factory::new(Good::Clothes, 10_000))
            .build();

        let pop = Pop {
            money: 10_000,
            health: 10,
            labour: 0,
        };

        Self {
            market: MarketNetwork::default(),
            population: Population::new(10, pop),
            industry,
            day: 0,
        }
    }

    pub fn tick(&mut self) {
        self.market.open();

        self.population.tick(&mut self.industry, 
                             &mut self.market);

        self.industry.tick(&mut self.population, 
                           &mut self.market);

        self.market.close();
        self.day += 1;
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

