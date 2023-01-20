use clap::Parser;

mod lamination;
mod cover;
use cover::MarkedMultCover;

use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Period of the marked cycle
   #[arg(short, long)]
   marked_period: u32,

   /// Period of the critical cycle (must be 1 or 2 for now)
   #[arg(short, long, default_value_t = 1)]
   crit_period: u32,
}

fn main() {
    let args = Args::parse();
    // let args: Vec<String> = env::args().collect();

    let period = args.marked_period;

    let use_per2 = match args.crit_period {
        2 => true,
        _ => false,
    };

    println!(
        "Computing combinatorics of (c,lambda) -> c cover for marked period {}, critical period {}",
        period, (args.crit_period == 2) as i32 + 1
    );

    let mut cov = MarkedMultCover::new(period, 2, use_per2);
    cov.run();

    cov.summarize(4);
}
