use std::env;

use game_of_life::structs::row::Row;
use game_of_life::structs::ruleset_parser::RulesetParser;
use game_of_life::util::user_input;

fn main() {
    let args: Vec<_> = env::args().collect();

    println!("Welcome to Conway's Game of Life!\n");

    let ruleset = RulesetParser::new(user_input::get_ruleset(&args));

    let mut input = Row::from(user_input::get_start(&args));

    let iterations: usize = env::var("ITERATIONS")
        .unwrap_or("20".to_string())
        .parse()
        .expect("Invalid number!");

    if args.len() > 2 {
        // If user provided start without entering manually, print it
        println!("{input}");
    }

    for _ in 0..iterations {
        input.compute_row(&ruleset);
        println!("{input}");
    }
}
