#![feature(test)]

extern crate test;
use test::Bencher;

use marked_cycles::{lamination::Lamination, marked_cycle_cover::MarkedCycleCover};

#[bench]
fn lamination(b: &mut Bencher)
{
    b.iter(|| {
        let _ = Lamination::new().into_arcs_of_period(16);
    });
}

#[bench]
fn mc_curve(b: &mut Bencher)
{
    b.iter(|| {
        let _curve = MarkedCycleCover::new(16, 2, 1);
    });
}
