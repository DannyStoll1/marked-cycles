use clap::Parser;
use marked_cycles::{common::cells::Face, prelude::*};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Period of the critical cycle (must be 1 or 2)
    #[arg(short, long, default_value_t = 1)]
    crit_period: Period,

    /// Max period of data table
    #[arg(short, long, default_value_t = 15)]
    max_period: Period,

    /// Write file with header that can be parsed by serde
    #[arg(short, long, default_value_t = false)]
    serde_header: bool,
}

struct TableRow {
    pub period: Period,
    pub max_face: usize,
    pub min_face: usize,
    pub min_face_irr: usize,
    pub num_max: usize,
    pub num_min: usize,
    pub num_min_irr: usize,
    pub num_reflexive: usize,
    pub num_odd_irr: usize,
}
impl std::fmt::Display for TableRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{}",
            self.period as usize,
            self.max_face,
            self.min_face,
            self.min_face_irr,
            self.num_max,
            self.num_min,
            self.num_min_irr,
            self.num_reflexive,
            self.num_odd_irr
        )
    }
}

struct Table(Vec<TableRow>);
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            writeln!(
                f,
                "period,\
                max_face,\
                min_face,\
                min_face_irr,\
                num_max,\
                num_min,\
                num_min_irr,\
                num_reflexive,\
                num_odd_irr\
                "
            )?;
        } else {
            writeln!(
                f,
                "Period, \
                Max face, \
                Min face, \
                Min irr. face, \
                # max faces, \
                # min faces, \
                # min irr. faces, \
                # refl. faces, \
                # odd irr. faces"
            )?;
        }

        self.0
            .iter()
            .try_for_each(|row| writeln!(f, "{}", format_args!("{row}")))
    }
}

fn compute_counts(period: Period, crit_per: Period) -> TableRow {
    let mc = MarkedCycleCover::new(period, crit_per);
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

    TableRow {
        period,
        max_face,
        min_face,
        min_face_irr,
        num_max,
        num_min,
        num_min_irr,
        num_reflexive,
        num_odd_irr,
    }
}

impl FromIterator<TableRow> for Table {
    fn from_iter<I: IntoIterator<Item = TableRow>>(iter: I) -> Self {
        let rows: Vec<TableRow> = iter.into_iter().collect();
        Self(rows)
    }
}

fn main() {
    let args = Args::parse();

    let max_period = args.max_period;
    let crit_per = args.crit_period;

    let table: Table = (1..=max_period)
        .map(|n| compute_counts(n, crit_per))
        .collect();

    if args.serde_header {
        print!("{table:#}");
    } else {
        print!("{table}");
    }
}
