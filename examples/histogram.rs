use clap::Parser;
use marked_cycles::prelude::*;
use plotters::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
{
    /// Period of the critical cycle (must be 1 or 2)
    #[arg(short, long, default_value_t = 1)]
    crit_period: Period,

    /// Max period of data table
    #[arg(short, long, default_value_t = 10)]
    period: Period,
}

fn make_histogram(period: Period, crit_per: Period)
{
    let mc = MarkedCycleCover::new(period, crit_per);

    let data = mc.face_sizes().map(|x| x as i32);

    let path = std::path::PathBuf::new()
        .join("plots")
        .join(format!("face_sizes_per{crit_per}_mc{period}.svg"));

    let drawing_area = SVGBackend::new(&path, (600, 400)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area);
    chart_builder
        .margin(5)
        .set_left_and_bottom_label_area_size(20);
    let mut chart_context = chart_builder
        .build_cartesian_2d(
            (2..mc.face_sizes().max().unwrap_or_default() as i32).into_segmented(),
            0..4800,
        )
        .unwrap();
    chart_context.configure_mesh().draw().unwrap();
    chart_context
        .draw_series(
            Histogram::vertical(&chart_context)
                .style(BLUE.filled())
                .margin(0)
                .data(data.map(|x| (x, 1))),
        )
        .unwrap();
}

fn main()
{
    let args = Args::parse();

    let period = args.period;
    let crit_per = args.crit_period;

    make_histogram(period, crit_per);
}
