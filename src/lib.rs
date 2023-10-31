#![allow(dead_code)]

pub mod abstract_cycles;
pub mod arithmetic;
pub mod combinatorics;
pub mod dynatomic_cover;
pub mod lamination;
pub mod marked_cycle_cover;
pub mod types;

#[cfg(test)]
mod tests
{
    use crate::lamination::Lamination;
    use crate::marked_cycle_cover::MarkedCycleCover;

    #[test]
    fn lamination()
    {
        let arcs = Lamination::new().into_arcs_of_period(8);
        assert_eq!(*arcs[68].0.numer(), 142);

        let arcs = Lamination::new().with_crit_period(2).into_arcs_of_period(8);
        assert_eq!(*arcs[48].0.numer(), 188);
    }

    #[test]
    fn genus()
    {
        let per1 = MarkedCycleCover::new(14, 1);

        assert_eq!(per1.genus(), 3154);

        let per2 = MarkedCycleCover::new(14, 2);
        assert_eq!(per2.genus(), 1912);
    }

    #[test]
    fn max_face()
    {
        let per1 = MarkedCycleCover::new(13, 1);

        assert_eq!(per1.face_sizes().max().unwrap_or_default(), 58);

        let per2 = MarkedCycleCover::new(13, 2);
        assert_eq!(per2.face_sizes().max().unwrap_or_default(), 52);
    }
}
