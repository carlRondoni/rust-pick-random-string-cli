use std::env;

use rand::seq::IndexedRandom;
use rand::rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <list>", args[0]);
        return;
    }

    let list: Vec<&str> = args[1].split(",").collect();

    let mut rng = rng();
    let random = list.choose(&mut rng).unwrap();

    println!("{}", random);
}
