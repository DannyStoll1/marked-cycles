use std::cmp::Ordering;

use crate::types::{Period, RatAngle};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CachedRatAngle
{
    angle: RatAngle,
    float_val: f64,
}
impl CachedRatAngle
{
    pub fn new(numer: Period, denom: Period) -> Self
    {
        let angle = RatAngle::new(numer, denom);
        let float_val = (*angle.numer() as f64) / (*angle.denom() as f64);
        Self { angle, float_val }
    }
}
impl std::cmp::PartialOrd for CachedRatAngle
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.float_val.partial_cmp(&other.float_val)
    }
}
impl From<RatAngle> for CachedRatAngle
{
    fn from(angle: RatAngle) -> Self
    {
        let float_val = (*angle.numer() as f64) / (*angle.denom() as f64);
        Self { angle, float_val }
    }
}
impl From<CachedRatAngle> for RatAngle
{
    fn from(cangle: CachedRatAngle) -> Self
    {
        cangle.angle
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CachedArc(CachedRatAngle, CachedRatAngle);
impl From<CachedArc> for (RatAngle, RatAngle)
{
    fn from(carc: CachedArc) -> Self
    {
        (carc.0.into(), carc.1.into())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Endpoint
{
    angle: CachedRatAngle,
    other: CachedRatAngle,
    left: bool,
}

impl Endpoint
{
    #[must_use]
    pub fn left(angle: CachedRatAngle, other: CachedRatAngle) -> Self
    {
        Self {
            angle,
            other,
            left: true,
        }
    }
    #[must_use]
    pub fn right(angle: CachedRatAngle, other: CachedRatAngle) -> Self
    {
        Self {
            angle,
            other,
            left: false,
        }
    }
}

impl From<Endpoint> for (RatAngle, RatAngle)
{
    fn from(endpt: Endpoint) -> Self
    {
        (endpt.angle.into(), endpt.other.into())
    }
}

impl std::cmp::PartialOrd for Endpoint
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.angle.partial_cmp(&other.angle)
    }
}

/// Implementation of Lavaurs' algorithm to compute the lamination for the combinatorial Mandelbrot
/// set.
#[derive(Clone, Debug, PartialEq)]
pub struct Lamination
{
    pub crit_period: Period,
    max_period: Period,
    arcs: Vec<Vec<(RatAngle, RatAngle)>>,
    endpoints: Vec<Endpoint>,
}

impl Lamination
{
    #[must_use]
    pub fn new() -> Self
    {
        let endpoints = vec![Endpoint::default()];

        let arcs = vec![Vec::new(), vec![(RatAngle::new(0, 1), RatAngle::new(0, 1))]];

        Self {
            crit_period: 1,
            max_period: 1,
            arcs,
            endpoints,
        }
    }

    #[must_use]
    pub fn with_crit_period(mut self, crit_period: Period) -> Self
    {
        self.crit_period = crit_period;
        self
    }

    #[must_use]
    pub fn per2(mut self) -> Self
    {
        self.crit_period = 2;
        self
    }

    fn extend(&mut self)
    {
        self.max_period += 1;
        let n = 2_i64.pow(self.max_period as u32) - 1;

        let mut stack: Vec<Period> = Vec::new();

        let mut new_endpoints = Vec::new();
        let mut endpoint_it = self.endpoints.iter().skip(1).peekable();

        'outer: for k in (1..n).filter(|k| self.crit_period == 1 || k * 3 < n || k * 3 > 2 * n)
        {
            let theta = CachedRatAngle::from(RatAngle::new(k, n));

            'inner: while let Some(&curr) = endpoint_it.peek()
            {
                match curr.angle.partial_cmp(&theta)
                {
                    Some(Ordering::Less) =>
                    {
                        if curr.left
                        {
                            stack.push(0);
                        }
                        else
                        {
                            let top = stack.pop();
                            debug_assert_eq!(top, Some(0));
                        }
                    }
                    Some(Ordering::Equal) =>
                    {
                        endpoint_it.next();
                        continue 'outer;
                    }
                    Some(Ordering::Greater) => break 'inner,
                    None => panic!(
                        "NaN encountered in comparison! curr.angle = {:?}, theta = {theta:?}",
                        curr.angle
                    ),
                }
                endpoint_it.next();
            }

            match stack.last()
            {
                Some(&j) if j != 0 =>
                {
                    let other = CachedRatAngle::new(j, n);
                    new_endpoints.push(Endpoint::left(other, theta));
                    new_endpoints.push(Endpoint::right(theta, other));
                    stack.pop();
                }
                _ =>
                {
                    stack.push(k);
                }
            }
        }

        new_endpoints.sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());

        self.endpoints = self
            .endpoints
            .iter()
            .cloned()
            .merge(new_endpoints.iter().cloned())
            .collect();

        let new_arcs = new_endpoints
            .into_iter()
            .filter(|e| e.left)
            .map(Into::into)
            .collect();

        self.arcs.push(new_arcs);
    }

    const fn len(&self) -> Period
    {
        self.max_period
    }

    pub fn extend_to_period(&mut self, period: Period)
    {
        for _ in self.max_period..(period as Period)
        {
            self.extend();
        }
    }

    #[must_use]
    pub fn arcs_of_period(&mut self, per: Period) -> &Vec<(RatAngle, RatAngle)>
    {
        self.extend_to_period(per);
        if per <= 0
        {
            return &self.arcs[0];
        }

        &self.arcs[per as usize]
    }

    #[must_use]
    pub fn into_arcs_of_period(mut self, per: Period) -> Vec<(RatAngle, RatAngle)>
    {
        self.extend_to_period(per);
        if per <= 0
        {
            return std::mem::take(&mut self.arcs[0]);
        }

        return std::mem::take(&mut self.arcs[per as usize]);
    }

    #[must_use]
    pub fn into_arcs(mut self, per: Period) -> Vec<Vec<(RatAngle, RatAngle)>>
    {
        self.extend_to_period(per);
        self.arcs
    }

    fn arc_lengths_of_period(&mut self, per: Period) -> Vec<RatAngle>
    {
        self.arcs_of_period(per)
            .iter()
            .map(|(a, b)| b - a)
            .collect()
    }
}

impl Default for Lamination
{
    fn default() -> Self
    {
        Self::new()
    }
}

fn main()
{
    let mut lamination = Lamination::new();
    let arcs = lamination.arcs_of_period(9);
    for (a, b) in arcs
    {
        println!(
            "{:>3}/{:<3} <--> {:>3}/{:<3}",
            a.numer(),
            a.denom(),
            b.numer(),
            b.denom()
        );
    }
}
