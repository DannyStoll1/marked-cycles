use crate::abstract_cycles::{AbstractCycle, AbstractCycleClass, AbstractPoint};
use crate::common::cells::{AugmentedVertex, HalfPlane, VertexData};
use crate::common::{cells, get_orbit};
use crate::global_state::{set_period, MAX_ANGLE, PERIOD};
use crate::lamination::Lamination;
use crate::types::{IntAngle, Period};
use std::collections::{HashMap, HashSet};

pub type MCVertex = AbstractCycle;
pub type MCEdge = cells::Edge<MCVertex>;
pub type MCFace = cells::Face<AugmentedVertex<MCVertex>, AbstractCycleClass>;

use self::cells::Wake;

#[derive(Debug, PartialEq, Eq)]
pub struct MarkedCycleCoverBuilder
{
    pub period: Period,
    pub crit_period: Period,
    adjacency_map: HashMap<AbstractCycle, Vec<(AbstractCycle, IntAngle, bool)>>,
}

impl MarkedCycleCoverBuilder
{
    #[must_use]
    pub fn new(period: Period, crit_period: Period) -> Self
    {
        Self {
            period,
            crit_period,
            adjacency_map: HashMap::new(),
        }
    }

    #[must_use]
    pub fn build(&mut self) -> MarkedCycleCover
    {
        set_period(self.period);
        let cycles = Self::cycles();
        let vertices = Self::vertices(&cycles);
        let edges = self.edges(&cycles);
        let faces = self.faces(&vertices);

        MarkedCycleCover {
            crit_period: self.crit_period,
            vertices,
            edges,
            faces,
        }
    }

    fn cycles() -> Vec<Option<AbstractCycle>>
    {
        let mut cycles = vec![
            None;
            usize::try_from(MAX_ANGLE.get())
                .expect("MAX_ANGLE appears to be negative!")
        ];
        for theta in 0..MAX_ANGLE.get().into() {
            let theta_usize = theta as usize;
            if cycles[theta_usize].is_some() {
                continue;
            }

            let orbit = get_orbit(theta.into());
            if orbit.len() == PERIOD.get() as usize {
                let cycle_rep = orbit.iter().min().expect("Orbit is empty");
                let cycle_rep = AbstractPoint::new(*cycle_rep);

                orbit
                    .iter()
                    .map(|x| usize::try_from(*x).expect("Negative value in orbit"))
                    .for_each(|x| {
                        let cycle = AbstractCycle { rep: cycle_rep };
                        cycles[x] = Some(cycle);
                    });
            }
        }
        if PERIOD.get() == 1 {
            let alpha_fp = AbstractPoint::new(IntAngle(1));
            cycles.push(Some(AbstractCycle { rep: alpha_fp }));
        }
        cycles
    }

    fn vertices(cycles: &[Option<AbstractCycle>]) -> Vec<AbstractCycle>
    {
        // Vertices, labeled by abstract point
        let mut vertices = cycles.iter().filter_map(|&v| v).collect::<Vec<_>>();
        vertices.sort_unstable_by_key(|x| x.rep);
        vertices.dedup();
        vertices
    }

    fn edges(&mut self, cycles: &[Option<AbstractCycle>]) -> Vec<MCEdge>
    {
        Lamination::new()
            .with_crit_period(self.crit_period)
            .into_arcs_of_period(PERIOD.get())
            .into_iter()
            .filter_map(|(theta0, theta1)| {
                let angle0 = MAX_ANGLE.get().scale_by_ratio(&theta0);
                let angle1 = MAX_ANGLE.get().scale_by_ratio(&theta1);

                let k0 = usize::try_from(angle0).ok()?;
                let k1 = usize::try_from(angle1).ok()?;

                let cyc0 = cycles[k0]?;
                let cyc1 = cycles[k1]?;

                if cyc0 == cyc1 {
                    return None;
                }

                let tag = angle0.max(angle1);
                self.adjacency_map.entry(cyc0).or_default().push((
                    cyc1,
                    tag,
                    angle0 + angle1 == MAX_ANGLE.get(),
                ));
                self.adjacency_map.entry(cyc1).or_default().push((
                    cyc0,
                    tag,
                    angle0 + angle1 == MAX_ANGLE.get(),
                ));

                Some(MCEdge {
                    start: cyc0,
                    end: cyc1,
                    wake: Wake { angle0, angle1 },
                })
            })
            .collect()
    }

    fn faces(&self, vertices: &[AbstractCycle]) -> Vec<MCFace>
    {
        let mut visited = HashSet::new();
        vertices
            .iter()
            .copied()
            .filter_map(|cyc| {
                if visited.contains(&cyc) {
                    return None;
                }
                Some(self.traverse_face(cyc, &mut visited))
            })
            .collect()
    }

    fn traverse_face(
        &self,
        starting_point: AbstractCycle,
        visited: &mut HashSet<AbstractCycle>,
    ) -> MCFace
    {
        // cycle that is currently marked
        let mut node: AbstractCycle = starting_point;

        // angle of the current parameter
        let mut curr_angle = IntAngle(0);

        let mut vertices: Vec<AugmentedVertex<MCVertex>> = Vec::new();

        let mut face_degree = 1;

        let mut region_0 = HalfPlane::PosReal;
        let mut region_1: HalfPlane;

        while let Some((next_node, next_angle, neg_edge)) =
            self.get_next_vertex_and_angle(node, curr_angle)
        {
            // If we are crossing the real axis
            let data = if curr_angle >= next_angle {
                if node == starting_point {
                    if neg_edge {
                        vertices.get_mut(0).map(|v| v.data = VertexData::NegEdgePos);
                    }
                    break;
                }
                visited.insert(node);
                face_degree += 1;
                region_1 = HalfPlane::from(next_angle);
                // region_1 is guaranteed to be Lower
                match (region_0, region_1, neg_edge) {
                    (HalfPlane::Lower, _, true) => VertexData::NegEdgePos,
                    (_, _, true) => VertexData::NegEdge,
                    (HalfPlane::Lower, HalfPlane::Upper, _) => VertexData::PosReal,
                    (HalfPlane::Lower, _, _) => VertexData::PosNeg,
                    _ => VertexData::NegPos,
                }
            } else {
                region_1 = HalfPlane::from(next_angle);
                match (region_0, region_1, neg_edge) {
                    (_, _, true) => VertexData::NegEdge,
                    (HalfPlane::Upper, HalfPlane::Lower, _) => VertexData::NegReal,
                    (HalfPlane::PosReal, HalfPlane::Upper, _) => VertexData::PosReal,
                    (HalfPlane::PosReal, _, _) => VertexData::PosNeg,
                    _ => VertexData::NonReal,
                }
            };

            let vertex = AugmentedVertex { vertex: node, data };

            vertices.push(vertex);
            node = next_node;

            curr_angle = next_angle;
            region_0 = region_1;
        }

        if vertices.is_empty() {
            let vertex = AugmentedVertex {
                vertex: node,
                data: VertexData::PosReal,
            };
            vertices.push(vertex);
        }

        let face_id = AbstractCycleClass::new(starting_point);

        MCFace {
            label: face_id,
            vertices,
            degree: face_degree,
        }
    }

    fn get_next_vertex_and_angle(
        &self,
        node: AbstractCycle,
        curr_angle: IntAngle,
    ) -> Option<(AbstractCycle, IntAngle, bool)>
    {
        self.adjacency_map
            .get(&node)?
            .iter()
            .min_by_key(|(_, ang, _)| (ang.0 - curr_angle.0 - 1).rem_euclid(MAX_ANGLE.get().0))
            .copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MarkedCycleCover
{
    pub crit_period: Period,
    pub vertices: Vec<AbstractCycle>,
    pub edges: Vec<MCEdge>,
    pub faces: Vec<MCFace>,
}

impl MarkedCycleCover
{
    #[must_use]
    pub fn new(period: Period, crit_period: Period) -> Self
    {
        MarkedCycleCoverBuilder::new(period, crit_period).build()
    }

    #[must_use]
    pub fn euler_characteristic(&self) -> i64
    {
        self.num_vertices() as i64 - self.num_edges() as i64 + self.num_faces() as i64
    }

    #[must_use]
    pub fn num_vertices(&self) -> usize
    {
        self.vertices.len()
    }

    #[must_use]
    pub fn num_edges(&self) -> usize
    {
        self.edges.len()
    }

    #[must_use]
    pub fn num_faces(&self) -> usize
    {
        self.faces.len()
    }

    #[must_use]
    pub fn genus(&self) -> i64
    {
        1 - self.euler_characteristic() / 2
    }

    pub fn face_sizes(&self) -> impl Iterator<Item = usize> + '_
    {
        self.faces.iter().map(MCFace::len)
    }

    pub fn face_sizes_irreflexive(&self) -> impl Iterator<Item = usize> + '_
    {
        self.faces.iter().filter(|f| f.degree > 1).map(MCFace::len)
    }

    #[must_use]
    pub fn num_odd_faces_irreflexive(&self) -> usize
    {
        self.faces
            .iter()
            .filter(|f| f.degree > 1 && f.len() % 2 == 1)
            .count()
    }

    #[must_use]
    pub fn num_odd_faces(&self) -> usize
    {
        self.face_sizes().filter(|&s| s % 2 == 1).count()
    }

    pub fn summarize(&self, indent: usize, binary: bool)
    {
        let indent_str = " ".repeat(indent);
        macro_rules! print_elements {
            ($title: expr, $iter: expr, $count: expr) => {
                if $count > crate::MAX_DISPLAY_ITEMS {
                    println!("\n{} {}", $count, $title);
                } else {
                    println!("\n{} {}:", $count, $title);
                    for elem in $iter {
                        if binary {
                            println!("{indent_str}{elem:b}",);
                        } else {
                            println!("{indent_str}{elem}");
                        }
                    }
                }
            };
        }

        print_elements!("vertices", &self.vertices, self.vertices.len());
        print_elements!("edges", &self.edges, self.edges.len());
        print_elements!("faces", &self.faces, self.faces.len());

        if self.faces.len() < crate::MAX_DISPLAY_ITEMS {
            println!("\nFace sizes:");
            println!("{}{:?}", indent_str, self.face_sizes().collect::<Vec<_>>());
        }

        println!(
            "\nSmallest face: {}",
            self.face_sizes().min().unwrap_or(usize::MAX)
        );
        println!("\nLargest face: {}", self.face_sizes().max().unwrap_or(0));
        println!("\nGenus is {}", self.genus());
    }
}
