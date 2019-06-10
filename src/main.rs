extern crate stopwatch;
use stopwatch::{Stopwatch};

mod day_one;

fn main() {
    let sw = Stopwatch::start_new();
    print_elapsed_time_str(String::from("day_one"), day_one(sw));
}

fn day_one(sw: stopwatch::Stopwatch) -> i64 {
    let numbers = [1, 15, 32, 91, 16, 12, 2];
    let find = 48;
    println!("we have {:?} and are looking for {}", numbers, find);
    let return_vals = day_one::calc(&numbers, find);
    let elapsed_ms = sw.elapsed_ms();
    println!("the matches are {} and {}", return_vals.0, return_vals.1);

    return elapsed_ms;
}

fn print_elapsed_time_str(program: String, ms: i64) {
    println!("{} took {}ms", program, ms);
}