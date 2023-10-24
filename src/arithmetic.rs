use crate::types::Period;
use derive_more::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub struct IntegerMod<const P: Period>
{
    pub rep: Period,
}
impl<const P: Period> IntegerMod<P>
{
    #[must_use]
    pub fn new(value: Period) -> Self
    {
        Self { rep: value % P }
    }
}

impl<const P: Period, T: Into<Period>> From<T> for IntegerMod<P>
{
    fn from(value: T) -> Self
    {
        Self::new(value.into())
    }
}

impl<const P: Period> std::ops::Add for IntegerMod<P>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        Self::new(self.rep + rhs.rep)
    }
}

impl<const P: Period> std::ops::Sub for IntegerMod<P>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output
    {
        Self::new(self.rep - rhs.rep)
    }
}

impl<const P: Period> std::ops::Neg for IntegerMod<P>
{
    type Output = Self;
    fn neg(self) -> Self::Output
    {
        Self::new(-self.rep)
    }
}

impl<const P: Period> std::ops::MulAssign for IntegerMod<P>
{
    fn mul_assign(&mut self, rhs: Self)
    {
        self.rep = (self.rep * rhs.rep) % P
    }
}

impl<const P: Period> std::ops::Mul for IntegerMod<P>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output
    {
        Self::new(self.rep * rhs.rep)
    }
}
