use crate::abstract_cycles::{AbstractPoint, AbstractPointClass, ShiftedCycle};
use crate::lamination::Lamination;
use crate::types::{IntAngle, Period};
use std::collections::{HashMap, HashSet};

mod cells;
use cells::{Edge, PrimitiveFace, SatelliteFace, Wake};
use num::Integer;

fn get_orbit(angle: IntAngle, max_angle: IntAngle, period: Period) -> Vec<IntAngle>
{
    let mut orbit = Vec::with_capacity(period as usize);

    orbit.push(angle);
    let mut theta = angle * 2 % max_angle;

    while theta != angle {
        orbit.push(theta);
        theta = theta * 2 % max_angle;
    }

    orbit
}

#[derive(Debug, PartialEq)]
pub struct DynatomicCoverBuilder
{
    pub period: Period,
    pub crit_period: Period,
    max_angle: IntAngle,
    adjacency_map: HashMap<AbstractPoint, Vec<(ShiftedCycle, Period, IntAngle)>>,
}

impl DynatomicCoverBuilder
{
    #[must_use]
    pub fn new(period: Period, crit_period: Period) -> Self
    {
        let max_angle = IntAngle(2_i64.pow(period.try_into().unwrap()) - 1);

        Self {
            period,
            crit_period,
            max_angle,
            adjacency_map: HashMap::new(),
        }
    }

    #[must_use]
    pub fn build(&mut self) -> DynatomicCover
    {
        let cycles = self.cycles();
        let wakes = self.wakes(&cycles);
        let vertices = self.vertices(&cycles);
        let edges = self.edges(&wakes);
        let primitive_faces = self.primitive_faces(&vertices);
        let satellite_faces = self.satellite_faces(&wakes);

        DynatomicCover {
            period: self.period,
            crit_period: self.crit_period,
            vertices,
            edges,
            primitive_faces,
            satellite_faces,
        }
    }

    #[inline]
    fn orbit(&self, angle: IntAngle) -> Vec<IntAngle>
    {
        get_orbit(angle, self.max_angle, self.period)
    }

    fn cycles(&self) -> Vec<Option<ShiftedCycle>>
    {
        let mut cycles = vec![None; usize::try_from(self.max_angle).unwrap()];
        for theta in 0..self.max_angle.into() {
            let theta_usize = usize::try_from(theta).unwrap();
            if cycles[theta_usize].is_some() {
                continue;
            }

            let orbit = self.orbit(theta.into());
            if orbit.len() == self.period as usize {
                let (rep_idx, cycle_rep) = orbit.iter().enumerate().min_by_key(|x| x.1).unwrap();
                let cycle_rep = AbstractPoint::new(*cycle_rep, self.period);

                orbit
                    .iter()
                    .map(|x| usize::try_from(*x).unwrap_or_default())
                    .enumerate()
                    .for_each(|(i, x)| {
                        let shift = ((i as i64) - (rep_idx as i64)).rem_euclid(self.period);
                        let shifted_cycle = ShiftedCycle {
                            rep: cycle_rep,
                            shift,
                        };
                        cycles[x] = Some(shifted_cycle);
                    });
            }
        }
        if self.period == 1 {
            let alpha_fp = AbstractPoint::new(IntAngle(1), 1);
            cycles.push(Some(ShiftedCycle {
                rep: alpha_fp,
                shift: 0,
            }));
        }
        cycles
    }

    fn vertices(&self, cycles: &[Option<ShiftedCycle>]) -> Vec<ShiftedCycle>
    {
        // Vertices, labeled by abstract point
        cycles.iter().filter_map(|&v| v).collect::<Vec<_>>()
    }

    fn wakes(&mut self, cycles: &[Option<ShiftedCycle>]) -> Vec<Wake>
    {
        // Leaves of lamination, labeled by shifted cycle
        Lamination::new()
            .with_crit_period(self.crit_period)
            .into_arcs_of_period(self.period)
            .into_iter()
            .filter_map(|(theta0, theta1)| {
                let angle0 = self.max_angle.scale_by_ratio(&theta0);
                let angle1 = self.max_angle.scale_by_ratio(&theta1);

                let k0 = usize::try_from(angle0).ok()?;
                let k1 = usize::try_from(angle1).ok()?;

                let cyc0 = cycles[k0]?;
                let cyc1 = cycles[k1]?;

                let tag = angle0.max(angle1);
                self.adjacency_map
                    .entry(cyc0.rep)
                    .or_insert_with(Vec::new)
                    .push((cyc1, cyc0.shift, tag));
                self.adjacency_map
                    .entry(cyc1.rep)
                    .or_insert_with(Vec::new)
                    .push((cyc0, cyc1.shift, tag));

                Some(Wake {
                    theta0: cyc0,
                    theta1: cyc1,
                })
            })
            .collect()
    }

    fn edges(&mut self, wakes: &[Wake]) -> Vec<Edge>
    {
        wakes
            .iter()
            .flat_map(|w| {
                (0..self.period).map(|i| Edge {
                    start: w.theta0.rotate(i),
                    end: w.theta1.rotate(i),
                })
            })
            .collect()
    }

    fn satellite_faces(&self, wakes: &[Wake]) -> Vec<SatelliteFace>
    {
        wakes
            .iter()
            .filter(|e| e.is_satellite())
            .flat_map(|e| {
                let shift = e.theta1.relative_shift(e.theta0);
                let num_faces = shift.gcd(&self.period);
                let face_period = self.period / num_faces;
                (0..num_faces).map(move |i| {
                    let base_point = e.theta0.with_shift(0).rotate(i);
                    SatelliteFace {
                        label: base_point,
                        vertices: (0..face_period)
                            .map(|j| base_point.rotate(j * num_faces))
                            .collect(),
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
                if node.rep.angle == starting_point.rep.angle {
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

        return PrimitiveFace {
            label: starting_point.to_point_class(),
            vertices: nodes,
            degree: face_degree,
        };
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
            .min_by_key(|(_, _, ang)| (ang.0 - curr_angle.0 - 1).rem_euclid(self.max_angle.0))
            .map(|(beta, alpha_shift, ang)| (beta.rotate(node.shift - alpha_shift), *ang))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DynatomicCover
{
    pub period: Period,
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
    pub fn euler_characteristic(&self) -> isize
    {
        self.num_vertices() as isize - self.num_edges() as isize + self.num_faces() as isize
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
    pub fn genus(&self) -> isize
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
            self.face_sizes().iter().min().unwrap()
        );
        println!(
            "\nLargest face: {}",
            self.face_sizes().iter().max().unwrap()
        );
        println!("\nGenus is {}", self.genus());
    }
}
