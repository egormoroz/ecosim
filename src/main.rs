extern crate rand;

pub mod good;
pub mod population;
pub mod industry;
pub mod market;
pub mod world;
pub mod misc;
pub mod bns;

use world::World;

fn main() {
    let mut world = World::new();
    world.tick();
}

