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
            market: MarketNetwork::new(),
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

    pub fn print_summary(&self) {
        let cloth_factories = self.industry
            .count_factories(Good::Clothes);
        let food_factories = self.industry
            .count_factories(Good::Food);
        let pops = self.population.count();

        let money_supply = self.industry.money_supply()
            + self.population.money_supply();

        println!("Day {}", self.day);
        println!("{}", self.market);
        // println!("{}", self.population);
        // println!("{}", self.industry);
        println!("Population: {}", pops);
        println!("Food factories: {}", food_factories);
        println!("Cloth factories: {}", cloth_factories);
        println!("Money supply: {}", money_supply);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

