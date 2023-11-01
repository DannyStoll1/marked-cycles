use crate::types::{INum, Period};

pub mod dynatomic;
pub mod marked_cycle;

// TODO: add CurveParams struct

pub trait Combinatorics
{
    fn points_of_period_dividing_n(&self, n: Period) -> INum;

    fn periodic_points(&self, n: Period) -> INum;

    fn cycles(&self, n: Period) -> INum;

    fn hyp_components_dividing_n(&self, n: Period) -> INum;

    fn hyperbolic_components(&self, n: Period) -> INum;

    fn satellite_components(&self, n: Period) -> INum;

    fn primitive_components(&self, n: Period) -> INum;

    fn self_conjugate_faces(&self, n: Period) -> INum;

    fn vertices(&self, n: Period) -> INum;

    fn edges(&self, n: Period) -> INum;

    fn faces(&self, n: Period) -> INum;

    fn genus(&self, n: Period) -> INum;
}
