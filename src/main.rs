extern crate rand;

use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::{Duration, Instant};

use rand::Rng;

const MAX_RANDOM: i32 = 10000;
const MAX_ROUTES: i32 = 100000;

#[derive(Clone, Hash, Eq, Debug, PartialEq)]
struct Route {
    source: String,
    destination: String,
}

impl Route {
    fn opposite(&self) -> Route {
        Route {
            source: self.clone().destination,
            destination: self.clone().source,
        }
    }
}

fn generate_routes() -> Vec<Route> {
    let mut routes: Vec<Route> = Vec::new();

    for _ in 0..MAX_ROUTES {
        let source: i32 = rand::thread_rng().gen_range(0, MAX_RANDOM);
        let destination: i32 = rand::thread_rng().gen_range(0, MAX_RANDOM);

        routes.push(Route {
            source: source.to_string(),
            destination: destination.to_string(),
        })
    }

    routes
}

fn calculate_routes(routes: Vec<Route>) -> usize {
    let unique_routes: HashSet<Route> = HashSet::from_iter(routes.iter().cloned());
    let mut visited_routes: HashSet<Route> = HashSet::new();

    for route in unique_routes.iter().cloned() {
        if unique_routes.contains(&route.opposite()) && !visited_routes.contains(&route.opposite())
        {
            visited_routes.insert(route);
        }
    }

    visited_routes.len()
}

fn main() {
    let routes: Vec<Route> = generate_routes();

    let start: Instant = Instant::now();
    let result: usize = calculate_routes(routes);
    let duration: Duration = start.elapsed();

    println!("Routes count: {:?}", MAX_ROUTES);
    println!("Same routes: {:?}", result);
    println!("Computation time: {:?} ms", duration.as_millis());
}
