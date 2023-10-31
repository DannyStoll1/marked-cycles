use crate::types::{INum, Period};
pub use num::integer::gcd;

pub fn divisors(n: Period) -> impl Iterator<Item = Period>
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

pub fn dirichlet_convolution<F, G>(f: F, g: G, n: Period) -> INum
where
    F: Fn(Period) -> INum,
    G: Fn(Period) -> INum,
{
    divisors(n).map(|d| f(d) * g(n / d)).sum()
}

pub fn filtered_dirichlet_convolution<F, G, H>(f: F, g: G, n: Period, filter_fn: H) -> INum
where
    F: Fn(Period) -> INum,
    G: Fn(Period) -> INum,
    H: FnMut(&Period) -> bool,
{
    divisors(n).filter(filter_fn).map(|d| f(d) * g(n / d)).sum()
}

pub fn moebius_inversion<F>(f: F, n: Period) -> INum
where
    F: Fn(Period) -> INum,
{
    dirichlet_convolution(moebius, f, n)
}
