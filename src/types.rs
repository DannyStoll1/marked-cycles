pub type Period = i64;
pub type UPeriod = u64;
pub type INum = i64;

use std::num::TryFromIntError;

use derive_more::*;
use num_rational::Rational64;

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
    Display
)]
pub struct Angle(pub Period);

impl Angle {
    fn scale_by_ratio(&self, ratio: Rational64) -> Self {
        let theta = (ratio * (self.0 as i64)).to_integer();
        Angle(theta)
    }
}

impl std::ops::Shl<Period> for Angle
{
    type Output = Angle;
    fn shl(self, rhs: Period) -> Self::Output
    {
        Angle(self.0 << rhs)
    }
}

impl std::ops::Shr<Period> for Angle
{
    type Output = Angle;
    fn shr(self, rhs: Period) -> Self::Output
    {
        Angle(self.0 >> rhs)
    }
}

impl std::ops::Rem for Angle
{
    type Output = Angle;
    fn rem(self, rhs: Angle) -> Self::Output
    {
        Angle(self.0 % rhs.0)
    }
}

impl TryFrom<Angle> for usize {
    type Error = TryFromIntError;
    fn try_from(value: Angle) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}
