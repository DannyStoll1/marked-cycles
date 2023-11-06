#![allow(dead_code)]

use clap::Parser;

use marked_cycles::combinatorics::{dynatomic, marked_cycle, Combinatorics};
use marked_cycles::dynatomic_cover::DynatomicCover;
use marked_cycles::marked_cycle_cover::MarkedCycleCover;
use marked_cycles::tikz::TikzRenderer;
use marked_cycles::types::Period;

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

    /// Generate tikz
    #[arg(long, default_value_t = false)]
    tikz: bool,
}

fn print_combinatorics(args: &Args)
{
    if args.marked_period > 0 {
        println!(
            "Computing combinatorics of (c,lambda) -> c cover for marked period {}, critical period {}",
            args.marked_period, args.crit_period
        );

        if args.dynatomic {
            let cov = DynatomicCover::new(args.marked_period, args.crit_period);
            cov.summarize(args.indent, args.binary);
        } else {
            let cov = MarkedCycleCover::new(args.marked_period, args.crit_period);
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
    let p2: Box<dyn Combinatorics> = if args.dynatomic {
        Box::new(dynatomic::Comb::new(args.crit_period))
    } else {
        Box::new(marked_cycle::Comb::new(args.crit_period))
    };

    if args.table_max_period > 0 {
        print_row!("period", "vertices", "edges", "faces", "genus");
        for period in 2..=args.table_max_period {
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

fn draw_largest_face(args: &Args)
{
    if args.tikz {
        let tikz = if args.dynatomic {
            let cov = DynatomicCover::new(args.marked_period, args.crit_period);
            TikzRenderer::new(cov.primitive_faces).draw_largest_face()
        } else {
            let cov = MarkedCycleCover::new(args.marked_period, args.crit_period);
            TikzRenderer::new(cov.faces).draw_largest_face()
        };
        println!("{tikz}");
    }
}

fn main()
{
    let args = Args::parse();

    print_combinatorics(&args);
    print_data_table(&args);
    draw_largest_face(&args);
}
