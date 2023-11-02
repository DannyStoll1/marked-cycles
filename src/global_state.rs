use crate::types::{IntAngle, Period};
use std::cell::Cell;

thread_local! {
    pub static PERIOD: Cell<Period> = Cell::new(3);
    pub static MAX_ANGLE: Cell<IntAngle> = Cell::new(IntAngle(7));
}

pub fn set_period(period: Period)
{
    PERIOD.set(period);
    MAX_ANGLE.set(IntAngle((1 << period) - 1));
}
