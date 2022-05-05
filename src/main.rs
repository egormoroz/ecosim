extern crate rand;

pub mod good;
pub mod population;
pub mod industry;
pub mod market;
pub mod world;
pub mod misc;
pub mod bns;

use std::io::{stdin, BufRead};

use world::World;

fn main() {
    let mut world = World::new();

    // world.print_summary();

    let mut line = String::with_capacity(256);
    while line.trim() != "q" {
        world.tick();
        world.print_summary();

        line.clear();
        stdin().lock().read_line(&mut line).unwrap();
    }
}

