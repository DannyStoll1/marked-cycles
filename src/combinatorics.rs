use crate::marked_cycle_cover::MarkedCycleCover;
use crate::types::{INum, Period};
use num::integer::gcd;
use num::pow;
use std::collections::HashMap;

// TODO: add CurveParams struct

fn divisors(n: Period) -> impl Iterator<Item = Period>
{
    (1..).take_while(move |&x| x * x <= n).flat_map(move |x| {
        if n % x == 0
        {
            if x * x == n
            {
                vec![x].into_iter()
            }
            else
            {
                vec![x, n / x].into_iter()
            }
        }
        else
        {
            vec![].into_iter()
        }
    })
}

#[must_use]
pub fn euler_totient(n: Period) -> INum
{
    (1..=n).filter(|&x| gcd(x, n) == 1).count() as INum
}

#[must_use]
pub const fn moebius(n: Period) -> INum
{
    if n == 1
    {
        return 1;
    }
    let mut result = 1;
    let mut n = n;
    let mut i = 2;
    while i * i <= n
    {
        if n % i == 0
        {
            result = -result;
            n /= i;
            if n % i == 0
            {
                return 0;
            }
        }
        i += 1;
    }
    if n > 1
    {
        result = -result;
    }
    result
}

fn dirichlet_convolution<F, G>(f: F, g: G, n: Period) -> INum
where
    F: Fn(Period) -> INum,
    G: Fn(Period) -> INum,
{
    divisors(n).map(|d| f(d) * g(n / d)).sum()
}

fn filtered_dirichlet_convolution<F, G, H>(f: F, g: G, n: Period, filter_fn: H) -> INum
where
    F: Fn(Period) -> INum,
    G: Fn(Period) -> INum,
    H: FnMut(&Period) -> bool,
{
    divisors(n).filter(filter_fn).map(|d| f(d) * g(n / d)).sum()
}

fn moebius_inversion<F>(f: F, n: Period) -> INum
where
    F: Fn(Period) -> INum,
{
    dirichlet_convolution(moebius, f, n)
}

pub struct MarkedCycleCombinatorics
{
    crit_period: Period,
    curves: HashMap<Period, MarkedCycleCover>,
}

impl MarkedCycleCombinatorics
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

    // pub fn _compute_curve(&self, n: Period) -> MarkedMultCover {
    //     let curve = MarkedMultCover::new(
    //         n.try_into().unwrap(),
    //         2,
    //         self.crit_period.try_into().unwrap(),
    //     );
    //     curve
    // }

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

    pub fn cover_genus(&mut self, n: Period) -> isize
    {
        let curve = self.curve(n);
        curve.genus()
    }

    #[must_use]
    pub fn points_of_period_dividing_n(&self, n: Period) -> INum
    {
        // Number of points of period dividing n
        // under z -> z^(+/- 2)
        let v = n.try_into().unwrap_or(0);
        match self.crit_period
        {
            1 => pow(2, v) - 1,
            2 => pow(2, v) - pow(-1, v),
            _ => 0,
        }
    }

    #[must_use]
    pub fn periodic_points(&self, n: Period) -> INum
    {
        // Number of n-periodic points for z -> z^(+/- 2)
        moebius_inversion(|d| self.points_of_period_dividing_n(d), n)
    }

    #[must_use]
    pub fn cycles(&self, n: Period) -> INum
    {
        // Number of n-cycles of z -> z^(+/- 2)
        self.periodic_points(n) / (n as INum)
    }

    #[must_use]
    pub fn hyp_components_dividing_n(&self, n: Period) -> INum
    {
        // Number of mateable hyperbolic components of period dividing n
        let v = n.try_into().unwrap_or(0);
        match self.crit_period
        {
            1 => pow(2, v) / 2,
            2 => (pow(2, v) - pow(-1, v)) / 3,
            _ => 0,
        }
    }

    #[must_use]
    pub fn hyperbolic_components(&self, n: Period) -> INum
    {
        // Number of mateable hyperbolic components of period n
        moebius_inversion(|d| self.hyp_components_dividing_n(d), n)
    }

    pub fn satellite_components(&self, n: Period) -> INum
    {
        // Number of mateable satellite hyperbolic components of period n
        dirichlet_convolution(euler_totient, |d| self.hyperbolic_components(d), n)
            - self.hyperbolic_components(n)
    }

    pub fn primitive_components(&self, n: Period) -> INum
    {
        // Number of mateable primitive hyperbolic components of period n
        2 * self.hyperbolic_components(n)
            - dirichlet_convolution(euler_totient, |d| self.hyperbolic_components(d), n)
    }

    pub fn self_conjugate_faces(&self, n: Period) -> INum
    {
        let symmetry_order = self.crit_period + 1;

        if n % symmetry_order > 0
        {
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
    pub fn vertices(&self, n: Period) -> INum
    {
        self.cycles(n)
    }

    #[must_use]
    pub fn edges(&self, n: Period) -> INum
    {
        self.primitive_components(n)
    }

    #[must_use]
    pub fn faces(&self, n: Period) -> INum
    {
        let cper = self.crit_period;
        let cyc = self.cycles(n);
        let selfconj = self.self_conjugate_faces(n);
        (cyc + cper * selfconj) / (cper + 1)
    }

    #[must_use]
    pub fn genus(&self, n: Period) -> INum
    {
        let prim = self.primitive_components(n);
        let cyc = self.cycles(n);
        let selfconj = self.self_conjugate_faces(n);
        match self.crit_period
        {
            1 => 1 + (2 * prim - 3 * cyc - selfconj) / 4,
            2 => 1 + (3 * prim - 4 * cyc - 2 * selfconj) / 6,
            _ => 0,
        }
    }
}
