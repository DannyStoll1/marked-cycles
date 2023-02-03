use clap::Parser;

pub mod combinatorics;
pub mod cover;
pub mod lamination;
pub mod types;

use combinatorics::MarkedCycleCombinatorics;
use cover::MarkedMultCover;
use types::Period;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Period of the marked cycle (0 to skip)
    #[arg(short, long, default_value_t = 0)]
    marked_period: u32,

    /// Period of the critical cycle (must be 1 or 2 for now)
    #[arg(short, long, default_value_t = 1)]
    crit_period: u32,

    /// Max period of data table (0 to skip)
    #[arg(short, long, default_value_t = 0)]
    table_max_period: u32,
}

fn print_combinatorics(period: Period, crit_period: Period) {
    if period > 0 {
        println!(
            "Computing combinatorics of (c,lambda) -> c cover for marked period {}, critical period {}",
            period, crit_period
        );

        let mut cov = MarkedMultCover::new(period, 2, crit_period);
        cov.run();

        cov.summarize(4);
    }
}

macro_rules! print_row {
    ($a: expr, $b: expr, $c: expr, $d: expr, $e: expr) => {
        println!(
            "{:>8} | {:>12} {:>12} {:>12} {:>12}",
            $a, $b, $c, $d, $e
        )
    };
}

fn print_data_table(max_period: Period, crit_period: Period) {

    let mut p2 = MarkedCycleCombinatorics::new(crit_period);

    if max_period > 0 {

        print_row!("period", "vertices", "edges", "faces", "genus");
        for period in 2..=max_period {
            print_row!(
                period,
                p2.vertices(period),
                p2.edges(period),
                p2.faces(period),
                p2.genus(period)
            );
        }
    }
}

fn main() {
    let args = Args::parse();

    let period = args.marked_period.into();
    let crit_period = args.crit_period.into();
    let table_max_period = args.table_max_period.into();

    print_combinatorics(period, crit_period);
    print_data_table(table_max_period, crit_period.into());

}
