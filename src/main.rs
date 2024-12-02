use clap::Parser;

mod args;
mod puzzles;

fn main() {
    let args = args::Args::parse();

    match args.day {
        1 => puzzles::day_one::run_day_one(),
        2 => puzzles::day_two::run_day_two(),
        _ => println!("This day hasn't been solved yet!"),
    }
}
