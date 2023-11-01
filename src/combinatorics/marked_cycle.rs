use std::collections::HashMap;

use super::Combinatorics;
use crate::arithmetic::*;
use crate::marked_cycle_cover::MarkedCycleCover;
use crate::types::{INum, Period};
use num::pow;

pub struct Comb
{
    crit_period: Period,
    curves: HashMap<Period, MarkedCycleCover>,
}

impl Comb
{
    #[must_use]
    pub fn new(crit_period: Period) -> Self
    {
        let curves = HashMap::new();

        Self {
            crit_period,
            curves,
        }
    }

    pub fn curve(&mut self, n: Period) -> &mut MarkedCycleCover
    {
        let crit_per = self.crit_period;
        self.curves
            .entry(n)
            .or_insert_with(|| MarkedCycleCover::new(n, crit_per))
    }

    pub fn cover_vertices(&mut self, n: Period) -> usize
    {
        let curve = self.curve(n);
        curve.num_vertices()
    }

    pub fn cover_edges(&mut self, n: Period) -> usize
    {
        let curve = self.curve(n);
        curve.num_edges()
    }

    pub fn cover_faces(&mut self, n: Period) -> usize
    {
        let curve = self.curve(n);
        curve.num_faces()
    }

    pub fn cover_genus(&mut self, n: Period) -> i64
    {
        let curve = self.curve(n);
        curve.genus()
    }
}
impl Combinatorics for Comb
{
    #[must_use]
    fn points_of_period_dividing_n(&self, n: Period) -> INum
    {
        // Number of points of period dividing n
        // under z -> z^(+/- 2)
        let v = n.try_into().unwrap_or(0);
        match self.crit_period {
            1 => pow(2, v) - 1,
            2 => pow(2, v) - pow(-1, v),
            _ => 0,
        }
    }

    #[must_use]
    fn periodic_points(&self, n: Period) -> INum
    {
        // Number of n-periodic points for z -> z^(+/- 2)
        moebius_inversion(|d| self.points_of_period_dividing_n(d), n)
    }

    #[must_use]
    fn cycles(&self, n: Period) -> INum
    {
        // Number of n-cycles of z -> z^(+/- 2)
        self.periodic_points(n) / (n as INum)
    }

    #[must_use]
    fn hyp_components_dividing_n(&self, n: Period) -> INum
    {
        // Number of mateable hyperbolic components of period dividing n
        let v = n.try_into().unwrap_or(0);
        match self.crit_period {
            1 => pow(2, v) / 2,
            2 => (pow(2, v) - pow(-1, v)) / 3,
            _ => 0,
        }
    }

    #[must_use]
    fn hyperbolic_components(&self, n: Period) -> INum
    {
        // Number of mateable hyperbolic components of period n
        moebius_inversion(|d| self.hyp_components_dividing_n(d), n)
    }

    fn satellite_components(&self, n: Period) -> INum
    {
        // Number of mateable satellite hyperbolic components of period n
        dirichlet_convolution(euler_totient, |d| self.hyperbolic_components(d), n)
            - self.hyperbolic_components(n)
    }

    fn primitive_components(&self, n: Period) -> INum
    {
        // Number of mateable primitive hyperbolic components of period n
        2 * self.hyperbolic_components(n)
            - dirichlet_convolution(euler_totient, |d| self.hyperbolic_components(d), n)
    }

    fn self_conjugate_faces(&self, n: Period) -> INum
    {
        let symmetry_order = self.crit_period + 1;

        if n % symmetry_order > 0 {
            return 0;
        }

        let k = n / symmetry_order;

        let u: INum = 1 - self.crit_period;

        self.crit_period
            * filtered_dirichlet_convolution(
                moebius,
                |d| {
                    let v = d.try_into().unwrap_or(0);
                    pow(2, v) - pow(u, v)
                },
                k,
                |d| d % symmetry_order > 0,
            )
            / n
    }

    #[must_use]
    fn vertices(&self, n: Period) -> INum
    {
        self.cycles(n)
    }

    #[must_use]
    fn edges(&self, n: Period) -> INum
    {
        self.primitive_components(n)
    }

    #[must_use]
    fn faces(&self, n: Period) -> INum
    {
        let cper = self.crit_period;
        let cyc = self.cycles(n);
        let selfconj = self.self_conjugate_faces(n);
        (cyc + cper * selfconj) / (cper + 1)
    }

    #[must_use]
    fn genus(&self, n: Period) -> INum
    {
        let prim = self.primitive_components(n);
        let cyc = self.cycles(n);
        let selfconj = self.self_conjugate_faces(n);
        match self.crit_period {
            1 => 1 + (2 * prim - 3 * cyc - selfconj) / 4,
            2 => 1 + (3 * prim - 4 * cyc - 2 * selfconj) / 6,
            _ => 0,
        }
    }
}
