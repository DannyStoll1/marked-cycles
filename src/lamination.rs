use num_bigint::BigInt;
use num_rational::Rational64;
use std::collections::HashSet;
use crate::types::Period;

pub struct Lamination {
    crit_period: Period,
    degree: Period,
    max_period: Period,
    size: Period,
    arcs: Vec<(Period, Rational64, Rational64)>,
    endpoints: HashSet<Rational64>,
    period_cutoffs: Vec<Period>,
}

impl Lamination {
    pub fn new(period: Period, degree: Period, crit_period: Period) -> Self {
        let mut lamination = Lamination {
            crit_period,
            degree,
            max_period: 1,
            size: 1,
            arcs: vec![(0, Rational64::new(0, 1), Rational64::new(0, 1))],
            endpoints: HashSet::new(),
            period_cutoffs: vec![0, 1],
        };
        lamination.endpoints.insert(Rational64::new(0, 1));
        lamination.extend_to_period(period);
        lamination
    }

    fn extend(&mut self) {
        self.max_period += 1;
        let n = self.degree.pow(self.max_period.try_into().unwrap()) - 1;
        let mut counters: Vec<BigInt> = vec![BigInt::from(0); n as usize];
        let neg_one = BigInt::from(-1);

        for k in 0..n {
            if self
                .endpoints
                .contains(&Rational64::new(k as Period, n as Period))
            {
                counters[k as usize] = neg_one.clone();
            }
        }
        if self.crit_period == 2 {
            let lo = n / 3 + 1;
            let hi = if n % 3 == 0 { 2 * n / 3 } else { 2 * n / 3 + 1 };
            for k in lo..hi {
                counters[k as usize] = neg_one.clone();
            }
        }

        for &(id, a, b) in self.arcs.iter() {
            let n_rat = Rational64::from(n as Period);
            let lo = (n_rat * a).ceil().to_integer();
            let hi = (n_rat * b).ceil().to_integer();
            let counter_modification = BigInt::from(1) << id;
            for k in lo..hi {
                if counters[k as usize] != neg_one {
                    counters[k as usize] ^= counter_modification.clone();
                }
            }
        }

        let mut angles = std::collections::HashMap::new();

        for (k, &ref counter) in counters.iter().enumerate().skip(1) {
            if *counter == neg_one {
                continue;
            }

            if let Some(&angle) = angles.get(&counter) {
                let id = self.size;
                self.arcs
                    .push((id, angle, Rational64::new(k as Period, n as Period)));
                self.size += 1;
                self.endpoints.insert(angle);
                self.endpoints.insert(Rational64::new(k as Period, n as Period));
                angles.remove(&counter);
            } else {
                angles.insert(counter, Rational64::new(k as Period, n as Period));
            }
        }

        self.period_cutoffs.push(self.size);
    }

    fn len(&self) -> Period {
        self.size
    }

    fn extend_to_period(&mut self, period: Period) {
        for _ in self.max_period..(period as Period) {
            self.extend();
        }
    }

    pub fn arcs_of_period(&self, per: Period, sort: bool) -> Vec<(Rational64, Rational64)> {
        let i = self.period_cutoffs[per as usize - 1] as usize;
        let j = self.period_cutoffs[per as usize] as usize;
        let mut out = self.arcs[i..j]
            .iter()
            .map(|(_, a, b)| (*a, *b))
            .collect::<Vec<_>>();
        if sort {
            out.sort_by(|(a, _), (b, _)| a.cmp(b));
        }
        out
    }

    fn arc_lengths_of_period(&self, per: Period) -> Vec<Rational64> {
        let i = self.period_cutoffs[per as usize - 1] as usize;
        let j = self.period_cutoffs[per as usize] as usize;
        self.arcs[i..j]
            .iter()
            .map(|(_, a, b)| b - a)
            .collect::<Vec<_>>()
    }

    fn arc_lengths_cumulative(&self, max_per: Period) -> Vec<Rational64> {
        let j = if max_per == -1 {
            self.arcs.len()
        } else {
            self.period_cutoffs[max_per as usize] as usize
        };
        self.arcs[..j]
            .iter()
            .map(|(_, a, b)| b - a)
            .collect::<Vec<_>>()
    }

    fn arc_lengths_cumulative_set(&self, max_per: Period) -> HashSet<Rational64> {
        let j = if max_per == -1 {
            self.arcs.len()
        } else {
            self.period_cutoffs[max_per as usize] as usize
        };
        self.arcs[..j]
            .iter()
            .map(|(_, a, b)| b - a)
            .collect::<HashSet<_>>()
    }
}

fn main() {
    let lamination = Lamination::new(10, 2, 1);
    let arcs = lamination.arcs_of_period(9, true);
    for (a, b) in arcs {
        println!(
            "{:>3}/{:<3} <--> {:>3}/{:<3}",
            a.numer(),
            a.denom(),
            b.numer(),
            b.denom()
        );
    }
}
