use crate::abstract_cycles::{AbstractPoint, AbstractPointClass, ShiftedCycle};
use crate::common::{
    cells::{self, Wake},
    get_orbit,
};
use crate::global_state::{set_period, MAX_ANGLE, PERIOD};
use crate::lamination::Lamination;
use crate::types::{IntAngle, Period};
use num::Integer;
use std::collections::{HashMap, HashSet};

type Vertex = ShiftedCycle;
type Edge = cells::Edge<Vertex>;
type PrimitiveFace = cells::Face<Vertex, AbstractPointClass>;
type SatelliteFace = cells::Face<Vertex, Vertex>;

#[derive(PartialEq, Eq)]
struct EdgeRep(pub Edge);

impl EdgeRep
{
    pub fn is_satellite(&self) -> bool
    {
        self.0.start.matches(self.0.end)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DynatomicCoverBuilder
{
    pub period: Period,
    pub crit_period: Period,
    adjacency_map: HashMap<AbstractPoint, Vec<(ShiftedCycle, Period, IntAngle)>>,
}

impl DynatomicCoverBuilder
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
    pub fn build(&mut self) -> DynatomicCover
    {
        set_period(self.period);
        let cycles = self.cycles();
        let edge_reps = self.edge_reps(&cycles);
        let vertices = Self::vertices(&cycles);
        let edges = self.edges(&edge_reps);
        let primitive_faces = self.primitive_faces(&vertices);
        let satellite_faces = self.satellite_faces(&edge_reps);

        DynatomicCover {
            crit_period: self.crit_period,
            vertices,
            edges,
            primitive_faces,
            satellite_faces,
        }
    }

    #[inline]
    fn orbit(angle: IntAngle) -> Vec<IntAngle>
    {
        get_orbit(angle)
    }

    fn cycles(&self) -> Vec<Option<ShiftedCycle>>
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
            if orbit.len() == self.period as usize {
                let cycle_rep = orbit[0]; // Always the minimum in the orbit
                let cycle_rep = AbstractPoint::new(cycle_rep);

                orbit
                    .iter()
                    .map(|x| usize::try_from(*x).unwrap_or_default())
                    .enumerate()
                    .for_each(|(i, x)| {
                        let shift = i as i64;
                        let shifted_cycle = ShiftedCycle {
                            rep: cycle_rep,
                            shift,
                        };
                        cycles[x] = Some(shifted_cycle);
                    });
            }
        }
        if PERIOD.get() == 1 {
            let alpha_fp = AbstractPoint::new(IntAngle(1));
            cycles.push(Some(ShiftedCycle {
                rep: alpha_fp,
                shift: 0,
            }));
        }
        cycles
    }

    fn vertices(cycles: &[Option<ShiftedCycle>]) -> Vec<ShiftedCycle>
    {
        // Vertices, labeled by abstract point
        cycles.iter().filter_map(|&v| v).collect::<Vec<_>>()
    }

    fn edge_reps(&mut self, cycles: &[Option<ShiftedCycle>]) -> Vec<EdgeRep>
    {
        // Leaves of lamination, labeled by shifted cycle
        Lamination::new()
            .with_crit_period(self.crit_period)
            .into_arcs_of_period(self.period)
            .into_iter()
            .filter_map(|(theta0, theta1)| {
                let angle0 = MAX_ANGLE.get().scale_by_ratio(&theta0);
                let angle1 = MAX_ANGLE.get().scale_by_ratio(&theta1);

                let k0 = usize::try_from(angle0).ok()?;
                let k1 = usize::try_from(angle1).ok()?;

                let cyc0 = cycles[k0]?;
                let cyc1 = cycles[k1]?;

                let tag = angle0.max(angle1);
                self.adjacency_map
                    .entry(cyc0.rep)
                    .or_default()
                    .push((cyc1, cyc0.shift, tag));
                self.adjacency_map
                    .entry(cyc1.rep)
                    .or_default()
                    .push((cyc0, cyc1.shift, tag));

                Some(EdgeRep(Edge {
                    start: cyc0,
                    end: cyc1,
                    wake: Wake { angle0, angle1 },
                }))
            })
            .collect()
    }

    fn edges(&mut self, edge_reps: &[EdgeRep]) -> Vec<Edge>
    {
        edge_reps
            .iter()
            .flat_map(|EdgeRep(e)| {
                (0..self.period).map(|i| Edge {
                    start: e.start.rotate(i),
                    end: e.end.rotate(i),
                    wake: e.wake.clone(),
                })
            })
            .collect()
    }

    fn satellite_faces(&self, wakes: &[EdgeRep]) -> Vec<SatelliteFace>
    {
        wakes
            .iter()
            .filter(|e| e.is_satellite())
            .flat_map(|EdgeRep(e)| {
                let shift = e.end.relative_shift(e.start);
                let num_faces = shift.gcd(&self.period);
                let face_period = self.period / num_faces;
                (0..num_faces).map(move |i| {
                    let base_point = e.start.with_shift(0).rotate(i);
                    SatelliteFace {
                        label: base_point,
                        vertices: (0..face_period)
                            .map(|j| base_point.rotate(j * shift))
                            .collect(),
                        degree: 1,
                    }
                })
            })
            .collect()
    }

    fn primitive_faces(&self, vertices: &[ShiftedCycle]) -> Vec<PrimitiveFace>
    {
        let mut visited = HashSet::new();
        vertices
            .iter()
            .filter_map(|cyc| {
                if visited.contains(cyc) {
                    return None;
                }
                Some(self.traverse_face(*cyc, &mut visited))
            })
            .collect()
    }

    fn traverse_face(
        &self,
        starting_point: ShiftedCycle,
        visited: &mut HashSet<ShiftedCycle>,
    ) -> PrimitiveFace
    {
        // Cycle that is currently marked
        let mut node: ShiftedCycle = starting_point;

        // Angle of the current parameter
        let mut curr_angle = IntAngle(0);
        let mut nodes = Vec::new();
        let mut face_degree = 1;

        while let Some((next_node, next_angle)) = self.get_next_vertex_and_angle(node, curr_angle) {
            // If we are crossing the real axis
            if curr_angle >= next_angle {
                if node == starting_point {
                    break;
                }
                visited.insert(node);
                face_degree += 1;
            }

            nodes.push(node);
            node = next_node;

            curr_angle = next_angle;
        }

        if nodes.is_empty() {
            nodes.push(node);
        }

        PrimitiveFace {
            label: starting_point.to_point_class(),
            vertices: nodes,
            degree: face_degree,
        }
    }

    fn get_next_vertex_and_angle(
        &self,
        node: ShiftedCycle,
        curr_angle: IntAngle,
    ) -> Option<(ShiftedCycle, IntAngle)>
    {
        self.adjacency_map
            .get(&node.rep)?
            .iter()
            .min_by_key(|(_, _, ang)| (ang.0 - curr_angle.0 - 1).rem_euclid(MAX_ANGLE.get().0))
            .map(|(beta, alpha_shift, ang)| (beta.rotate(node.shift - alpha_shift), *ang))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DynatomicCover
{
    pub crit_period: Period,
    pub vertices: Vec<ShiftedCycle>,
    pub edges: Vec<Edge>,
    pub primitive_faces: Vec<PrimitiveFace>,
    pub satellite_faces: Vec<SatelliteFace>,
}

impl DynatomicCover
{
    #[must_use]
    pub fn new(period: Period, crit_period: Period) -> Self
    {
        DynatomicCoverBuilder::new(period, crit_period).build()
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
        self.primitive_faces.len() + self.satellite_faces.len()
    }

    #[must_use]
    pub fn genus(&self) -> i64
    {
        1 - self.euler_characteristic() / 2
    }

    #[must_use]
    pub fn face_sizes(&self) -> Vec<usize>
    {
        let primitive_sizes = self.primitive_faces.iter().map(|f| f.vertices.len());

        let satellite_sizes = self.satellite_faces.iter().map(|f| f.vertices.len());

        primitive_sizes.chain(satellite_sizes).collect()
    }

    #[must_use]
    pub fn num_odd_faces(&self) -> usize
    {
        self.face_sizes().iter().filter(|&s| s % 2 == 1).count()
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
                            println!("{indent_str}{elem:b}");
                        } else {
                            println!("{indent_str}{elem}");
                        }
                    }
                }
            };
        }

        print_elements!(
            "vertices",
            self.vertices.iter().map(|v| v.to_point()),
            self.vertices.len()
        );
        print_elements!("edges", &self.edges, self.edges.len());
        print_elements!(
            "primitive faces",
            &self.primitive_faces,
            self.primitive_faces.len()
        );
        print_elements!(
            "satellite faces",
            &self.satellite_faces,
            self.satellite_faces.len()
        );

        if self.primitive_faces.len() < crate::MAX_DISPLAY_ITEMS {
            println!("\nFace sizes:");
            println!("{}{:?}", indent_str, self.face_sizes());
        }

        println!(
            "\nSmallest face: {}",
            self.face_sizes().iter().min().unwrap_or(&usize::MAX)
        );
        println!(
            "\nLargest face: {}",
            self.face_sizes().iter().max().unwrap_or(&0)
        );
        println!("\nGenus is {}", self.genus());
    }
}
