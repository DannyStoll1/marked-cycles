use crate::abstract_cycles::{AbstractCycle, AbstractCycleClass, AbstractPoint};
use crate::lamination::Lamination;
use crate::types::{IntAngle, Period};
use std::collections::{HashMap, HashSet};

mod cells;
use cells::{Edge, Face};

use self::cells::Wake;

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
pub struct MarkedCycleCoverBuilder
{
    pub period: Period,
    pub crit_period: Period,
    max_angle: IntAngle,
    adjacency_map: HashMap<AbstractCycle, Vec<(AbstractCycle, IntAngle)>>,
}

impl MarkedCycleCoverBuilder
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
    pub fn build(&mut self) -> MarkedCycleCover
    {
        let cycles = self.cycles();
        let vertices = self.vertices(&cycles);
        let edges = self.edges(&cycles);
        let faces = self.faces(&cycles, &vertices);

        MarkedCycleCover {
            period: self.period,
            crit_period: self.crit_period,
            vertices,
            edges,
            faces,
        }
    }

    #[inline]
    fn orbit(&self, angle: IntAngle) -> Vec<IntAngle>
    {
        get_orbit(angle, self.max_angle, self.period)
    }

    fn cycles(&self) -> Vec<Option<AbstractCycle>>
    {
        let mut cycles = vec![None; usize::try_from(self.max_angle).unwrap()];
        for theta in 0..self.max_angle.into() {
            let theta_usize = usize::try_from(theta).unwrap();
            if cycles[theta_usize].is_some() {
                continue;
            }

            let orbit = self.orbit(theta.into());
            if orbit.len() == self.period as usize {
                let cycle_rep = orbit.iter().min().unwrap();
                let cycle_rep = AbstractPoint::new(*cycle_rep, self.period);

                orbit
                    .iter()
                    .map(|x| usize::try_from(*x).unwrap())
                    .for_each(|x| {
                        let cycle = AbstractCycle { rep: cycle_rep };
                        cycles[x] = Some(cycle);
                    });
            }
        }
        if self.period == 1 {
            let alpha_fp = AbstractPoint::new(IntAngle(1), 1);
            cycles.push(Some(AbstractCycle { rep: alpha_fp }));
        }
        cycles
    }

    fn vertices(&self, cycles: &[Option<AbstractCycle>]) -> Vec<AbstractCycle>
    {
        // Vertices, labeled by abstract point
        let mut vertices = cycles.iter().filter_map(|&v| v).collect::<Vec<_>>();
        vertices.sort_unstable_by_key(|x| x.rep);
        vertices.dedup();
        vertices
    }

    fn edges(&mut self, cycles: &[Option<AbstractCycle>]) -> Vec<Edge>
    {
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

                if cyc0 == cyc1 {
                    return None;
                }

                let tag = angle0.max(angle1);
                self.adjacency_map
                    .entry(cyc0)
                    .or_insert_with(Vec::new)
                    .push((cyc1, tag));
                self.adjacency_map
                    .entry(cyc1)
                    .or_insert_with(Vec::new)
                    .push((cyc0, tag));

                Some(Edge {
                    start: cyc0,
                    end: cyc1,
                    wake: Wake { angle0, angle1 },
                })
            })
            .collect()
    }

    fn faces(&self, cycles: &[Option<AbstractCycle>], vertices: &[AbstractCycle]) -> Vec<Face>
    {
        let mut visited = HashSet::new();
        vertices
            .iter()
            .cloned()
            .filter_map(|cyc| {
                if visited.contains(&cyc) {
                    return None;
                }
                let k = usize::try_from(cyc.rep.bit_flip().angle).ok()?;
                let dual = cycles.get(k).cloned().flatten()?;
                let face_id = AbstractCycleClass::new_raw(cyc.rep.min(dual.rep));
                Some(self.traverse_face(face_id, &mut visited))
            })
            .collect()
    }

    fn traverse_face(
        &self,
        face_id: AbstractCycleClass,
        visited: &mut HashSet<AbstractCycle>,
    ) -> Face
    {
        let starting_point = face_id.into();

        // cycle that is currently marked
        let mut node: AbstractCycle = starting_point;

        // angle of the current parameter
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

        return Face {
            label: face_id,
            vertices: nodes,
            degree: face_degree,
        };
    }

    fn get_next_vertex_and_angle(
        &self,
        node: AbstractCycle,
        curr_angle: IntAngle,
    ) -> Option<(AbstractCycle, IntAngle)>
    {
        self.adjacency_map
            .get(&node)?
            .iter()
            .min_by_key(|(_, ang)| (ang.0 - curr_angle.0 - 1).rem_euclid(self.max_angle.0))
            .cloned()
    }
}

#[derive(Debug, PartialEq)]
pub struct MarkedCycleCover
{
    pub period: Period,
    pub crit_period: Period,
    pub vertices: Vec<AbstractCycle>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
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
        self.faces.iter().map(Face::len)
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
                            println!("{indent_str}{elem:b}");
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

        println!("\nSmallest face: {}", self.face_sizes().min().unwrap());
        println!("\nLargest face: {}", self.face_sizes().max().unwrap());
        println!("\nGenus is {}", self.genus());
    }
}

// impl From<Lamination> for MarkedCycleCover {
//     fn from(value: Lamination) -> Self {
//         let max_angle = Angle(degree.pow(period.try_into().unwrap()) - 1);
//
//         let ray_sets = Vec::new();
//
//         let cycles_with_shifts = vec![None; max_angle.try_into().unwrap()];
//         let point_classes = vec![None; max_angle.try_into().unwrap()];
//
//         let mut curve = Self {
//             period,
//             degree,
//             crit_period,
//             max_angle,
//             ray_sets,
//             cycles: cycles_with_shifts,
//             cycle_classes: point_classes,
//             vertices: Vec::new(),
//             wakes: Vec::new(),
//             edges: Vec::new(),
//             faces: Vec::new(),
//             visited_face_ids: HashSet::new(),
//         };
//         curve.run();
//         curve
//     }
// }
