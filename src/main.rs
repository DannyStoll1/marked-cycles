#![allow(dead_code)]

use clap::Parser;

pub mod abstract_cycles;
pub mod combinatorics;
pub mod dynatomic_cover;
pub mod lamination;
pub mod marked_cycle_cover;
pub mod types;

use combinatorics::MarkedCycleCombinatorics;
use marked_cycle_cover::MarkedCycleCover;
use types::Period;

use crate::dynatomic_cover::DynatomicCover;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
{
    /// Period of the marked cycle (0 to skip)
    #[arg(short, long, default_value_t = 0)]
    marked_period: Period,

    /// Period of the critical cycle (must be 1 or 2 for now)
    #[arg(short, long, default_value_t = 1)]
    crit_period: Period,

    /// Max period of data table (0 to skip)
    #[arg(short, long, default_value_t = 0)]
    table_max_period: Period,

    /// Compute dynatomic curve instead of marked cycle curve
    #[arg(short, long, default_value_t = false)]
    dynatomic: bool,

    /// Display cell ids in binary
    #[arg(short, long, default_value_t = false)]
    binary: bool,

    /// How far to indent the cell descriptions
    #[arg(long, default_value_t = 4)]
    indent: usize,
}

fn print_combinatorics(args: &Args)
{
    if args.marked_period > 0
    {
        println!(
            "Computing combinatorics of (c,lambda) -> c cover for marked period {}, critical period {}",
            args.marked_period, args.crit_period
        );

        if args.dynatomic
        {
            let cov = DynatomicCover::new(args.marked_period, 2, args.crit_period);
            cov.summarize(args.indent, args.binary);
        }
        else
        {
            let cov = MarkedCycleCover::new(args.marked_period, 2, args.crit_period);
            cov.summarize(args.indent, args.binary);
        }
    }
}

macro_rules! print_row {
    ($a: expr, $b: expr, $c: expr, $d: expr, $e: expr) => {
        println!("{:>8} | {:>12} {:>12} {:>12} {:>12}", $a, $b, $c, $d, $e)
    };
}

fn print_data_table(args: &Args)
{
    let p2 = MarkedCycleCombinatorics::new(args.crit_period);

    if args.table_max_period > 0
    {
        if args.dynatomic
        {
            println!("\nData table not yet supported for dynatomic curves.");
            return;
        }
        print_row!("period", "vertices", "edges", "faces", "genus");
        for period in 2..=args.table_max_period
        {
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

fn main()
{
    let args = Args::parse();

    print_combinatorics(&args);
    print_data_table(&args);
}
