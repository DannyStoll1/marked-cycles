use clap::Parser;
use marked_cycles::{prelude::*, common::cells::Face};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
{
    /// Period of the critical cycle (must be 1 or 2)
    #[arg(short, long, default_value_t = 1)]
    crit_period: Period,

    /// Max period of data table
    #[arg(short, long, default_value_t = 15)]
    max_period: Period,
}

fn main()
{
    let args = Args::parse();

    let max_period = args.max_period;
    let crit_per = args.crit_period;

    // for crit_per in [1, 2] {
    // println!("\nPer({crit_per}) MC(n)");
    println!(
        "Period, \
                Max face, \
                Min face, \
                Min irr. face, \
                # max faces, \
                # min faces, \
                # min irr. faces, \
                # refl. faces, \
                # odd irr. faces"
    );
    for n in 1..=max_period {
        let mc = MarkedCycleCover::new(n, crit_per);
        let max_face = mc.face_sizes().max().unwrap_or_default();
        let min_face = mc.face_sizes().min().unwrap_or_default();

        let num_max = mc.faces.iter().filter(|f| f.len() == max_face).count();
        let num_min = mc.faces.iter().filter(|f| f.len() == min_face).count();

        let min_face_irr = mc
            .faces
            .iter()
            .filter(|f| !f.is_reflexive())
            .map(Face::len)
            .min()
            .unwrap_or_default();
        let num_min_irr = mc
            .faces
            .iter()
            .filter(|f| !f.is_reflexive() && f.len() == min_face_irr)
            .count();
        let num_odd_irr = mc
            .faces
            .iter()
            .filter(|f| !f.is_reflexive() && f.len() % 2 == 1)
            .count();

        let num_reflexive = mc.faces.iter().filter(|f| f.is_reflexive()).count();

        let row = [
            n as usize,
            max_face,
            min_face,
            min_face_irr,
            num_max,
            num_min,
            num_min_irr,
            num_reflexive,
            num_odd_irr,
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");
        println!("{row}");
    }
}
