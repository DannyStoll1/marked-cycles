use std::num::TryFromIntError;

use derive_more::*;
use num_rational::Rational64;

use crate::global_state::PERIOD;

pub type Period = i64;
pub type UPeriod = u64;
pub type INum = i64;
pub type RatAngle = Rational64;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Into,
    Add,
    Sub,
    Mul,
    MulAssign,
    AddAssign,
    Div,
    RemAssign,
    BitAnd,
    Not,
    Binary,
    Display,
)]
pub struct IntAngle(pub Period);

impl IntAngle
{
    pub fn scale_by_ratio(&self, ratio: &Rational64) -> Self
    {
        let theta = (ratio * self.0).to_integer();
        Self(theta)
    }
}

impl std::ops::Shl<Period> for IntAngle
{
    type Output = Self;
    fn shl(self, rhs: Period) -> Self::Output
    {
        Self(self.0 << rhs)
    }
}

impl std::ops::Shr<Period> for IntAngle
{
    type Output = Self;
    fn shr(self, rhs: Period) -> Self::Output
    {
        Self(self.0 >> rhs)
    }
}

impl std::ops::Rem for IntAngle
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output
    {
        Self(self.0 % rhs.0)
    }
}

impl TryFrom<IntAngle> for usize
{
    type Error = TryFromIntError;
    fn try_from(value: IntAngle) -> Result<Self, Self::Error>
    {
        value.0.try_into()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KneadingSequence
{
    itinerary: i64,
}

impl KneadingSequence
{
    #[inline]
    pub fn increment(&mut self)
    {
        self.itinerary += 1;
    }

    #[inline]
    pub fn shift(&mut self)
    {
        self.itinerary <<= 1;
    }
}

impl std::fmt::Display for KneadingSequence
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "{:0>width$b}*",
            self.itinerary >> 1,
            width = (PERIOD.get() - 1) as usize
        )
    }
}
