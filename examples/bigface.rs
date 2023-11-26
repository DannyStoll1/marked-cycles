use clap::Parser;
use marked_cycles::{
    common::cells::{AugmentedVertex as Aug, Face},
    global_state::*,
    marked_cycle_cover::{MCEdge, MCFace, MCVertex},
    prelude::*,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
{
    /// Period of the critical cycle (must be 1 or 2)
    #[arg(short, long, default_value_t = 1)]
    crit_period: Period,

    /// Max period of data table
    #[arg(short, long, default_value_t = 8)]
    period: Period,
}

fn rel_shift(a: IntAngle, mut b: IntAngle) -> Period
{
    let mut res = 0;
    for _ in 0..PERIOD.get() {
        if a == b {
            return res;
        }
        b = (b * 2) % MAX_ANGLE.get();
        res += 1
    }
    panic!(
        "rel_shift was called on angles in different cycles: \
        {a:0>period$b}, {b:0>period$b}",
        period = PERIOD.get() as usize
    );
}

fn main()
{
    let args = Args::parse();

    let period = args.period;
    let crit_per = args.crit_period;

    let mc = MarkedCycleCover::new(period, crit_per);
    let max_face = mc.faces.into_iter().max_by_key(Face::len).unwrap();
    let shifts = get_shifts(&max_face, mc.edges);
    println!(
        "{}",
        shifts
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn find_real_edge(face: &MCFace, edges: &Vec<MCEdge>) -> (Aug<MCVertex>, IntAngle, usize)
{
    let mut v = face.vertices[0].clone();
    for _ in 0..2 {
        for (i, e) in edges.iter().enumerate() {
            if e.start == v.vertex {
                if e.is_real() {
                    return (v, e.wake.angle0, i);
                }
                v.vertex = e.end;
            } else if e.end == v.vertex {
                if e.is_real() {
                    return (v, e.wake.angle1, i);
                }
                v.vertex = e.start;
            }
        }
    }
    panic!("Failed to find real edge");
}

fn get_shifts(face: &MCFace, mut edges: Vec<MCEdge>) -> Vec<Period>
{
    println!("{}", face.label);

    // find last active edge
    // for (i, e) in edges.iter().enumerate() {
    //     if e.is_real() {
    //         start_idx = edges.len() - i;
    //         break
    //     }
    //     // if e.start == v.vertex {
    //     //     angle = e.wake.angle0;
    //     //     break;
    //     // } else if e.end == v.vertex {
    //     //     angle = e.wake.angle1;
    //     //     break;
    //     // }
    // }

    let (mut v, mut angle, start_idx) = find_real_edge(face, &edges);
    edges.rotate_left(start_idx);

    for e in &edges {
        println!("{e}");
    }

    let mut shifts = Vec::new();

    for _ in 0..2 {
        for e in edges.iter() {
            if e.start == v.vertex {
                let shift = rel_shift(angle, e.wake.angle0);
                println!(
                    "angle={angle:0>period$b} wangle0={:0>period$b} shift={shift}",
                    e.wake.angle0,
                    period = PERIOD.get() as usize
                );
                shifts.push(shift);
                angle = e.wake.angle1;
                v.vertex = e.end;
                for _ in 0..shift {
                    angle = angle * 2 % MAX_ANGLE.get();
                }
            } else if e.end == v.vertex {
                let shift = rel_shift(angle, e.wake.angle1);
                println!(
                    "angle={angle:0>period$b} wangle1={:0>period$b} shift={shift}",
                    e.wake.angle1,
                    period = PERIOD.get() as usize
                );
                shifts.push(PERIOD.get() - shift);
                angle = e.wake.angle0;
                for _ in 0..shift {
                    angle = angle * 2 % MAX_ANGLE.get();
                }
                v.vertex = e.start;
            }
        }
        println!("Cross R+");
    }
    shifts
    // for _ in 0..2 {
    //     for e in mc.edges.iter().rev() {
    //         if e.start == v.vertex {
    //             let shift = rel_shift(e.wake.angle0, angle);
    //             rev_shifts.push(format!("{shift}"));
    //             angle = e.wake.angle1;
    //             v.vertex = e.end;
    //             for _ in 0..shift {
    //                 angle = angle * 2 % MAX_ANGLE.get();
    //             }
    //         } else if e.end == v.vertex {
    //             let shift = rel_shift(angle, e.wake.angle1);
    //             rev_shifts.push(format!("{shift}"));
    //             angle = e.wake.angle0;
    //             for _ in 0..(PERIOD.get() - shift) {
    //                 angle = angle * 2 % MAX_ANGLE.get();
    //             }
    //             v.vertex = e.start;
    //         }
    //     }
    // }
    // println!(
    //     "{}",
    //     rev_shifts.into_iter().rev().collect::<Vec<_>>().join(" ")
    // );
}
