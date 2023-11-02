#![allow(dead_code)]

pub mod abstract_cycles;
pub mod arithmetic;
pub mod combinatorics;
pub mod common;
pub mod dynatomic_cover;
pub mod global_state;
pub mod lamination;
pub mod marked_cycle_cover;
pub mod tikz;
pub mod types;

const MAX_DISPLAY_ITEMS: usize = 100;

#[cfg(test)]
mod tests
{
    use crate::abstract_cycles::AbstractPoint;
    use crate::combinatorics::{dynatomic, marked_cycle, Combinatorics};
    use crate::dynatomic_cover::DynatomicCover;
    use crate::global_state::PERIOD;
    use crate::lamination::Lamination;
    use crate::marked_cycle_cover::MarkedCycleCover;
    use crate::tikz::TikzRenderer;
    use crate::types::IntAngle;

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
        let start = 3;
        let end = 15;

        for crit_period in [1, 2] {
            for period in start..end {
                let per1 = MarkedCycleCover::new(period, 1);
                let comb = marked_cycle::Comb::new(1);
                assert_eq!(
                    per1.genus(),
                    comb.genus(period),
                    "Testing MC_{period}(Per_{crit_period})"
                );
            }

            for period in start..end {
                let per2 = MarkedCycleCover::new(period, 2);
                let comb = marked_cycle::Comb::new(2);
                assert_eq!(
                    per2.genus(),
                    comb.genus(period),
                    "Testing MC_{period}(Per_{crit_period})"
                );
            }
        }
    }

    #[test]
    fn genus_dynatomic()
    {
        let start = 3;
        let end = 15;

        for crit_period in [1, 2] {
            for period in start..end {
                let per1 = DynatomicCover::new(period, 1);
                let comb = dynatomic::Comb::new(1);
                assert_eq!(
                    per1.genus(),
                    comb.genus(period),
                    "Testing Dyn_{period}(Per_{crit_period})"
                );
            }

            for period in start..end {
                let per2 = DynatomicCover::new(period, 2);
                let comb = dynatomic::Comb::new(2);
                assert_eq!(
                    per2.genus(),
                    comb.genus(period),
                    "Testing Dyn_{period}(Per_{crit_period})"
                );
            }
        }
    }

    #[test]
    fn num_faces()
    {
        let start = 3;
        let end = 15;

        for period in start..end {
            let per1 = MarkedCycleCover::new(period, 1);
            let comb = marked_cycle::Comb::new(1);
            assert_eq!(
                per1.num_faces() as i64,
                comb.faces(period),
                "Testing per1 in period {period}"
            );
        }

        for period in start..end {
            let per2 = MarkedCycleCover::new(period, 2);
            let comb = marked_cycle::Comb::new(2);
            assert_eq!(
                per2.num_faces() as i64,
                comb.faces(period),
                "Testing per2 in period {period}"
            );
        }
    }

    #[test]
    fn max_face()
    {
        let per1 = MarkedCycleCover::new(13, 1);

        assert_eq!(per1.face_sizes().max().unwrap_or_default(), 58);

        let per2 = MarkedCycleCover::new(13, 2);
        assert_eq!(per2.face_sizes().max().unwrap_or_default(), 52);
    }

    #[test]
    fn kneading_sequence()
    {
        PERIOD.set(6);
        let point = AbstractPoint::new(IntAngle(13));
        let ks = point.kneading_sequence();
        assert_eq!(format!("{ks:6}"), "00110*");

        let (_, ks) = point.orbit_min_and_kneading_sequence();
        assert_eq!(format!("{ks:6}"), "00110*");
    }

    #[test]
    fn tikz()
    {
        let per1 = MarkedCycleCover::new(6, 1);

        let tikz = TikzRenderer::new(per1.faces).generate();
        println!("{tikz}");
    }
}
