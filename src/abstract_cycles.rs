use crate::types::{Angle, Period};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbstractPoint
{
    pub angle: Angle,
    pub period: Period,
    max_angle: Angle,
}

impl AbstractPoint
{
    #[must_use]
    pub fn new(angle: Angle, period: Period) -> Self
    {
        let max_angle = Angle((1 << period) - 1);
        Self {
            angle,
            period,
            max_angle,
        }
    }

    #[must_use]
    pub fn with_angle(&self, angle: Angle) -> Self
    {
        Self {
            angle,
            period: self.period,
            max_angle: self.max_angle,
        }
    }

    pub fn orbit_min(&self) -> Self
    {
        let mut theta = self.angle;
        let mut min_theta = theta;

        while theta != self.angle
        {
            theta = (theta * 2) % self.max_angle;
            min_theta = min_theta.min(theta);
        }
        self.with_angle(min_theta)
    }

    pub fn rotate(&self, shift: Period) -> Self
    {
        let rep = (self.angle << shift) % self.max_angle;
        self.with_angle(rep)
    }

    pub fn bit_flip(&self) -> Self
    {
        self.with_angle(self.max_angle & !self.angle)
    }
}

impl std::fmt::Display for AbstractPoint
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.angle)
    }
}

impl std::fmt::Binary for AbstractPoint
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:0n$b}", self.angle, n = self.period as usize)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AbstractPointClass
{
    pub rep: AbstractPoint,
}
impl AbstractPointClass
{
    #[must_use]
    pub fn new(point: AbstractPoint) -> Self
    {
        Self {
            rep: point.min(point.bit_flip()),
        }
    }
}
impl From<AbstractPoint> for AbstractPointClass
{
    fn from(point: AbstractPoint) -> Self
    {
        Self::new(point)
    }
}

impl std::fmt::Binary for AbstractPointClass
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "[{:0n$b}]", self.rep.angle, n = self.rep.period as usize)
    }
}

impl std::fmt::Display for AbstractPointClass
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "[{}]", self.rep.angle)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbstractCycle
{
    pub rep: AbstractPoint,
}

impl AbstractCycle
{
    #[must_use]
    pub fn new_compute(point: AbstractPoint) -> Self
    {
        Self {
            rep: point.orbit_min(),
        }
    }

    pub fn compute_cycle_class(&self) -> AbstractCycleClass {
        let dual_rep = self.rep.bit_flip().orbit_min();
        AbstractCycleClass {
            rep: self.rep.min(dual_rep),
        }
    }
}

impl std::fmt::Display for AbstractCycle
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({})", self.rep.angle)
    }
}

impl std::fmt::Binary for AbstractCycle
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({:0n$b})", self.rep.angle, n = self.rep.period as usize)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AbstractCycleClass
{
    pub rep: AbstractPoint,
}
impl AbstractCycleClass
{
    #[must_use]
    pub fn new_compute(cycle: AbstractCycle) -> Self
    {
        let dual_rep = cycle.rep.bit_flip().orbit_min();
        Self {
            rep: cycle.rep.min(dual_rep),
        }
    }
}
impl From<AbstractCycle> for AbstractCycleClass
{
    fn from(cycle: AbstractCycle) -> Self
    {
        Self::new_compute(cycle)
    }
}

impl std::fmt::Binary for AbstractCycleClass
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "<{:0n$b}>", self.rep.angle, n = self.rep.period as usize)
    }
}

impl std::fmt::Display for AbstractCycleClass
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "<{}>", self.rep.angle)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShiftedCycle
{
    pub rep: AbstractPoint,
    pub shift: Period,
}

impl ShiftedCycle
{
    #[must_use]
    pub fn with_shift(self, shift: Period) -> Self
    {
        Self {
            rep: self.rep,
            shift,
        }
    }

    pub fn matches(&self, other: ShiftedCycle) -> bool
    {
        self.rep == other.rep
    }

    // Get shift of self relative to another shifted cycle
    pub fn relative_shift(&self, other: ShiftedCycle) -> Period
    {
        (self.shift - other.shift).rem_euclid(self.rep.period)
    }

    // Return a copy of self, rotated by a given shift
    pub fn rotate(&self, shift: Period) -> Self
    {
        let new_shift = (self.shift + shift).rem_euclid(self.rep.period);
        Self {
            rep: self.rep,
            shift: new_shift,
        }
    }

    #[inline]
    pub fn to_point(&self) -> AbstractPoint
    {
        self.rep.rotate(self.shift)
    }

    #[inline]
    pub fn into_point(self) -> AbstractPoint
    {
        self.rep.rotate(self.shift)
    }

    #[inline]
    pub fn to_point_class(&self) -> AbstractPointClass
    {
        self.rep.rotate(self.shift).into()
    }

    #[inline]
    pub fn into_point_class(self) -> AbstractPointClass
    {
        self.rep.rotate(self.shift).into()
    }
}

impl From<ShiftedCycle> for AbstractPoint
{
    fn from(value: ShiftedCycle) -> Self
    {
        value.into_point()
    }
}

impl From<ShiftedCycle> for AbstractPointClass
{
    fn from(value: ShiftedCycle) -> Self
    {
        value.rep.rotate(value.shift).into()
    }
}

impl std::fmt::Binary for ShiftedCycle
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "[{:0n$b}; {}]",
            self.rep.angle,
            self.shift,
            n = self.rep.period as usize
        )
    }
}

impl std::fmt::Display for ShiftedCycle
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "[{}; {}]", self.rep.angle, self.shift)
    }
}
